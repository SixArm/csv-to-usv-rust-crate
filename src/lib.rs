pub mod examples;

use usv::style::Style;

pub fn csv_to_usv<
    S: AsRef<str> + Sized
>(
    csv: S,
    style: &Style,
) -> String {
    csv_to_usv_with_delimiter(csv, b',', style)
}

pub fn csv_to_usv_with_delimiter<
    S: AsRef<str> + Sized
>(
    csv: S,
    delimiter: u8,
    style: &Style,
) -> String {
    let mut s = String::new();
    let mut reader = csv::ReaderBuilder::new()
    .has_headers(false)
    .delimiter(delimiter)
    .from_reader(csv.as_ref().as_bytes());
    for result in reader.records() {
        if let Ok(record) = result {
            for unit in record.iter() {
                s += &unit;
                s += &style.unit_separator;
            }
            s += &style.record_separator;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use usv::style::Style;

    #[test]
    fn simple() {
        let csv = String::from("ab,cd,ef\ngh,ij,kl\n");
        let usv = String::from("ab␟cd␟ef␟␞gh␟ij␟kl␟␞");
        assert_eq!(csv_to_usv(&csv, &Style::default()), usv);
    }

    #[test]
    fn quotes() {
        let csv = String::from("\"ab\"\"cd\"\"ef\"\n");
        let usv = String::from("ab\"cd\"ef␟␞");
        assert_eq!(csv_to_usv(&csv, &Style::default()), usv);
    }

    #[test]
    fn commas() {
        let csv = String::from("\"ab,cd,ef\"\n");
        let usv = String::from("ab,cd,ef␟␞");
        assert_eq!(csv_to_usv(&csv, &Style::default()), usv);
    }

}
