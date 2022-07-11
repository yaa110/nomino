# nomino

[![Test](https://github.com/yaa110/nomino/workflows/Test/badge.svg)](https://github.com/yaa110/nomino/actions) [![Download](https://img.shields.io/badge/download-releases-blue.svg)](https://github.com/yaa110/nomino/releases/latest) [![Wiki](https://img.shields.io/badge/wiki-docs-orange.svg)](https://github.com/yaa110/nomino/wiki)

Batch rename utility for developers

![Alt text](/screenshots/usage.png?raw=true "Regex Screenshot")

## How to install

### Pre-Compiled

You can download a pre-compiled executable for Linux, MacOS and Windows operating systems, then you should copy that executable to a location from your `$PATH` env:

- [Linux 64bit](https://github.com/yaa110/nomino/releases/latest/download/nomino-linux-64bit)
- [MacOS 64bit](https://github.com/yaa110/nomino/releases/latest/download/nomino-macos-64bit)
- [Windows 64bit](https://github.com/yaa110/nomino/releases/latest/download/nomino-windows-64bit.exe)

You might need to run `chmod +x nomino-linux-64bit` or `chmod +x nomino-macos-64bit`.

### Arch Linux

You can use [nomino](https://aur.archlinux.org/packages/nomino)<sup>AUR</sup> package to install nomino in Arch Linux.

### Build Manually

If you prefer to build nomino manually, or a pre-compiled executable is not provided for your target, then you can build nomino from scratch:

- Install Rust: `curl -sSf https://sh.rustup.rs | sh`
- Run `cargo install nomino`

## Usage

```bash
USAGE:
    nomino [FLAGS] [OPTIONS] [[SOURCE] OUTPUT]...

FLAGS:
    -e, --extension    Preserves the extension of input files in 'sort' and 'regex' options
    -h, --help         Prints help information
    -k, --mkdir        Recursively creates all parent directories of '<OUTPUT>' if they are missing
    -w, --overwrite    Overwrites output files, otherwise, a '_' is prepended to filename
    -p, --print        Prints the map table to stdout
    -t, --test         Runs in test mode without renaming actual files (dry-run)
    -V, --version      Prints version information

OPTIONS:
        --depth <DEPTH>        Optional value to overwrite inferred subdirectory depth value in 'regex' mode
        --max-depth <DEPTH>    Optional value to set the maximum of subdirectory depth value in 'regex' mode
    -d, --dir <PATH>           Sets the working directory
    -g, --generate <PATH>      Stores a JSON map file in '<PATH>' after renaming files
    -m, --map <PATH>           Sets the path of map file to be used for renaming files
    -r, --regex <PATTERN>      Regex pattern (RE2 syntax) to match by filenames
    -s, --sort <ORDER>         Sets the order of natural sorting (by name) to rename files using enumerator [possible values: ASC, DESC]

ARGS:
    <[SOURCE] OUTPUT>...    OUTPUT is the pattern to be used for renaming files, and SOURCE is the optional regex pattern to match by filenames. SOURCE has the same function as -r option
```

## Map file format

```json
{
    "<input1>": "<output1>",
    "<input2>": "<output2>",
    "<...>": "<...>"
}
```

## Output

The output is necessary when using `--sort` or `--regex` options.

### Regex

The accepted syntax of regex pattern is [RE2](https://github.com/google/re2/wiki/Syntax).

### Placeholders

1. Placeholders have the format of `{I:P}` where `I` is the index of captured group and `P` is the padding of digits with `0`. For example, `{2:3}` means the third captured group with a padding of 3, i.e. `1` is formatted as `001`.
1. Indices start from `0`, and `{0}` means the filename.
1. The index `I` could be dropped, i.e. `{}` or `{:3}`. In this case an auto incremental index is used which starts from `1`. For example, `{} {}` equals `{1} {2}`.
1. `{` and `}` characters could be escaped using `\` character, i.e. `\\{` and `\\}` in cli.
1. Padding is only used for positive numbers, e.g. the formatted result of `{:3}` for `1` is `001`, for `-1` is `-1` and for `a` is `a`.
1. If `--sort` option is used, the first index `{0}` is the filename and the second index `{1}` or first occurrence of `{}` is the enumerator index.

## Wiki

- **[Examples](https://github.com/yaa110/nomino/wiki/Examples)** learn nomino by examples
- **[Benchmark](https://github.com/yaa110/nomino/wiki/Benchmark)** benchmark test of similar utilities to nomino
