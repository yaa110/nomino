use async_std::fs;
use async_std::prelude::*;
use async_std::task;
use atty::Stream;
use clap::{load_yaml, App};
use colored::{self, Colorize};
use nomino::errors::SourceError;
use nomino::input::{Context, Formatter, Source};
use prettytable::{cell, format, row, Table};
use serde_json::map::Map;
use serde_json::value::Value;
use std::env::{args, set_current_dir};
use std::error::Error;
use std::path::Path;
use std::process::exit;

async fn read_source(
    regex: Option<&str>,
    sort: Option<&str>,
    map: Option<&str>,
) -> Result<Source, Box<dyn Error>> {
    match (regex, sort, map) {
        (Some(pattern), _, _) => Source::new_regex(pattern).await,
        (_, Some(order), _) => Source::new_sort(order).await,
        (_, _, Some(filename)) => Source::new_map(filename).await,
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

async fn read_output(output: Option<&str>) -> Result<Option<Formatter>, Box<dyn Error>> {
    if output.is_none() {
        return Ok(None);
    }
    Ok(Some(Formatter::new(output.unwrap()).await?))
}

async fn map_files(
    context: Context,
    test_mode: bool,
    need_map: bool,
) -> Result<Option<Map<String, Value>>, Box<dyn Error>> {
    let mut map_iter = context.into_iter().await?;
    let mut map = if need_map { Some(Map::new()) } else { None };
    let mut rename_result = true;
    while let Some((input, output)) = map_iter.next().await {
        if !test_mode {
            rename_result = fs::rename(input.as_str(), output.as_str()).await.is_ok();
        }
        if rename_result && need_map {
            map.as_mut().map(|m| m.insert(output, Value::String(input)));
        }
        rename_result = true;
    }
    Ok(map)
}

async fn print_map_table(map: Map<String, Value>) -> Result<(), Box<dyn Error>> {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!["Input".cyan(), "Output".cyan()]);
    map.into_iter()
        .enumerate()
        .for_each(|(i, (output, input))| {
            if let Value::String(input) = input {
                if i % 2 == 0 {
                    table.add_row(row![input.as_str(), output.as_str()]);
                } else {
                    table.add_row(row![input.as_str().purple(), output.as_str().purple()]);
                }
            }
        });
    table.printstd();
    Ok(())
}

async fn run_app() -> Result<(), Box<dyn Error>> {
    let opts_format = load_yaml!("opts.yml");
    let opts = App::from_yaml(opts_format).get_matches();
    if let Some(cwd) = opts.value_of("directory").map(Path::new) {
        set_current_dir(cwd)?;
    }
    let context = Context::new(
        read_source(
            opts.value_of("regex"),
            opts.value_of("sort"),
            opts.value_of("map"),
        )
        .await?,
        read_output(opts.value_of("output")).await?,
        opts.is_present("extension"),
    )
    .await;
    let print_map = opts.is_present("print");
    let generate_map = opts.value_of("generate");
    let test_mode = opts.is_present("test");
    let map = map_files(context, test_mode, print_map || generate_map.is_some()).await?;
    if let Some(map_file) = generate_map {
        fs::write(
            map_file,
            serde_json::to_vec_pretty(map.as_ref().unwrap())?.as_slice(),
        )
        .await?;
    }
    if print_map {
        colored::control::set_override(atty::is(Stream::Stdout));
        print_map_table(map.unwrap()).await?;
    }
    Ok(())
}

async fn async_main() -> i32 {
    match run_app().await {
        Ok(_) => 0,
        Err(err) => {
            colored::control::set_override(atty::is(Stream::Stderr));
            eprintln!("{}: {}", "error".red().bold(), err.to_string());
            1
        }
    }
}

fn main() {
    exit(task::block_on(async_main()));
}
