use anyhow::{ensure, Result};
use colored::{self, Colorize};
use is_terminal::IsTerminal;
use nomino::cli::{Cli, Order};
use nomino::errors::SourceError;
use nomino::input::{Formatter, InputIterator, Source};
use prettytable::{format, row, Table};
use serde_json::map::Map;
use serde_json::value::Value;
use std::env::{args, set_current_dir};
use std::fs;
use std::path::Path;
use std::process::exit;

fn read_source(
    regex: Option<(&str, Option<usize>, Option<usize>)>,
    sort: Option<Order>,
    map: Option<&Path>,
) -> Result<Source> {
    match (regex, sort, map) {
        (Some((pattern, depth, max_depth)), _, _) => Source::new_regex(pattern, depth, max_depth),
        (_, Some(order), _) => Source::new_sort(order),
        (_, _, Some(filename)) => Source::new_map(filename),
        _ => {
            colored::control::set_override(std::io::stderr().is_terminal());
            Err(SourceError::new(format!(
                "one of '{}', '{}', '{}' or '{}' options must be set.\n{}: run '{} {}' for more information.",
                "regex".cyan(),
                "sort".cyan(),
                "map".cyan(),
                "SOURCE".cyan(),
                "usage".yellow().bold(),
                args().next().unwrap().cyan(),
                "--help".cyan(),
            )).into())
        }
    }
}

fn read_output(output: Option<&str>) -> Result<Option<Formatter>> {
    if output.is_none() {
        return Ok(None);
    }
    Ok(Some(Formatter::new(output.unwrap())?))
}

fn rename_files(
    input_iter: InputIterator,
    test_mode: bool,
    need_map: bool,
    overwrite: bool,
    mkdir: bool,
) -> (Option<Map<String, Value>>, bool) {
    let mut map = if need_map { Some(Map::new()) } else { None };
    let mut is_renamed = true;
    let mut with_err = false;
    for (input, mut output) in input_iter {
        if input.as_str() == output.as_str() {
            map.as_mut().map(|m| m.insert(output, Value::String(input)));
            continue;
        }
        let mut file_path_buf;
        let mut file_path = Path::new(output.as_str());
        if !overwrite {
            while file_path.exists() {
                file_path_buf = file_path
                    .with_file_name(
                        (String::from("_")
                            + file_path
                                .file_name()
                                .and_then(|name| name.to_str())
                                .unwrap_or_default())
                        .as_str(),
                    )
                    .to_path_buf();
                file_path = file_path_buf.as_path();
                output = file_path.to_string_lossy().to_string();
            }
        }
        if mkdir {
            let _ = file_path
                .parent()
                .and_then(|parent| fs::create_dir_all(parent).ok());
        }
        if !test_mode {
            match fs::rename(input.as_str(), file_path) {
                Ok(_) => is_renamed = true,
                Err(e) => {
                    is_renamed = false;
                    with_err = true;
                    eprintln!(
                        "[{}] unable to rename '{}': {}",
                        "error".red().bold(),
                        input.as_str(),
                        e
                    );
                }
            }
        }
        if is_renamed && need_map {
            map.as_mut().map(|m| m.insert(output, Value::String(input)));
        }
        is_renamed = true;
    }
    (map, with_err)
}

fn print_map_table(map: Map<String, Value>) {
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
}

fn run_app() -> Result<bool> {
    let mut opts = Cli::parse();
    if let Some(cwd) = opts.directory {
        set_current_dir(cwd)?;
    }

    ensure!(
        (opts.regex.is_none() && opts.sort.is_none() && opts.map.is_none())
            || opts.output.len() <= 1,
        "optional SOURCE must be used without setting regex, map or sort flags",
    );

    let output = opts.output.pop();
    let pattern = opts.output.pop();
    let input_iter = InputIterator::new(
        read_source(
            opts.regex
                .or(pattern)
                .as_deref()
                .map(|pattern| (pattern, opts.depth, opts.max_depth)),
            opts.sort,
            opts.map.as_deref(),
        )?,
        read_output(output.as_deref())?,
        opts.extension,
    )?;
    let (map, with_err) = rename_files(
        input_iter,
        opts.test,
        opts.print || opts.generate.is_some(),
        opts.overwrite,
        opts.mkdir,
    );
    if let Some(map_file) = opts.generate {
        fs::write(
            map_file,
            serde_json::to_vec_pretty(map.as_ref().unwrap())?.as_slice(),
        )?;
    }
    if let Some(map) = map.filter(|map| opts.print && !map.is_empty()) {
        colored::control::set_override(std::io::stdout().is_terminal());
        print_map_table(map);
    }
    Ok(with_err)
}

fn main() {
    exit(match run_app() {
        Ok(with_err) => i32::from(with_err),
        Err(err) => {
            colored::control::set_override(std::io::stderr().is_terminal());
            eprintln!("{}: {}", "error".red().bold(), err);
            1
        }
    });
}
