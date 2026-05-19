use regex::Regex;

pub fn is_match(pattern: &str, text: &str) -> bool {
    if let Ok(re) = Regex::new(pattern) {
        re.is_match(text)
    } else {
        false
    }
}

pub fn replace_all(pattern: &str, text: &str, replacement: &str) -> String {
    if let Ok(re) = Regex::new(pattern) {
        re.replace_all(text, replacement).into_owned()
    } else {
        text.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        assert!(is_match(r"^\d+$", "12345"));
        assert!(!is_match(r"^\d+$", "123a5"));
        assert_eq!(replace_all(r"foo", "foo bar foo", "baz"), "baz bar baz");
    }
}
