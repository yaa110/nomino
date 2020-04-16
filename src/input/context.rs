use crate::input::{Output, Source};
use async_std::prelude::*;
use std::collections::HashMap;
use std::error::Error;

pub struct Context {
    source: Source,
    output: Option<Output>,
}

impl Context {
    pub async fn new(source: Source, output: Option<Output>) -> Self {
        Self { source, output }
    }

    pub async fn files_map(&mut self) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let map = HashMap::new();
        let mut map_iter = self.source.try_iter().await?;
        while let Some(filename) = map_iter.next().await {
            // TODO
        }
        Ok(map)
    }
}
