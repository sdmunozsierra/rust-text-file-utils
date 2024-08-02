use regex::Regex;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CleanedTitle(String);

impl fmt::Display for CleanedTitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<&str> for CleanedTitle {
    fn eq(&self, other: &&str) -> bool {
        &self.0 == other
    }
}

pub fn clean_title(title: &str) -> Result<String, Box<dyn Error>> {
    // Define the regex pattern to capture the relevant part of the string
    let re = Regex::new(r"(?m)^\d+ - [^-]+ - (.+?) - lang_[a-zA-Z-]+\.srt$")?;

    // Use captures_iter to find matches and extract the relevant part
    if let Some(captures) = re.captures_iter(title).next() {
        if let Some(matched) = captures.get(1) {
            return Ok(matched.as_str().to_string());
        }
    }

    Err("Failed to clean title".into())
}
