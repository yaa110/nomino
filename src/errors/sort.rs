use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SortOrderError(String);

impl fmt::Display for SortOrderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid sort order: {}", self.0)
    }
}

impl Error for SortOrderError {}

impl SortOrderError {
    pub fn new(invalid_order: &str) -> Self {
        Self(invalid_order.to_string())
    }
}
