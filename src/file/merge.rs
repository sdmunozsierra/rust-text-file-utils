use std::fs::File;
use std::io::{self, Read, Write};

pub fn merge_files(output: &str, inputs: &[&str]) -> io::Result<()> {
    let mut output_file = File::create(output)?;
    for input in inputs {
        let mut input_file = File::open(input)?;
        let mut buffer = Vec::new();
        input_file.read_to_end(&mut buffer)?;
        output_file.write_all(&buffer);
    }
    Ok(())
}
