use rust_text_file_utils::text::flatten;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_text() {
        let text = "This is a line.\nThis is another line.\nAnd another one.";
        let expected = "This is a line.\nThis is another line.\nAnd another one.";
        assert_eq!(flatten::flatten_text(text), expected);

        let text_with_no_period =
            "This is a line without period\nstill no period\nfinally a period.";
        let expected_with_no_period =
            "This is a line without period still no period finally a period.";
        assert_eq!(
            flatten::flatten_text(text_with_no_period),
            expected_with_no_period
        );

        let text_with_commas = "First part, \nsecond part,\nthird part.";
        let expected_with_commas = "First part, second part, third part.";
        assert_eq!(
            flatten::flatten_text(text_with_commas),
            expected_with_commas
        );

        let text_with_exclamations = "Wow!\nThis is amazing!\nIncredible!";
        let expected_with_exclamations = "Wow!\nThis is amazing!\nIncredible!";
        assert_eq!(
            flatten::flatten_text(text_with_exclamations),
            expected_with_exclamations
        );

        let text_with_questions = "Is this a test?\nYes, it is.\nAre we sure?";
        let expected_with_questions = "Is this a test?\nYes, it is.\nAre we sure?";
        assert_eq!(
            flatten::flatten_text(text_with_questions),
            expected_with_questions
        );

        let mixed_punctuation = "Start with a statement.\nThen a question?\nAn exclamation!";
        let expected_mixed_punctuation =
            "Start with a statement.\nThen a question?\nAn exclamation!";
        assert_eq!(
            flatten::flatten_text(mixed_punctuation),
            expected_mixed_punctuation
        );

        let complex_text = "First part,\nsecond part.\nThird part?\nYes, indeed!";
        let expected_complex_text = "First part, second part.\nThird part?\nYes, indeed!";
        assert_eq!(flatten::flatten_text(complex_text), expected_complex_text);
    }
}
