use crate::input::{Output, Source};

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
}
