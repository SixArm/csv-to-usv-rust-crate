# csv-to-usv

Convert Comma Separated Values (CSV) to [Unicode Separated Values (USV)](https://github.com/sixarm/usv).

Syntax:

```sh
stdin | csv-to-usv [options] | stdout
```

Example:

```sh
cat example.csv | csv-to-usv
```

Example with output to a file:

```sh
cat example.csv | csv-to-usv > example.usv
```

Example with custom delimiter:

```sh
cat example.csv | csv-to-usv --delimiter ";"
```


## Options

Options for CSV parsing:

* -d, --delimiter : Set the CSV field delimiter character.

Options for USV separators and modifiers:

* -u, --unit-separator : Set the unit separator string.

* -r, --record-separator : Set the record separator string.

Options for USV style sets:

* --style-braces : Set the style to use braces, such as "{US}" for Unit Separator.

* --style-controls : Set the style to use controls, such as "\u{001F}" for Unit Separator.

* --style-symbols : Set the style to use symbols, such as "␟" for Unit Separator.

* --style-liners : Set the style to use liners wrapping every symbol, such as "\n␟\n" for Unit Separator.

* --style-sheets : Set the style similar to spreadsheet sheets, such as "␟" for Unit Separator and "␟\n" for Record Separator.

Options for command line tools:

* -h, --help : Print help

* -V, --version : Print version

* -v, --verbose... : Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace. Example: --verbose …

* --test : Print test output for debugging, verifying, tracing, and the like. Example: --test


## Install

Install:

```sh
cargo install csv-to-usv
```

Link: [https://crates.io/crates/csv-to-usv](https://crates.io/crates/csv-to-usv)


## Example

Suppose file example.csv contains:

```csv
a,b
c,d
```

Run:

```sh
cat example.csv | csv-to-usv
```

Output:

```usv
a␟b␟␞c␟d␟␞
```

Run:

```sh
cat example.csv | csv-to-usv --style-sheets
```

Output:

```usv
a␟b␟␞
c␟d␟␞
```

## FAQ

### When to use this command?

Use this command when you want to convert from CSV to USV.

A typical use case is when you have CSV data, such as a spreadsheet export,
and you want to convert it to USV, such as to make the data easier to view,
or edit, or maintain.

Our real-world use case is converting a bunch of CSV spreadsheet exports
from a variety of programs, including Excel, to USV so we're better-able to
handle quoting, and multi-line data units, and Unicode characters in a wide
variety of human languages.

### Is there a similar command to convert from USV to CSV?

Yes: [usv-to-csv](https://crates.io/crates/usv-to-csv).

### Why use USV instead of CSV?

See the documentation for [USV](https://github.com/sixarm/usv).

### Is USV aiming to become a standard?

Yes, USV is submitted to IETF.org as an Internet-Draft work in progress:
[link](https://datatracker.ietf.org/doc/draft-unicode-separated-values/).

## Help wanted

Constructive feedback welcome. Pull requests and feature requests welcome.

## Tracking

* Package: csv-to-usv-rust-crate
* Version: 1.4.0
* Created: 2024-03-09T13:33:20Z
* Updated: 2024-03-19T17:12:06Z
* License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
* Contact: Joel Parker Henderson (joel@sixarm.com)
