use csv_to_usv::*;
mod common;

#[test]
fn simple() {
    let csv = String::from("ab,cd,ef\ngh,ij,kl\n");
    let usv = String::from("ab␟cd␟ef␟␞gh␟ij␟kl␟␞");
    assert_eq!(csv_to_usv(&csv), usv);
}

#[test]
fn quotes() {
    let csv = String::from("\"ab\"\"cd\"\"ef\"\n");
    let usv = String::from("ab\"cd\"ef␟␞");
    assert_eq!(csv_to_usv(&csv), usv);
}

#[test]
fn commas() {
    let csv = String::from("\"ab,cd,ef\"\n");
    let usv = String::from("ab,cd,ef␟␞");
    assert_eq!(csv_to_usv(&csv), usv);
}
