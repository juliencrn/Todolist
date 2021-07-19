<div align="center">
<h1>Rusty Journal</h1>

A command line to-do app written in Rust.

**README Sections:** [Options](#options) — [Installation](#installation) - [License](#license)

<!-- Badges -->
</div>

<!-- ![Screenshots of Rusty Journal](screenshots.png) -->

---

## Options

```
Rusty Journal 0.1.0
A command line to-do app written in Rust

USAGE:
    rusty-journal [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -j, --journal-file <journal-file>    Use a different journal file

SUBCOMMANDS:
    add     Write tasks to the journal file
    done    Remove an entry from the journal file by position
    help    Prints this message or the help of the given subcommand(s)
    list    List all tasks in the journal file
```

---

## Installation

_Rusty journal_ is written in [Rust](https://www.rust-lang.org/), so you'll need to grab a Rust installation in order to compile it. The recommended way to install Rust for development is from the [official download page](https://www.rust-lang.org/tools/install), using rustup.

Once Rust is installed, you can compile _Rusty journal_ with Cargo:

```bash
$ git clone https://github.com/juliencrn/rusty-journal
$ cd rusty-journal
$ cargo build --release
$ ./target/release/rusty-journal --version
Rusty Journal 0.1.0
```

## License
Distributed under the MIT License. See [`LICENSE`](./LICENSE) for more information.
