#[path = "testing.rs"]
mod testing;
use testing::*;

#[path = "command_io.rs"]
mod command_io;
use command_io::*;

use std::process::Command;

#[test]
fn delimiter_default() {
    let input = "a,b,c\nd,e,f\ng,h,i\n";
    let expect = "a␟b␟c␟␞d␟e␟f␟␞g␟h␟i␟␞";
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(&mut command, &input);
    assert_eq!(actual, expect);
}

#[test]
fn delimiter_with_short_arg() {
    let input = "a;b;c\nd;e;f\ng;h;i\n";
    let expect = "a␟b␟c␟␞d␟e␟f␟␞g␟h␟i␟␞";
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(&mut command.arg("-d").arg(";"), &input);
    assert_eq!(actual, expect);
}

#[test]
fn delimiter_with_long_arg() {
    let input = "a;b;c\nd;e;f\ng;h;i\n";
    let expect = "a␟b␟c␟␞d␟e␟f␟␞g␟h␟i␟␞";
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(&mut command.arg("--delimiter").arg(";"), &input);
    assert_eq!(actual, expect);
}
