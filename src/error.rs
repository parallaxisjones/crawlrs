use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct FetchError {
    details: String,
}

impl FetchError {
    pub fn new(msg: &str) -> FetchError {
        FetchError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for FetchError {
    fn description(&self) -> &str {
        &self.details
    }
}
