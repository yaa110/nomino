# nomino

[![Build Status](https://travis-ci.org/yaa110/nomino.svg?branch=master)](https://travis-ci.org/yaa110/nomino) [![Download](https://img.shields.io/badge/download-release-blue.svg)](https://github.com/yaa110/nomino/releases)

Batch rename utility for developers

## How to install

### Pre-Compiled

you can download a [pre-compiled executable](https://github.com/yaa110/nomino/releases) for Linux, then you should copy that executable to `/usr/bin` or add it to your `$PATH` env. Do not forget to `chmod +x nomino`.

### Build Manually

- Install rust: `curl -sSf https://sh.rustup.rs | sh`
- Run `cargo install --git https://github.com/yaa110/nomino.git`

## Usage

```bash
USAGE:
    nomino [FLAGS] [OPTIONS] [OUTPUT]

FLAGS:
    -e, --extension    Preserves the extension of input files in 'sort' and 'regex' options
    -h, --help         Prints help information
    -p, --print        Prints the map table to stdout
    -t, --test         Runs in test mode without renaming actual files
    -V, --version      Prints version information

OPTIONS:
    -d, --dir <PATH>         Sets the working directory
    -g, --generate <PATH>    Stores a JSON map file in 'PATH' after renaming files
    -m, --map <PATH>         Sets the path of map file to be used for renaming files
    -r, --regex <PATTERN>    Regex pattern (RE2 syntax) to match by filenames
    -s, --sort <ORDER>       Sets the order of sorting (by name) to rename files using enumerator [possible values: ASC, DESC]

ARGS:
    <OUTPUT>    Output pattern to be used for renaming files
```

It _might_ work on Windows, MacOS and other operating systems, however, the pre-compiled executable only is generated for Linux.

### Examples

- Rename all *.mkv files in a folder: `nomino -e -r "name 2020 season (\d+) episode (\d+).mkv" "S{:2}E{:2}"`. A filename of `name 2020 season 1 episode 5.mkv` is matched and renamed to `S01E05.mkv`.
- Store a JSON map file after renaming files: `nomino -g "map.json" -r "PATTERN" "OUTPUT"`. The map file could be used to undo renaming.

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
1. Padding is only used for positive numbers, e.g. the result `{:3}` for `1` is `001`, `-1` is `-1` and `a` is `a`.
1. If `--sort` option is used, the first index `{0}` is the filename and the second index `{1}` or first occurrence of `{}` is the enumerator index.
