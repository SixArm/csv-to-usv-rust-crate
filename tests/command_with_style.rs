mod common; use common::*;
use csv_to_usv::examples::*;
use std::process::Command;

#[test]
fn command_with_style_symbols() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command.arg("--style-symbols"), EXAMPLE_INPUT_RECORDS);
    assert_eq!(actual, usv::examples::EXAMPLE_STYLE_SYMBOLS_RECORDS_AND_LAYOUT_RECORDS);
}

#[test]
fn command_with_style_controls() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command.arg("--style-controls"), EXAMPLE_INPUT_RECORDS);
    assert_eq!(actual, usv::examples::EXAMPLE_STYLE_CONTROLS_RECORDS_AND_LAYOUT_RECORDS);
}

#[test]
fn command_with_style_braces() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command.arg("--style-braces"), EXAMPLE_INPUT_RECORDS);
    assert_eq!(actual, usv::examples::EXAMPLE_STYLE_BRACES_RECORDS_AND_LAYOUT_RECORDS);
}
