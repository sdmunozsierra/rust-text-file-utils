use rust_text_file_utils::text::search::find;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_valid() {
        let test_cases = vec![
            ("world", "Hello, world!", Some(7)),
            ("foo", "foo bar foo", Some(0)),
            ("bar", "foo bar foo", Some(4)),
            ("baz", "foo bar foo", None),
        ];

        for (pattern, text, expected) in test_cases {
            let result = find(pattern, text);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }
    }

    #[test]
    fn test_find_empty_pattern() {
        let result = find("", "some text");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Pattern cannot be empty");
    }

    #[test]
    fn test_find_no_matches() {
        let result = find("xyz", "some text with no matches");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_find_empty_text() {
        let result = find("pattern", "");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_find_pattern_at_start() {
        let result = find("start", "start of the text");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(0));
    }

    #[test]
    fn test_find_pattern_at_end() {
        let result = find("end", "text at the end");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(12));
    }

    #[test]
    fn test_find_pattern_in_middle() {
        let result = find("middle", "text in the middle of the string");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(12));
    }
}
