use regex::Regex;

pub fn clean_title(title: &str) -> String {
    // Define the regex pattern to match the unwanted parts of the string
    let re = Regex::new(
        r"^\d+ - CD\d+ GenAI C\d+ L\d+ A\d+ | V\d+ - lang_en-us\.srt$| V\d+ - lan\.srt$",
    )
    .unwrap();
    // Replace the unwanted parts with an empty string
    let cleaned_title = re.replace_all(title, "").to_string();
    cleaned_title
}
