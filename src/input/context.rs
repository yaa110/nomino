use crate::input::{Formatter, InputIterator, Source};
use std::error::Error;
use std::iter::IntoIterator;

pub struct Context(InputIterator);

impl Context {
    pub fn new(
        source: Source,
        formatter: Option<Formatter>,
        preserve_extension: bool,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self(InputIterator::new(
            source,
            formatter,
            preserve_extension,
        )?))
    }
}

impl IntoIterator for Context {
    type Item = (String, String);
    type IntoIter = InputIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.0
    }
}
