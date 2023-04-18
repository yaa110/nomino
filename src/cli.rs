use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(
    about,
    author,
    version,
    after_help = "OUTPUT pattern accepts placeholders that have the format of '{I:P}' where 'I' \
    is the index of captured group and 'P' is the padding of digits with `0`. Please refer to \
    https://github.com/yaa110/nomino for more information.",
    next_display_order = None,
)]
pub struct Cli {
    /// Runs in test mode without renaming actual files (dry-run).
    #[arg(short, long)]
    pub test: bool,
    /// Recursively creates all parent directories of '<OUTPUT>' if they are missing.
    #[arg(short = 'k', long)]
    pub mkdir: bool,
    /// Overwrites output files, otherwise, a '_' is prepended to filename.
    #[arg(short = 'w', long)]
    pub overwrite: bool,
    /// Preserves the extension of input files in 'sort' and 'regex' options.
    #[arg(short, long)]
    pub extension: bool,
    /// Sets the working directory.
    #[arg(short, long = "dir", value_name = "PATH")]
    pub directory: Option<PathBuf>,
    /// Optional value to set the maximum of subdirectory depth value in 'regex' mode.
    #[arg(long, value_name = "DEPTH")]
    pub max_depth: Option<usize>,
    /// Optional value to overwrite inferred subdirectory depth value in 'regex' mode.
    #[arg(long)]
    pub depth: Option<usize>,
    /// Stores a JSON map file in '<PATH>' after renaming files.
    #[arg(short, long, value_name = "PATH")]
    pub generate: Option<PathBuf>,
    /// Prints the map table to stdout.
    #[arg(short, long)]
    pub print: bool,
    /// Sets the path of map file to be used for renaming files.
    #[arg(short, long, value_name = "PATH")]
    pub map: Option<PathBuf>,
    /// Sets the order of natural sorting (by name) to rename files using enumerator.
    #[arg(short, long, value_name = "ORDER", ignore_case = true)]
    pub sort: Option<Order>,
    /// Regex pattern (RE2 syntax) to match by filenames.
    #[arg(short, long, value_name = "PATTERN", requires = "output")]
    pub regex: Option<String>,
    /// OUTPUT is the pattern to be used for renaming files, and SOURCE is the optional regex pattern to match by filenames. SOURCE has the same function as -r option.
    #[arg(value_name = "[SOURCE] OUTPUT")]
    pub output: Vec<String>,
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

#[derive(Clone, ValueEnum)]
pub enum Order {
    /// Sort in ascending order.
    Asc,
    /// Sort in descending order.
    Desc,
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        super::Cli::command().debug_assert();
    }
}
