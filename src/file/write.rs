use std::fs::File;
use std::io::{self, Write};

pub fn write_file(path: &str, contents: &str) -> io::Result<()> {
    let mut file = File::create(path).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to create file '{}': {}", path, e))
    })?;
    file.write_all(contents.as_bytes()).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to write to file '{}': {}", path, e),
        )
    })?;
    Ok(())
}
