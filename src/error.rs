#[derive(Debug)]
pub enum Error {
    Write { url: String, e: std::io::Error },
    Fetch { url: String, e: reqwest::Error },
}

impl<S: AsRef<str>> From<(S, std::io::Error)> for Error {
    fn from((url, e): (S, std::io::Error)) -> Self {
        Error::Write {
            url: url.as_ref().to_string(),
            e,
        }
    }
}

impl<S: AsRef<str>> From<(S, reqwest::Error)> for Error {
    fn from((url, e): (S, reqwest::Error)) -> Self {
        Error::Fetch {
            url: url.as_ref().to_string(),
            e,
        }
    }
}
