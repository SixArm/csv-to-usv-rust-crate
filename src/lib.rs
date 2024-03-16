pub fn csv_to_usv<
    S: AsRef<str> + Sized
>(
    csv: S
) -> String {
    csv_to_usv_with_delimiter(csv, b',')
}

pub fn csv_to_usv_with_delimiter<
    S: AsRef<str> + Sized
>(
    csv: S,
    delimiter: u8,
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
                s += "␟";
            }
            s += "␞\n";
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let csv = String::from("ab,cd,ef\ngh,ij,kl\n");
        let usv = String::from("ab␟cd␟ef␟␞\ngh␟ij␟kl␟␞\n");
        assert_eq!(csv_to_usv(&csv), usv);
    }

    #[test]
    fn quotes() {
        let csv = String::from("\"ab\"\"cd\"\"ef\"\n");
        let usv = String::from("ab\"cd\"ef␟␞\n");
        assert_eq!(csv_to_usv(&csv), usv);
    }

    #[test]
    fn commas() {
        let csv = String::from("\"ab,cd,ef\"\n");
        let usv = String::from("ab,cd,ef␟␞\n");
        assert_eq!(csv_to_usv(&csv), usv);
    }

}
