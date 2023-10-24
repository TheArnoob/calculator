pub fn split_plus_minus(input: &str) -> Vec<String> {
    let delimiters = &['+', '-'];

    let mut terms = Vec::new();

    let mut term = String::new();
    for c in input.chars() {
        if delimiters.contains(&c) {
            terms.push(term.clone());
            term.clear();
            term.push(c);
        } else {
            term.push(c);
        }
    }

    terms.push(term);

    terms
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_split() {
        let input = "1+2-3";
        let expected = vec!["1".to_string(), "+2".to_string(), "-3".to_string()];
        let actual = split_plus_minus(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn split_with_decimal_points() {
        let input = "1.0+2-3.5";
        let expected = vec!["1.0".to_string(), "+2".to_string(), "-3.5".to_string()];
        let actual = split_plus_minus(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn split_empty_string() {
        let input = "";
        let expected = vec!["".to_string()];
        let actual = split_plus_minus(input);
        assert_eq!(expected, actual);
    }
}
