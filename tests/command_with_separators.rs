mod common; use common::*;
use csv_to_usv::examples::*;
use std::process::Command;

#[test]
fn command_with_separators_with_short_options() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command
        .arg("-u").arg("{US}")
        .arg("-r").arg("{RS}")
        .arg("-g").arg("{GS}")
        .arg("-f").arg("{FS}")
        .arg("-e").arg("{ESC}")
        .arg("-z").arg("{EOT}")
        , EXAMPLE_INPUT_RECORDS
    );
    assert_eq!(actual, usv::examples::EXAMPLE_RECORDS_STYLE_BRACES);
}

#[test]
fn command_with_separators_with_long_options() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command
        .arg("--unit-separator").arg("{US}")
        .arg("--record-separator").arg("{RS}")
        .arg("--group-separator").arg("{GS}")
        .arg("--file-separator").arg("{FS}")
        .arg("--escape").arg("{ESC}")
        .arg("--end-of-transmission").arg("{EOT}")
        , EXAMPLE_INPUT_RECORDS
    );
    assert_eq!(actual, usv::examples::EXAMPLE_RECORDS_STYLE_BRACES);
}
