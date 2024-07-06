use std::fs::File;
use std::io::{self, Write};

pub fn write_file(path: &str, contents: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())
}
