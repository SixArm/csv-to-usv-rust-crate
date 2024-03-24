mod common; use common::*;
use csv_to_usv::examples::*;
use std::process::Command;

#[test]
fn command_with_delimiter_default() {
    let input = "a,b\nc,d\n";
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(&mut command, &input);
    assert_eq!(actual, usv::examples::EXAMPLE_STYLE_SYMBOLS_RECORDS_AND_LAYOUT_RECORDS);
}

#[test]
fn command_with_delimiter_with_short_option() {
    let input = "a;b\nc;d\n";
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(&mut command.arg("-d").arg(";"), &input);
    assert_eq!(actual, usv::examples::EXAMPLE_STYLE_SYMBOLS_RECORDS_AND_LAYOUT_RECORDS);
}

#[test]
fn command_with_delimiter_with_long_option() {
    let input = "a;b\nc;d\n";
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(&mut command.arg("--delimiter").arg(";"), &input);
    assert_eq!(actual, usv::examples::EXAMPLE_STYLE_SYMBOLS_RECORDS_AND_LAYOUT_RECORDS);
}
