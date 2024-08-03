use regex::Regex;
use std::error::Error;
use std::fmt;
use log::{warn, error, debug};

use crate::config::logger;

// Initialize logging for the entire module
#[ctor::ctor]
fn init() {
    logger::init_logging();
}


// Define a custom error type for title cleaning
#[derive(Debug)]
pub struct TitleError(String);

impl fmt::Display for TitleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for TitleError {}

// Define an enum for different title formats
#[derive(Debug)]
pub enum TitleFormat {
    Default,
    Simple,
    Detailed,
    CdFormat,
    Smart,
    SmartIgnore3,
    SmartIgnore4,
}

impl TitleFormat {
    // Retrieve regex pattern based on title format
    pub fn get_regex(&self) -> &'static str {
        match self {
            TitleFormat::Default => r"(?m)^\d+ - [^-]+ - (.+?) - lang_[a-zA-Z-]+\.srt$",
            TitleFormat::Simple => r"(?m)^\d+ - (.+?) - lang_[a-zA-Z-]+\.srt$",
            TitleFormat::Detailed => r"(?m)^\d+ - [^-]+ - (.+?) - [^-]+ - lang_[a-zA-Z-]+\.srt$",
            TitleFormat::CdFormat  => r"(?m)^\d+ - Cd\d+ [A-Z]\d+ (.+? V\d+)-? - lang_[a-zA-Z-]+\.srt$",
            TitleFormat::Smart => r"(?m)^\d+ - [^ ]+ [^ ]+ (.+?) (?:V\d+|Pt \d+)",
            TitleFormat::SmartIgnore3 => r"(?m)^\d+ - [^ ]+ [^ ]+ [^ ]+ (.+?)-",
            TitleFormat::SmartIgnore4 => r"(?m)^\d+ - [^ ]+ [^ ]+ [^ ]+ [^ ]+ (.+?)-",
        }
    }
}

// Define a struct for cleaned titles
#[derive(Debug)]
pub struct CleanedTitle(String);

impl CleanedTitle {
    // Constructor for creating a new CleanedTitle
    pub fn new(title: &str, format: TitleFormat) -> Result<Self, Box<dyn Error>> {
        // Get the appropriate regex based on the title format
        let pattern = format.get_regex();
        debug!("Using regex pattern: {}", pattern);
        
        let re = Regex::new(pattern)?;
        
        debug!("Attempting to match title: '{}'", title);

        // Attempt to capture the relevant part of the title
        if let Some(captures) = re.captures_iter(title).next() {
            debug!("Regex matched: {:?}", captures);

            if let Some(matched) = captures.get(1) {
                debug!("Matched group: '{}'", matched.as_str());
                return Ok(CleanedTitle(matched.as_str().to_string()));
            } else {
                warn!("Regex match found, but group capture failed.");
            }
        } else {
            warn!("Regex did not match the title.");
        }

        // Return an error if cleaning fails
        let error_message = format!(
            "Failed to clean title using pattern '{}': '{}'",
            pattern, title
        );
        error!("{}", error_message);

        Err(Box::new(TitleError(error_message)))
    }

    // Convert cleaned title to string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

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

// Function to clean title using a specified format
pub fn clean_title(title: &str, format: TitleFormat) -> Result<CleanedTitle, Box<dyn Error>> {
    CleanedTitle::new(title, format)
}