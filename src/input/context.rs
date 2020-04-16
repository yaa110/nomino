use crate::input::{Formatter, InputStream, Source};
use std::error::Error;

pub struct Context {
    source: Source,
    formatter: Option<Formatter>,
}

impl Context {
    pub async fn new(source: Source, formatter: Option<Formatter>) -> Self {
        Self { source, formatter }
    }

    pub async fn into_iter(self) -> Result<InputStream, Box<dyn Error>> {
        InputStream::try_from(self.source, self.formatter).await
    }
}
