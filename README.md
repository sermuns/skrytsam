<a href="https://github.com/sermuns/skrytsam">
  <img src="media/banner.png">
</a>

<div align="center">
  <p>
    <em>
        blazingly fast github profile stats for your README
    </em>
  </p>
  <a href="https://github.com/sermuns/skrytsam/releases/latest">
    <img alt="release-badge" src="https://img.shields.io/github/v/release/sermuns/skrytsam.svg"></a>
  <a href="https://github.com/sermuns/skrytsam/blob/main/LICENSE">
    <img alt="WTFPL" src="https://img.shields.io/badge/License-WTFPL-brightgreen.svg"></a>
  <a href="https://crates.io/crates/skrytsam"><img src="https://img.shields.io/crates/v/skrytsam.svg" alt="Version info"></a>
</div>
<br>

> [!NOTE]
> work in progress, only [languages](#languages) card is implemented

`skrytsam` is a command-line tool that fetches GitHub profile statistics and generates SVG cards for use in your profile README.



## Usage

```present cargo run -- -h
generate pretty svgs for your profile on GitHub

Usage: skrytsam [OPTIONS] <GITHUB_USERNAME> <COMMAND>

Commands:
  languages  
  help       Print this message or the help of the given subcommand(s)

Arguments:
  <GITHUB_USERNAME>  

Options:
      --skip-forks     don't include repos that are forks
      --skip-private   don't include private repos
      --skip-archived  don't include archived repos
  -h, --help           Print help
  -V, --version        Print version
```

## Cards

### `languages`

![languages](media/example/languages.svg)

```present cargo run -- languages -h
Usage: skrytsam <GITHUB_USERNAME> languages [OPTIONS]

Options:
  -s, --skipped-languages <SKIPPED_LANGUAGES>
          don't include these languages
  -n, --num-languages <NUM_LANGUAGES>
          how many languages to show the rest will be merged into "Other" 0 means infinite [default: 5]
  -o, --output <OUTPUT>
          where to put output svg `-` to use stdout [default: languages.svg]
  -h, --help
          Print help
```

## Installation

- from source:

  ```sh
  cargo install skrytsam
  ```

- from binaries, using [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall):

  ```sh
  cargo binstall skrytsam
  ```

- or download and extract the [latest release](https://github.com/sermuns/skrytsam/releases/latest)
