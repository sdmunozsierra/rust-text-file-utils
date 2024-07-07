use anyhow::{Context, Result};
use regex::Regex;
use std::fs::{self, File};
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

pub fn read_files_sequentially(
    directory: &str,
    file_extension: &str,
) -> Result<Vec<(String, String)>> {
    let mut entries: Vec<_> = fs::read_dir(directory)
        .with_context(|| format!("Failed to read directory: {}", directory))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map_or(false, |ext| ext == file_extension)
        })
        .collect();

    let re = Regex::new(r"^(\d+)").unwrap();

    entries.sort_by(|a, b| {
        let a_name_binding = a.file_name();
        let a_name = a_name_binding.to_string_lossy();
        let b_name_binding = b.file_name();
        let b_name = b_name_binding.to_string_lossy();

        let a_captures = re.captures(&a_name);
        let b_captures = re.captures(&b_name);

        match (a_captures, b_captures) {
            (Some(a_cap), Some(b_cap)) => {
                let a_num: u32 = a_cap[1].parse().unwrap_or(0);
                let b_num: u32 = b_cap[1].parse().unwrap_or(0);
                a_num.cmp(&b_num)
            }
            _ => a_name.cmp(&b_name),
        }
    });

    let mut contents = Vec::new();
    for entry in entries {
        let path = entry.path();
        let path_str = path.to_string_lossy().to_string();
        let filename = entry.file_name().to_string_lossy().to_string();
        let content = read_file(&path_str)?;
        contents.push((filename, content));
    }
    Ok(contents)
}
