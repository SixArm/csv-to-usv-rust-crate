#[path = "testing.rs"]
mod testing;
use testing::*;

#[path = "command_io.rs"]
mod command_io;
use command_io::*;

use std::process::Command;

pub const CSV_EXAMPLE: &str = "a,b\nc,d\n";
pub const STYLE_BRACES_EXAMPLE: &str = "a{US}b{US}{RS}c{US}d{US}{RS}";
pub const STYLE_CONTROLS_EXAMPLE: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}";
pub const STYLE_SYMBOLS_EXAMPLE: &str = "a␟b␟␞c␟d␟␞";
pub const STYLE_LINERS_EXAMPLE: &str = "a\n␟\nb\n␟\n\n␞\nc\n␟\nd\n␟\n\n␞\n";
pub const STYLE_SHEETS_EXAMPLE: &str = "a␟b␟␞\nc␟d␟␞\n";

#[test]
fn command_with_style_braces() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-braces"), &CSV_EXAMPLE);
    assert_eq!(actual, STYLE_BRACES_EXAMPLE);
}

#[test]
fn command_with_style_controls() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-controls"), &CSV_EXAMPLE);
    assert_eq!(actual, STYLE_CONTROLS_EXAMPLE);
}

#[test]
fn command_with_style_symbols() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-symbols"), &CSV_EXAMPLE);
    assert_eq!(actual, STYLE_SYMBOLS_EXAMPLE);
}

#[test]
fn command_with_style_liners() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-liners"), &CSV_EXAMPLE);
    assert_eq!(actual, STYLE_LINERS_EXAMPLE);
}

#[test]
fn command_with_style_sheets() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-sheets"), &CSV_EXAMPLE);
    assert_eq!(actual, STYLE_SHEETS_EXAMPLE);
}
