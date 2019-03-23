mod error;

use error::Error;
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Session {
    client: reqwest::Client,
    url_base: url::Url,
}

const TESTCASE_NUM_MAX: usize = 8;

#[derive(Clone, Debug, PartialEq)]
pub struct Testcase {
    pub input: String,
    pub output: String,
}

impl Session {
    pub fn new() -> Result<Self> {
        let url_base =
            url::Url::parse_with_params("https://atcoder.jp/contests/", &[("lang", "en")])?;

        Ok(Self {
            client: reqwest::Client::new(),
            url_base,
        })
    }

    /// ```rust
    /// # use testcase_scraping::Result;
    /// # fn main() -> Result<()> {
    /// use testcase_scraping::{Session, Testcase};
    ///
    /// let session = Session::new()?;
    /// let testcases = session.fetch_testcases("abc118", ["a", "b", "c", "d"].iter().cloned())?;
    ///
    /// assert_eq!(
    ///     testcases["a"],
    ///     vec![
    ///         Some(Testcase {
    ///             input: "4 12\n".to_owned(),
    ///             output: "16\n".to_owned(),
    ///         }),
    ///         Some(Testcase {
    ///             input: "8 20\n".to_owned(),
    ///             output: "12\n".to_owned(),
    ///         }),
    ///         Some(Testcase {
    ///             input: "1 1\n".to_owned(),
    ///             output: "2\n".to_owned(),
    ///         }),
    ///     ],
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn fetch_testcases<'a>(
        &'a self,
        contest: &'a str,
        tasks: impl Iterator<Item = &'a str>,
    ) -> Result<HashMap<&'a str, Vec<Option<Testcase>>>> {
        log::info!("Fetch testcases for contest `{}`", contest);

        // Ensure that `contest` has the trailing slash.
        let url = if contest.ends_with('/') {
            self.url_base.join(contest)?.join("tasks/")?
        } else {
            self.url_base
                .join(&format!("{}/", contest))?
                .join("tasks/")?
        };

        let mut testcases = HashMap::new();
        for task in tasks {
            let t = format!("{}_{}", contest, task);
            let mut tc = self.fetch_testcases_task(&url, &t)?.collect::<Vec<_>>();

            // Drop Nones at the tail.
            let mut len = tc.len();
            while len > 0 && tc[len - 1].is_none() {
                len -= 1;
            }
            tc.truncate(len);

            testcases.insert(task, tc);
        }

        Ok(testcases)
    }

    fn fetch_testcases_task(
        &self,
        url: &url::Url,
        task: &str,
    ) -> Result<impl Iterator<Item = Option<Testcase>>> {
        use reqwest::StatusCode;

        let url = url.join(task)?;
        log::info!("Fetch testcases for task `{}` from {}", task, url);

        let mut resp = self.client.get(url).send()?;
        match resp.status() {
            StatusCode::OK => {
                let text = resp.text()?;
                scrape(&text)
            }
            e => Err(e)?,
        }
    }
}

/// Write the given testcases to files, named like `./testcases/task_name/testcase_1.in` for input,
/// and `./testcases/task_name/testcase_1.out` for output.
pub fn write<'a, S>(testcases: &HashMap<&'a str, Vec<Option<Testcase>>, S>) -> Result<()>
where
    S: std::hash::BuildHasher,
{
    use std::{
        fs::{self, File},
        io::Write,
        path::Path,
    };

    static TESTCASES_DIR: &str = "testcases";

    log::info!("Write testcases");

    if let Err(e) = fs::create_dir(TESTCASES_DIR) {
        log::warn!("{}", e);
    }

    for (task, testcases) in testcases {
        let path = Path::new(TESTCASES_DIR).join(task);
        log::info!("Write testcases for task `{}` to {:?}", task, path);

        let path = path.as_path();

        if let Err(e) = fs::create_dir(path) {
            log::warn!("Task `{}`: {}", task, e);
            continue;
        }

        for (i, tc) in testcases.iter().enumerate() {
            if let Some(tc) = tc {
                // Input
                let path_input = path.join(&format!("testcase_{}.in", i));
                let mut file = File::create(path_input)?;
                file.write_all(tc.input.as_bytes())?;

                // Output
                let path_output = path.join(&format!("testcase_{}.out", i));
                let mut file = File::create(path_output)?;
                file.write_all(tc.output.as_bytes())?;
            }
        }
    }

    Ok(())
}

fn scrape(text: &str) -> Result<impl Iterator<Item = Option<Testcase>>> {
    use lazy_static::lazy_static;
    use regex::Regex;
    use scraper::{Html, Selector};

    #[derive(Clone, Debug, Default)]
    struct TestcaseTemp {
        input: Option<String>,
        output: Option<String>,
    }

    let mut testcases = vec![TestcaseTemp::default(); TESTCASE_NUM_MAX];

    let doc = Html::parse_document(&text);

    let sel_part =
        Selector::parse(r#"div[class = "part"]"#).expect("Unexpectedly invalid CSS selector.");
    let sel_section = Selector::parse("section").expect("Unexpectedly invalid CSS selector.");

    for section in doc
        .select(&sel_part)
        .map(|part| part.select(&sel_section))
        .flatten()
    {
        let sel_h3 = Selector::parse("h3").unwrap();
        let sel_pre = Selector::parse("pre").unwrap();

        lazy_static! {
            static ref RE_INPUT: Regex = Regex::new(r"Sample Input (\d)").unwrap();
            static ref RE_OUTPUT: Regex = Regex::new(r"Sample Output (\d)").unwrap();
        }

        if let Some(h3) = section.select(&sel_h3).next() {
            let h3_text = h3.text().collect::<String>();

            macro_rules! parse {
                ($field: ident, $digits: ident) => {{
                    let number = $digits[1].parse::<usize>().expect("Unexpected non-digit."); // 1-based
                    if number >= TESTCASE_NUM_MAX {
                        Err(Error::Testcase("Too many testcases."))?;
                    }

                    if let Some(pre) = section.select(&sel_pre).next() {
                        if testcases[number - 1] // 0-based
                            .$field
                            .replace(pre.text().collect::<String>()).is_some()
                        {
                            Err(Error::Testcase("Duplicate testcases."))?;
                        }
                    }
                }};
            }

            if let Some(digits) = RE_INPUT.captures_iter(&h3_text).next() {
                parse!(input, digits);
            }
            if let Some(digits) = RE_OUTPUT.captures_iter(&h3_text).next() {
                parse!(output, digits);
            }
        }
    }

    let testcases = testcases.into_iter().map(|tc| match (tc.input, tc.output) {
        (Some(input), Some(output)) => Some(Testcase { input, output }),
        _ => None,
    });

    Ok(testcases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrape() {
        let text = include_str!("../tests/doc.html");
        let testcases = scrape(&text)
            .expect("Failed to scrape.")
            .collect::<Vec<_>>();

        assert_eq!(testcases.len(), TESTCASE_NUM_MAX);

        assert_eq!(
            testcases[0],
            Some(Testcase {
                input: "4 12\n".to_owned(),
                output: "16\n".to_owned(),
            })
        );

        assert_eq!(
            testcases[1],
            Some(Testcase {
                input: "8 20\n".to_owned(),
                output: "12\n".to_owned(),
            })
        );

        assert_eq!(
            testcases[2],
            Some(Testcase {
                input: "1 1\n".to_owned(),
                output: "2\n".to_owned(),
            })
        );

        for testcase in &testcases[3..] {
            assert!(testcase.is_none());
        }
    }
}
