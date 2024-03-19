//! Command IO: Spawn a command, write stdin, read stdout.

use std::process::{Command, Stdio};
use std::io::Write;

pub fn command_io(command: &mut Command, stdin: &str) -> String {
    // Spawn child
    let mut child = command
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("command");

    // Handle stdin
    let child_stdin = child.stdin.as_mut().expect("child_stdin");
    child_stdin.write_all(stdin.as_bytes()).expect("write_all");
    #[allow(dropping_references)]
    let _ = drop(child_stdin);

    // Handle stdout
    let output = child.wait_with_output().expect("wait_with_output");
    String::from_utf8(output.stdout).unwrap()
}
