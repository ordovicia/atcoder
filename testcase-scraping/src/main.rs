#![allow(unused)]

use testcase_scraping::{Result, Session, Testcase};

fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_module("testcase_scraping", log::LevelFilter::Info)
        .default_format_timestamp(false)
        .init();

    let mut args = std::env::args();

    let contest = match args.nth(1) {
        Some(c) => c,
        None => {
            log::error!("Invalid usage.");
            return Ok(());
        }
    };

    let tasks = {
        let mut t = args.collect::<Vec<_>>();
        if t.is_empty() {
            ["a", "b", "c", "d"].iter().map(|&t| t.to_owned()).collect()
        } else {
            t
        }
    };

    let session = Session::new()?;
    let testcases = session.fetch_testcases(&contest, tasks.iter().map(|t| t.as_ref()))?;
    testcase_scraping::write(&testcases)?;

    Ok(())
}
