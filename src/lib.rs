pub fn csv_to_usv(csv: &String) -> String {
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
