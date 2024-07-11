use std::error::Error;

pub fn replace(pattern: &str, replacement: &str, text: &str) -> Result<String, Box<dyn Error>> {
    if pattern.is_empty() {
        return Err("Pattern cannot be empty".into());
    }

    let result = text.replace(pattern, replacement);
    Ok(result)
}
