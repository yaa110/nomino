use crate::input::{Formatter, InputStream, Source};
use std::error::Error;

pub struct Context {
    source: Source,
    formatter: Option<Formatter>,
    preserve_extension: bool,
}

impl Context {
    pub async fn new(
        source: Source,
        formatter: Option<Formatter>,
        preserve_extension: bool,
    ) -> Self {
        Self {
            source,
            formatter,
            preserve_extension,
        }
    }

    pub async fn into_iter(self) -> Result<InputStream, Box<dyn Error>> {
        InputStream::try_from(self.source, self.formatter, self.preserve_extension).await
    }
}
