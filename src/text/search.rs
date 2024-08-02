use std::error::Error;

pub fn find(pattern: &str, text: &str) -> Result<Option<usize>, Box<dyn Error>> {
    if pattern.is_empty() {
        return Err("Pattern cannot be empty".into());
    }

    let result = text.find(pattern);
    Ok(result)
}
