use crate::input::{Formatter, InputIterator, Source};
use std::error::Error;

pub struct Context {
    source: Source,
    formatter: Option<Formatter>,
    preserve_extension: bool,
}

impl Context {
    pub fn new(source: Source, formatter: Option<Formatter>, preserve_extension: bool) -> Self {
        Self {
            source,
            formatter,
            preserve_extension,
        }
    }

    pub fn into_iter(self) -> Result<InputIterator, Box<dyn Error>> {
        InputIterator::try_from(self.source, self.formatter, self.preserve_extension)
    }
}
