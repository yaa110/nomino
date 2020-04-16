use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // Read template file
    let mut template = File::open("data/opts-template.yml")?;
    let mut template_contents = String::new();
    template.read_to_string(&mut template_contents)?;

    // Replace metadata from Cargo.toml
    template_contents = template_contents
        .replace("%VERSION%", env!("CARGO_PKG_VERSION"))
        .replace("%NAME%", env!("CARGO_PKG_NAME"))
        .replace("%DESCRIPTION%", env!("CARGO_PKG_DESCRIPTION"));

    // Write to opts
    let mut opts = File::create("src/opts.yml")?;
    opts.write_all(template_contents.as_bytes())?;
    opts.sync_all()?;

    Ok(())
}
