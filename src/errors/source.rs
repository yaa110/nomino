use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SourceError(String);

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SourceError {}

impl SourceError {
    pub fn new(message: String) -> Self {
        Self(message)
    }
}
