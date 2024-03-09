//! # csv-to-usv
//!
//! Convert Comma Separated Values (CSV) to [Unicode Separated Values (USV)]([Unicode Separated Values (USV)](https://github.com/sixarm/usv).
//!
//! Syntax:
//!
//! ```sh
//! stdin | csv-to-usv | stdout
//! ```
//!
//! Example:
//!
//! ```sh
//! cat example.csv | csv-to-usv > example.usv
//! ```
//!
//! ## Install
//! 
//! Install:
//!
//! ```sh
//! cargo install csv-to-usv
//! ```
//! Link: [https://crates.io/crates/csv-to-usv](https://crates.io/crates/csv-to-usv)
//!
//! ## Example
//!
//! Suppose file example.csv contains:
//!
//! ```csv
//! a,b,c
//! d,e,f
//! g,h,i
//! ```
//!
//! Run:
//!
//! ```sh
//! cat example.csv | csv-to-usv
//! ```
//!
//! Output:
//!
//! ```usv
//! a␟b␟c␟␞d␟e␟f␟␞g␟h␟i␟␞
//! ```
//!

use std::io::{Read, stdin}; 
//use usv::*;

fn csv_to_usv(csv: &String) -> String {
    let mut s = String::new();
    let mut reader = csv::ReaderBuilder::new()
    .has_headers(false)
    .from_reader(csv.as_bytes());
    for result in reader.records() {
        if let Ok(record) = result {
            for unit in record.iter() {
                s += &format!("{}␟", unit);
            }
            s += "␞";
        }
    }
    s
}

fn main() -> std::io::Result<()> {
    let mut stdin = stdin().lock(); 
    let mut s = String::new();
    stdin.read_to_string(&mut s)?;
    println!("{}", csv_to_usv(&s));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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

}