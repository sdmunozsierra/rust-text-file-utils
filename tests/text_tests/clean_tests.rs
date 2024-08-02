use rust_text_file_utils::text::clean;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_title() {
        let test_cases = vec![
            (
                "4 - CD13317 GenAI C2 L1 A04 Encoder Vs Decoder Models V1 - lang_en-us.srt",
                "Encoder Vs Decoder Models V1",
            ),
            (
                "5 - NLP Fundamentals A05 Challenges In NLP V02 - lang_en-us.srt",
                "Challenges In NLP V02",
            ),
            (
                "4 - GEN AI C2 L3 A04 Defining Attention V2 - lang_en-us.srt",
                "Defining Attention V2",
            ),
            (
                "12 - CD12792 C1 L4 A01 Composing A Text Prompt - lang_en-us.srt",
                "Composing A Text Prompt",
            ),
        ];

        for (input, expected) in test_cases {
            match clean::clean_title(input) {
                Ok(cleaned) => assert_eq!(cleaned, expected, "Failed for input: {}", input),
                Err(e) => panic!("Error cleaning title for input: {}: {}", input, e),
            }
        }
    }
}
