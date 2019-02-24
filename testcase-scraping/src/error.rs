#[derive(Debug)]
pub enum Error {
    Url(url::ParseError),
    Reqwest(reqwest::Error),
    HttpStatus(reqwest::StatusCode),
    Testcase(&'static str),
    Io(std::io::Error),
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::Url(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<reqwest::StatusCode> for Error {
    fn from(e: reqwest::StatusCode) -> Self {
        Error::HttpStatus(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
