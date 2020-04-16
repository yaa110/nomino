use crate::input::{Output, Source};
use std::collections::HashMap;
use std::error::Error;

pub struct Context {
    source: Source,
    test_mode: bool,
    generate_map: bool,
    output: Option<Output>,
}

impl Context {
    pub fn new(
        source: Source,
        test_mode: bool,
        generate_map: bool,
        output: Option<Output>,
    ) -> Self {
        Self {
            source,
            test_mode,
            generate_map,
            output,
        }
    }

    pub fn map_files(&mut self) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let map = HashMap::new();
        for filename in self.source.try_iter()? {
            // TODO
        }
        Ok(map)
    }
}
