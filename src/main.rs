use atty::Stream;
use clap::{load_yaml, App};
use colored::{self, Colorize};
use nomino::errors::SourceError;
use nomino::input::{Context, Output, Source};
use std::env::{args, set_current_dir};
use std::error::Error;
use std::path::Path;
use std::process::exit;

fn read_source(
    regex: Option<&str>,
    sort: Option<&str>,
    map: Option<&str>,
) -> Result<Source, Box<dyn Error>> {
    match (regex, sort, map) {
        (Some(pattern), _, _) => Source::new_regex(pattern),
        (_, Some(order), _) => Source::new_sort(order),
        (_, _, Some(filename)) => Source::new_map(filename),
        _ => {
            colored::control::set_override(atty::is(Stream::Stderr));
            Err(Box::new(SourceError::new(format!(
                "one of '{}', '{}' or '{}' options must be set.\n{}: run '{} {}' for more information.",
                "regex".cyan(),
                "sort".cyan(),
                "map".cyan(),
                "usage".yellow().bold(),
                args().next().unwrap().cyan(),
                "--help".cyan(),
            ))))
        }
    }
}

fn read_output(output: Option<&str>) -> Result<Option<Output>, Box<dyn Error>> {
    if output.is_none() {
        return Ok(None);
    }
    Ok(Some(Output::new(output.unwrap())?))
}

fn run_app() -> Result<(), Box<dyn Error>> {
    let opts_format = load_yaml!("opts.yml");
    let opts = App::from_yaml(opts_format).get_matches();
    if let Some(cwd) = opts.value_of("directory").map(Path::new) {
        set_current_dir(cwd)?;
    }
    let mut context = Context::new(
        read_source(
            opts.value_of("regex"),
            opts.value_of("sort"),
            opts.value_of("map"),
        )?,
        opts.is_present("test"),
        opts.is_present("generate"),
        read_output(opts.value_of("output"))?,
    );
    let map_file = context.map_files()?;
    Ok(())
}

fn main() {
    exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            colored::control::set_override(atty::is(Stream::Stderr));
            eprintln!("{}: {}", "error".red().bold(), err.to_string());
            1
        }
    });
}
