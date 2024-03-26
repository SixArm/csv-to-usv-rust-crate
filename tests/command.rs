mod common; use common::*;
use csv_to_usv::examples::*;
use std::process::Command;

#[test]
fn command() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(&mut command, EXAMPLE_INPUT_RECORDS);
    assert_eq!(actual, usv::examples::EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_RECORDS);
}
