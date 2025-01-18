# nomino

[![Test](https://github.com/yaa110/nomino/workflows/Test/badge.svg)](https://github.com/yaa110/nomino/actions) [![Download](https://img.shields.io/badge/download-releases-blue.svg)](https://github.com/yaa110/nomino/releases/latest) [![Wiki](https://img.shields.io/badge/wiki-docs-orange.svg)](https://github.com/yaa110/nomino/wiki)

Batch rename utility for developers

![Alt text](/screenshots/regex.png?raw=true "Example Screenshot")

## How to install

### Pre-Compiled

You can download a pre-compiled executable for Linux, MacOS and Windows operating systems, then you should copy that executable to a location from your `$PATH` env:

- [Linux 64bit](https://github.com/yaa110/nomino/releases/latest/download/nomino-linux-64bit)
- [MacOS 64bit](https://github.com/yaa110/nomino/releases/latest/download/nomino-macos-64bit)
- [Windows 64bit](https://github.com/yaa110/nomino/releases/latest/download/nomino-windows-64bit.exe)

You might need to run `chmod +x nomino-linux-64bit` or `chmod +x nomino-macos-64bit`.

### Build Manually

If you prefer to build nomino manually, or a pre-compiled executable is not provided for your target, then you can build nomino from scratch:

- Install Rust: `curl -sSf https://sh.rustup.rs | sh`
- Run `cargo install nomino`

## Usage

```bash
Usage:
    nomino [OPTIONS] [[SOURCE] OUTPUT]...

Arguments:
  [[SOURCE] OUTPUT]...
          OUTPUT is the pattern to be used for renaming files, and SOURCE is the optional regex pattern to match by filenames. SOURCE has the same function as -r option

Options:
  -d, --dir <PATH>          Sets the working directory
      --depth <DEPTH>       Optional value to overwrite inferred subdirectory depth value in 'regex' mode
  -E, --no-extension        Does not preserve the extension of input files in 'sort' and 'regex' options
  -g, --generate <PATH>     Stores a JSON map file in '<PATH>' after renaming files
  -h, --help                Print help (see a summary with '-h')
  -k, --mkdir               Recursively creates all parent directories of '<OUTPUT>' if they are missing
  -m, --map <PATH>          Sets the path of map file to be used for renaming files
      --from-file <PATH>    Alias for --map
      --max-depth <DEPTH>   Optional value to set the maximum of subdirectory depth value in 'regex' mode
  -q, --quiet               Does not print the map table to stdout
  -r, --regex <PATTERN>     Regex pattern to match by filenames
  -s, --sort <ORDER>        Sets the order of natural sorting (by name) to rename files using enumerator
                                Possible ORDER values:
                                - asc:  Sort in ascending order
                                - desc: Sort in descending order
  -t, --test                Runs in test mode without renaming actual files
      --dry-run             Alias for --test
  -V, --version             Print version
  -w, --overwrite           Overwrites output files, otherwise, a '_' is prepended to filename

OUTPUT pattern accepts placeholders that have the format of '{G:P}' where 'G' is the captured group and 'P' is the padding of digits with `0`. Please refer to https://github.com/yaa110/nomino for more information.
```

### Placeholders

1. Placeholders have the format of `{G:P}` where `G` is the captured group and `P` is the padding of digits with `0`. For example, `{2:3}` means the third captured group with a padding of 3, i.e. `1` is formatted as `001`.
1. Indices start from `0`, and `{0}` means the filename.
1. The capture group `G` could be dropped, i.e. `{}` or `{:3}`. In this case an auto incremental index is used which starts from `1`. For example, `{} {}` equals `{1} {2}`.
1. `{` and `}` characters could be escaped using `\` character, i.e. `\\{` and `\\}` in cli.
1. Padding is only used for positive numbers, e.g. the formatted result of `{:3}` for `1` is `001`, for `-1` is `-1` and for `a` is `a`.
1. If `--sort` option is used, the first index `{0}` is the filename and the second index `{1}` or first occurrence of `{}` is the enumerator index.

### Capture Groups

The accepted syntax of regex pattern is [Rust Regex](https://docs.rs/regex/latest/regex/).

Consider this example:

```regex
(?<first>\w)(\w)\w(?<last>\w)
```

This regular expression defines 4 capture groups:

- The group at index `0` corresponds to the overall match. It is always present in every match and never has a name: `{0}`.
- The group at index `1` with name `first` corresponding to the first letter: `{1}`, `{first}` or the first occurrence of `{}`.
- The group at index `2` with no name corresponding to the second letter: `{2}` or the second occurrence of `{}`.
- The group at index `3` with name `last` corresponding to the fourth and last letter: `{3}`, `{last}` or the third occurrence of `{}`.

`?<first>` and `?<last>` are named capture groups.

### Windows

On Windows, `\\` must be used to separate path components in file paths because `\` is a special character in regular expressions.

## Map file format

```json
{
    "<input1>": "<output1>",
    "<input2>": "<output2>",
    "<...>": "<...>"
}
```

## Wiki

- **[Examples](https://github.com/yaa110/nomino/wiki/Examples)** learn nomino by examples
- **[Benchmark](https://github.com/yaa110/nomino/wiki/Benchmark)** benchmark test of similar utilities to nomino
