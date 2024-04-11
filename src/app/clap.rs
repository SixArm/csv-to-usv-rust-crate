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
        .help("Set the CSV field delimiter character.")
        .short('d')
        .long("delimiter")
        .default_value(",")
        .value_parser(clap::value_parser!(char))
        .action(clap::ArgAction::Set))
        .arg(Arg::new("unit-separator")
        .help("Set the unit separator (US) string")
        .short('u')
        .long("unit-separator")
        .action(clap::ArgAction::Set))
    .arg(Arg::new("record-separator")
        .help("Set the record separator (RS) string.")
        .short('r')
        .long("record-separator")
        .action(clap::ArgAction::Set))
    .arg(Arg::new("group-separator")
        .help("Set the group separator (GS) string.")
        .short('g')
        .long("group-separator")
        .action(clap::ArgAction::Set))
    .arg(Arg::new("file-separator")
        .help("Set the file separator (FS) string.")
        .short('f')
        .long("file-separator")
        .action(clap::ArgAction::Set))
    .arg(Arg::new("escape")
        .help("Set the escape (ESC) string.")
        .short('e')
        .long("escape")
        .action(clap::ArgAction::Set))
    .arg(Arg::new("end-of-transmission")
        .help("Set the end of transmission (EOT) string.")
        .short('z')
        .long("end-of-transmission")
        .action(clap::ArgAction::Set))
    .arg(Arg::new("style-symbols")
        .help(r#"Show marks as symbols, such as "␟" for Unit Separator."#)
        .long("style-symbols")
        .conflicts_with_all([
            "style-braces", 
            "style-controls"
        ])
        .action(clap::ArgAction::SetTrue))
    .arg(Arg::new("style-controls")
        .help(r#"Show marks as controls, such as "\u001F" for Unit Separator. This is most like ASCII Separated Values (ASV)."#)
        .long("style-controls")
        .conflicts_with_all([
            "style-braces", 
            "style-symbols",
        ])
        .action(clap::ArgAction::SetTrue))
    .arg(Arg::new("style-braces")
        .help(r#"Show marks as braces, such as "{US}" for Unit Separator. This is to help plain text readers, and is not USV output."#)
        .long("style-braces")
        .conflicts_with_all([
            "style-controls", 
            "style-symbols",
        ])
        .action(clap::ArgAction::SetTrue))
    .arg(Arg::new("layout-units")
        .help(r#"Show each unit on one line. This can be helpful for line-oriented tools."#)
        .long("layout-units")
        .conflicts_with_all([
            "layout-records",
            "layout-groups",
            "layout-files",
            "layout-0",
            "layout-1",
            "layout-2",
            ])
        .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("layout-records")
        .help(r#"Show each record on one line. This is like a typical spreadsheet sheet export."#)
        .long("layout-records")
        .conflicts_with_all([
            "layout-units", 
            "layout-groups",
            "layout-files",
            "layout-0",
            "layout-1",
            "layout-2",
            ])
        .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("layout-groups")
        .help(r#"Show each group on one line. This can be helpful for folio-oriented tools."#)
        .long("layout-groups")
        .conflicts_with_all([
            "layout-units", 
            "layout-records",
            "layout-files",
            "layout-0",
            "layout-1",
            "layout-2",
            ])
        .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("layout-files")
        .help(r#"Show each file on one line. This can be helpful for archive-oriented tools."#)
        .long("layout-files")
        .conflicts_with_all([
            "layout-units", 
            "layout-records",
            "layout-groups",
            "layout-0",
            "layout-1",
            "layout-2",
            ])
        .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("layout-0")
        .help(r#"Show each item with no line around it. This is no layout, in other words one long line."#)
        .long("layout-0")
        .conflicts_with_all([
            "layout-units", 
            "layout-records",
            "layout-groups",
            "layout-files",
            "layout-1",
            "layout-2",
            ])
        .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("layout-1")
        .help(r#"Show each item with one line around it. This is like single-space lines for long form text."#)
        .long("layout-1")
        .conflicts_with_all([
            "layout-units", 
            "layout-records",
            "layout-groups",
            "layout-files",
            "layout-0",
            "layout-2",
            ])
        .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("layout-2")
        .help(r#"Show each item with two lines around it. This is like double-space lines for long form text."#)
        .long("layout-2")
        .conflicts_with_all([
            "layout-units", 
            "layout-records",
            "layout-groups",
            "layout-files",
            "layout-0",
            "layout-1",
            ])
        .action(clap::ArgAction::SetTrue))
    .get_matches()
}

fn style(matches: &clap::ArgMatches) -> usv::style::Style {
    let style = if matches.get_flag("style-symbols") {
        usv::style::style_symbols()
    } else
    if matches.get_flag("style-controls") {
        usv::style::style_controls()
    } else
    if matches.get_flag("style-braces") {
        usv::style::style_braces()
    } else {
        usv::style::style_symbols()
    };
    let mut style = layout(matches).map_style(&style);
    if let Some(x) = matches.get_one::<String>("unit-separator") {
        style.unit_separator = String::from(x)
    }
    if let Some(x) = matches.get_one::<String>("record-separator") {
        style.record_separator = String::from(x)
    }
    if let Some(x) = matches.get_one::<String>("group-separator") {
        style.group_separator = String::from(x)
    }
    if let Some(x) = matches.get_one::<String>("file-separator") {
        style.file_separator = String::from(x)
    }
    if let Some(x) = matches.get_one::<String>("escape") {
        style.escape = String::from(x)
    }
    if let Some(x) = matches.get_one::<String>("end-of-transmission") {
        style.end_of_transmission = String::from(x)
    }
    style
}

fn layout(matches: &clap::ArgMatches) -> Box<dyn usv::style::MapStyle> {
    if matches.get_flag("layout-0") {
        Box::new(usv::layout::layout_0::Layout0)
    } else
    if matches.get_flag("layout-1") {
        Box::new(usv::layout::layout_1::Layout1)
    } else
    if matches.get_flag("layout-2") {
        Box::new(usv::layout::layout_2::Layout2)
    } else
    if matches.get_flag("layout-units") {
        Box::new(usv::layout::layout_units::LayoutUnits)
    } else
    if matches.get_flag("layout-records") {
        Box::new(usv::layout::layout_records::LayoutRecords)
    } else
    if matches.get_flag("layout-groups") {
        Box::new(usv::layout::layout_groups::LayoutGroups)
    } else
    if matches.get_flag("layout-files") {
        Box::new(usv::layout::layout_files::LayoutFiles)
    } else {
        Box::new(usv::layout::layout_records::LayoutRecords)
    }
}
