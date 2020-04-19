# nomino

[![Test](https://github.com/yaa110/nomino/workflows/Test/badge.svg)](https://github.com/yaa110/nomino/actions) [![Download](https://img.shields.io/badge/download-releases-blue.svg)](https://github.com/yaa110/nomino/releases/latest) [![Benchmark](https://img.shields.io/badge/test-benchmark-orange.svg)](https://github.com/yaa110/nomino/wiki)

Batch rename utility for developers

![Alt text](/screenshots/nomino.png?raw=true "Regex Screenshot")

## How to install

### Pre-Compiled

You can download a pre-compiled executable for Linux, MacOS and Windows operating systems, then you should copy that executable to a location from your `$PATH` env:

- [Linux 64bit](https://github.com/yaa110/nomino/releases/latest/download/nomino-linux-64bit)
- [MacOS 64bit](https://github.com/yaa110/nomino/releases/latest/download/nomino-macos-64bit)
- [Windows 64bit](https://github.com/yaa110/nomino/releases/latest/download/nomino-windows-64bit.exe)

You might need to run `chmod +x nomino-linux-64bit` or `chmod +x nomino-macos-64bit`.

### Arch Linux

You can use [nomino](https://aur.archlinux.org/packages/nomino)<sup>AUR</sup> or [nomino-bin](https://aur.archlinux.org/packages/nomino-bin/)<sup>AUR</sup> packages to install nomino in Arch Linux.

The [nomino](https://aur.archlinux.org/packages/nomino)<sup>AUR</sup> package depends on [rust](https://www.archlinux.org/packages/extra/x86_64/rust) package, if you have installed `rust` using `rustup`, then use `makepkg -dsi` to install it by ignoring dependencies.

### Build Manually

If you prefer to build nomino manually, or a pre-compiled executable is not provided for your target, then you can build nomino from scratch:

- Install Rust: `curl -sSf https://sh.rustup.rs | sh`
- Run `cargo install nomino`

## Usage

```bash
USAGE:
    nomino [FLAGS] [OPTIONS] [OUTPUT]

FLAGS:
    -e, --extension    Preserves the extension of input files in 'sort' and 'regex' options
    -h, --help         Prints help information
    -w, --overwrite    Overwrites output files, otherwise, a '_' is prepended to filename
    -p, --print        Prints the map table to stdout
    -t, --test         Runs in test mode without renaming actual files
    -V, --version      Prints version information

OPTIONS:
    -d, --dir <PATH>         Sets the working directory
    -g, --generate <PATH>    Stores a JSON map file in '<PATH>' after renaming files
    -m, --map <PATH>         Sets the path of map file to be used for renaming files
    -r, --regex <PATTERN>    Regex pattern (RE2 syntax) to match by filenames
    -s, --sort <ORDER>       Sets the order of sorting (by name) to rename files using enumerator [possible values: ASC, DESC]

ARGS:
    <OUTPUT>    Output pattern to be used for renaming files
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

## Example

Consider the following directory:

```bash
➜  ls
Nomino (2020) S1.E1.1080p.mkv
Nomino (2020) S1.E2.1080p.mkv
Nomino (2020) S1.E3.1080p.mkv
Nomino (2020) S1.E4.1080p.mkv
Nomino (2020) S1.E5.1080p.mkv
```

Note that `-p` flag is used to print the table and `-e` flag is used to preserve the extension of input.

- Rename files using `regex` option:

```bash
➜  nomino -pr ".* S(\d+).E(\d+).*.(mkv)" "S{:2}E{:2}.{}"
+-------------------------------+------------+
| Input                         | Output     |
+-------------------------------+------------+
| Nomino (2020) S1.E1.1080p.mkv | S01E01.mkv |
| Nomino (2020) S1.E2.1080p.mkv | S01E02.mkv |
| Nomino (2020) S1.E3.1080p.mkv | S01E03.mkv |
| Nomino (2020) S1.E4.1080p.mkv | S01E04.mkv |
| Nomino (2020) S1.E5.1080p.mkv | S01E05.mkv |
+-------------------------------+------------+
```

- Rename files using `sort` option:

```bash
➜  nomino -pes asc "{:3}"
+-------------------------------+---------+
| Input                         | Output  |
+-------------------------------+---------+
| Nomino (2020) S1.E1.1080p.mkv | 001.mkv |
| Nomino (2020) S1.E2.1080p.mkv | 002.mkv |
| Nomino (2020) S1.E3.1080p.mkv | 003.mkv |
| Nomino (2020) S1.E4.1080p.mkv | 004.mkv |
| Nomino (2020) S1.E5.1080p.mkv | 005.mkv |
+-------------------------------+---------+
```

```bash
➜  nomino -pes desc "{:3}"
+-------------------------------+----------+
| Input                         | Output   |
+-------------------------------+----------+
| Nomino (2020) S1.E5.1080p.mkv | 001.mkv  |
| Nomino (2020) S1.E4.1080p.mkv | 002.mkv  |
| Nomino (2020) S1.E3.1080p.mkv | 003.mkv  |
| Nomino (2020) S1.E2.1080p.mkv | 004.mkv  |
| Nomino (2020) S1.E1.1080p.mkv | 005.mkv  |
+-------------------------------+----------+
```

- Rename files using the following `map.json` file:

```json
{
    "Nomino (2020) S1.E1.1080p.mkv": "0101.mkv",
    "Nomino (2020) S1.E2.1080p.mkv": "0102.mkv",
    "Nomino (2020) S1.E3.1080p.mkv": "0103.mkv",
    "Nomino (2020) S1.E4.1080p.mkv": "0104.mkv",
    "Nomino (2020) S1.E5.1080p.mkv": "0105.mkv"
}
```

```bash
➜  nomino -pm map.json
+-------------------------------+----------+
| Input                         | Output   |
+-------------------------------+----------+
| Nomino (2020) S1.E1.1080p.mkv | 0101.mkv |
| Nomino (2020) S1.E2.1080p.mkv | 0102.mkv |
| Nomino (2020) S1.E3.1080p.mkv | 0103.mkv |
| Nomino (2020) S1.E4.1080p.mkv | 0104.mkv |
| Nomino (2020) S1.E5.1080p.mkv | 0105.mkv |
+-------------------------------+----------+
```

- Undo renaming files: rename files by creating a map file using `-g` option, then use that map file to undo renaming:

```bash
➜  nomino -g undo.json -pr ".*.(mkv)" "a.{}"
+-------------------------------+-----------+
| Input                         | Output    |
+-------------------------------+-----------+
| Nomino (2020) S1.E1.1080p.mkv | ____a.mkv |
| Nomino (2020) S1.E4.1080p.mkv | ___a.mkv  |
| Nomino (2020) S1.E3.1080p.mkv | __a.mkv   |
| Nomino (2020) S1.E2.1080p.mkv | _a.mkv    |
| Nomino (2020) S1.E5.1080p.mkv | a.mkv     |
+-------------------------------+-----------+

➜  nomino -pm undo.json
+-----------+-------------------------------+
| Input     | Output                        |
+-----------+-------------------------------+
| ____a.mkv | Nomino (2020) S1.E1.1080p.mkv |
| _a.mkv    | Nomino (2020) S1.E2.1080p.mkv |
| __a.mkv   | Nomino (2020) S1.E3.1080p.mkv |
| ___a.mkv  | Nomino (2020) S1.E4.1080p.mkv |
| a.mkv     | Nomino (2020) S1.E5.1080p.mkv |
+-----------+-------------------------------+
```

### Benchmark

Please refer to [wiki](https://github.com/yaa110/nomino/wiki) for benchmark results of similar tools.
