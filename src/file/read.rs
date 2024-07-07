use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, Read};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileReadError {
    #[error("Failed to open the file at path: {0}")]
    OpenError(String, #[source] io::Error),
    #[error("Failed to read the file at path: {0}")]
    ReadError(String, #[source] io::Error),
}

pub fn read_file(path: &str) -> Result<String> {
    let mut file =
        File::open(path).with_context(|| format!("Failed to open the file at path: {}", path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .with_context(|| format!("Failed to read the file at path: {}", path))?;
    Ok(contents)
}
