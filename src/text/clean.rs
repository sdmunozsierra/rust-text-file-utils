use regex::Regex;
use std::error::Error;

pub fn clean_title(title: &str) -> Result<String, Box<dyn Error>> {
    // Define the regex pattern to match the unwanted parts of the string
    let pattern =
        r"^\d+ - CD\d+ GenAI C\d+ L\d+ A\d+.\A? | V\d+ - lang_en-us\.srt$| V\d+ - lan\.srt$";
    let re = Regex::new(pattern)?;

    // Replace the unwanted parts with an empty string
    let cleaned_title = re.replace_all(title, "").to_string();
    Ok(cleaned_title)
}
