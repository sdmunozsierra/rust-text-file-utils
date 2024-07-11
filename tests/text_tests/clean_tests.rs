use rust_text_file_utils::text::clean;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_title() {
        let test_cases = vec![
            (
                "123 - CD456 GenAI C789 L012 A345. Something V1 - lang_en-us.srt",
                "Something",
            ),
            (
                "123 - CD456 GenAI C789 L012 A345. Something else V1 - lan.srt",
                "Something else",
            ),
            (
                "123 - CD456 GenAI C789 L012 A345. Random text",
                "Random text",
            ),
            ("Just some normal title", "Just some normal title"),
            ("V1 - lang_en-us.srt", "V1 - lang_en-us.srt"),
            ("V1 - lan.srt", "V1 - lan.srt"),
        ];

        for (input, expected) in test_cases {
            match clean::clean_title(input) {
                Ok(cleaned) => assert_eq!(cleaned, expected, "Failed for input: {}", input),
                Err(e) => panic!("Error cleaning title for input: {}: {}", input, e),
            }
        }
    }
}
