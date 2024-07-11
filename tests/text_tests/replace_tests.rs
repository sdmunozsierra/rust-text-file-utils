use rust_text_file_utils::text::replace::replace;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_valid() {
        let test_cases = vec![
            ("world", "Rust", "Hello, world!", "Hello, Rust!"),
            ("foo", "bar", "foo bar foo", "bar bar bar"),
            ("", "nothing", "empty pattern", "Pattern cannot be empty"), // This should return an error
        ];

        for (pattern, replacement, text, expected) in test_cases {
            let result = replace(pattern, replacement, text);
            if pattern.is_empty() {
                assert!(result.is_err());
                assert_eq!(result.unwrap_err().to_string(), expected);
            } else {
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), expected);
            }
        }
    }

    #[test]
    fn test_replace_empty_pattern() {
        let result = replace("", "replacement", "some text");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Pattern cannot be empty");
    }

    #[test]
    fn test_replace_no_matches() {
        let result = replace("xyz", "replacement", "some text with no matches");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "some text with no matches");
    }

    #[test]
    fn test_replace_empty_replacement() {
        let result = replace("remove", "", "remove this part of the text");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), " this part of the text");
    }

    #[test]
    fn test_replace_empty_text() {
        let result = replace("pattern", "replacement", "");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }
}
