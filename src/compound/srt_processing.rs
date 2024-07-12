use crate::file::read::read_files_sequentially;
use crate::parser::srt::{self, OutputFormat};
use crate::text::clean::clean_title;
use crate::text::flatten::flatten_text;

pub fn process_directory(directory: &str) {
    // Read all SRT files in the specified directory
    let result = read_files_sequentially(directory, "srt");
    match result {
        Ok(contents) => {
            let mut file_number = 1;
            for (filename, content) in contents {
                match clean_title(filename.as_str()) {
                    Ok(clean_filename) => {
                        println!("## 4.{} {}", file_number, clean_filename);
                        // Increment the file number
                        file_number += 1;
                        // Parse each SRT file content
                        match srt::parse_srt(&content, OutputFormat::TextOnly) {
                            Ok(subtitles) => {
                                // Collect text from parsed subtitles and convert to &[&str]
                                let subtitle_texts: Vec<&str> =
                                    subtitles.iter().map(|s| s.text.as_str()).collect();
                                // Flatten the text
                                let flatten = flatten_text(&subtitle_texts.join("\n"));
                                println!("{}\n", flatten);
                            }
                            Err(e) => eprintln!("Failed to parse SRT content: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Failed to clean filename: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Failed to read files: {}", e),
    }
}
