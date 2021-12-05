use std::fs;

pub fn parse(file: &str) -> Vec<String> {
    fs::read_to_string(file)
        .unwrap_or(String::from("Error!"))
        .lines()
        .map(|x| x.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_line() {
        assert_eq!(parse("test.txt")[0], "199");
    }

    #[test]
    fn entire_file() {
        assert_eq!(parse("test.txt"), vec!("199", "200", "208", "210", "200", "207", "240", "269", "260", "263"));
    }
}
