use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct StrError<'a>(&'a str);

impl<'a> Error for StrError<'a> {}

impl<'a> fmt::Display for StrError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> StrError<'a> {
    pub fn boxed(msg: &'a str) -> Box<dyn Error + 'a> {
        Box::new(Self(msg))
    }
}
