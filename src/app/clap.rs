//! clap setup.
//!
//! clap is a crate for command line argument parsing.
//! See https://docs.rs/clap/
//!
//! We prefer clap using the `command!` macro, which runs at compile time.
//! We prefer clap using the builder pattern, which offers more capabilities.
//!
//! We favor our convention of doing clap setup in a file named `clap.rs`,
//! rather than in `main.rs`, because we favor the separation of concerns.

use clap::Arg;

pub fn clap() -> crate::app::args::Args {
    let matches = matches();
    crate::app::args::Args {
        test: matches.get_flag("test"),
        log_level: crate::app::log::u8_to_log_level(matches.get_count("verbose")),
        delimiter: *matches.get_one::<char>("delimiter").expect("delimiter is missing default value") as u8,
        style: style(&matches),
    }
}

fn matches() -> clap::ArgMatches {
    clap::command!()
    .name(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .arg(Arg::new("test")
        .long("test")
        .help("Print test output for debugging, verifying, tracing, and the like.\nExample: --test")
        .action(clap::ArgAction::SetTrue))
    .arg(Arg::new("verbose")
        .short('v')
        .long("verbose")
        .help("Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace.\nExample: --verbose …")
        .action(clap::ArgAction::Count))
    .arg(Arg::new("delimiter")
        .help("Set the delimiter character")
        .short('d')
        .long("delimiter")
        .default_value(",")
        .value_parser(clap::value_parser!(char))
        .action(clap::ArgAction::Set))
        .arg(Arg::new("unit-separator")
        .help("Set the unit separator string")
        .short('u')
        .long("unit-separator")
        .default_value(usv::UNIT_SEPARATOR_SYMBOL_STR)
        .action(clap::ArgAction::Set))
    .arg(Arg::new("record-separator")
        .help("Set the record separator string.")
        .short('r')
        .long("record-separator")
        .default_value(usv::RECORD_SEPARATOR_SYMBOL_STR)
        .action(clap::ArgAction::Set))
    .arg(Arg::new("group-separator")
        .help("Set the group separator string. Not currently used. Reserved for future use.")
        .short('g')
        .long("group-separator")
        .default_value(usv::GROUP_SEPARATOR_SYMBOL_STR)
        .action(clap::ArgAction::Set))
    .arg(Arg::new("file-separator")
        .help("Set the file separator string. Not currently used. Reserved for future use.")
        .short('f')
        .long("file-separator")
        .default_value(usv::FILE_SEPARATOR_SYMBOL_STR)
        .action(clap::ArgAction::Set))
    .arg(Arg::new("escape")
        .help("Set the escape string. Not currently used. Reserved for future use.")
        .long("escape")
        .default_value(usv::ESCAPE_SYMBOL_STR)
        .action(clap::ArgAction::Set))
    .arg(Arg::new("end-of-transmission")
        .help("Set the end-of-transmission string. Not currently used. Reserved for future use.")
        .long("end-of-transmission")
        .default_value(usv::END_OF_TRANSMISSION_SYMBOL_STR)
        .action(clap::ArgAction::Set))
    .arg(Arg::new("style-braces")
        .help(r#"Set the style to use braces, such as "{US}" for Unit Separator."#)
        .long("style-braces")
        .conflicts_with_all(["style-controls", "style-symbols", "style-liners", "style-sheets"])
        .action(clap::ArgAction::SetTrue))
    .arg(Arg::new("style-controls")
        .help(r#"Set the style to use controls, such as "\u{001F}" for Unit Separator."#)
        .long("style-controls")
        .conflicts_with_all(["style-braces", "style-symbols", "style-liners", "style-sheets"])
        .action(clap::ArgAction::SetTrue))
    .arg(Arg::new("style-symbols")
        .help(r#"Set the style to use symbols, such as "␟" for Unit Separator."#)
        .long("style-symbols")
        .conflicts_with_all(["style-braces", "style-controls", "style-liners", "style-sheets"])
        .action(clap::ArgAction::SetTrue))
    .arg(Arg::new("style-liners")
        .help(r#"Set the style to use liners wrapping every marker, such as "\n␟\n" for Unit Separator."#)
        .long("style-liners")
        .conflicts_with_all(["style-braces", "style-controls", "style-symbols", "style-sheets"])
        .action(clap::ArgAction::SetTrue))
    .arg(Arg::new("style-sheets")
        .help(r#"Set the style similar to spreadsheet sheets, such as "␟" for Unit Separator and "␟\n" for Record Separator."#)
        .long("style-sheets")
        .conflicts_with_all(["style-braces", "style-controls", "style-symbols", "style-liners"])
        .action(clap::ArgAction::SetTrue))
    .get_matches()
}

fn style(matches: &clap::ArgMatches) -> usv::style::Style {
    if matches.get_flag("style-braces") {
        return usv::style::Style::braces()
    }
    if matches.get_flag("style-controls") {
        return usv::style::Style::controls()
    }
    if matches.get_flag("style-symbols") {
        return usv::style::Style::symbols()
    }
    if matches.get_flag("style-liners") {
        return usv::style::Style::liners()
    }
    if matches.get_flag("style-sheets") {
        return usv::style::Style::sheets()
    }
    usv::style::Style {
        unit_separator: matches.get_one::<String>("unit-separator").expect("unit-separator").to_string(),
        record_separator: matches.get_one::<String>("record-separator").expect("record-separator").to_string(),
        group_separator: matches.get_one::<String>("group-separator").expect("group-separator").to_string(),
        file_separator: matches.get_one::<String>("file-separator").expect("file-separator").to_string(),
        escape: matches.get_one::<String>("escape").expect("escape").to_string(),
        end_of_transmission: matches.get_one::<String>("end-of-transmission").expect("end-of-transmission").to_string(),
    }
}
