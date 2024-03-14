#[path = "testing.rs"]
mod testing;
use testing::*;

use std::process::{Command, Stdio};
use std::io::Write;

// use std::path::PathBuf;
// use once_cell::sync::Lazy;
//
// pub static DIR: Lazy<PathBuf> = Lazy::new(||
//     TESTS_DIR.join("custom_separators")
// );

#[test]
fn custom_delimiter_with_short_arg() {
    let input = "a;b;c\nd;e;f\ng;h;i\n";
    let expect = "a␟b␟c␟␞\nd␟e␟f␟␞\ng␟h␟i␟␞\n";

    // Given
    let mut command = Command::new(&*COMMAND_OS)
    .arg("-d")
    .arg(";")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("command");

    // When
    let child_stdin = command.stdin.as_mut().expect("child_stdin");
    child_stdin.write_all(input.as_bytes()).expect("write_all");
    #[allow(dropping_references)]
    let _ = drop(child_stdin);

    // Then
    let output = command.wait_with_output().expect("wait_with_output");
    let actual: String = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, expect);
}

#[test]
fn custom_delimiter_with_long_arg() {
    let input = "a;b;c\nd;e;f\ng;h;i\n";
    let expect = "a␟b␟c␟␞\nd␟e␟f␟␞\ng␟h␟i␟␞\n";

    // Given
    let mut command = Command::new(&*COMMAND_OS)
    .arg("--delimiter")
    .arg(";")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("command");

    // When
    let child_stdin = command.stdin.as_mut().expect("child_stdin");
    child_stdin.write_all(input.as_bytes()).expect("write_all");
    #[allow(dropping_references)]
    let _ = drop(child_stdin);

    // Then
    let output = command.wait_with_output().expect("wait_with_output");
    let actual: String = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, expect);
}
