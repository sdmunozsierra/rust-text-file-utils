use crate::file::read::read_files_sequentially;
use crate::parser::srt::{self, OutputFormat};
use crate::text::clean::{TitleFormat, clean_title};
use crate::text::flatten::flatten_text;
use log::{debug, error};

use crate::config::logger;


pub fn process_directory(directory: &str, index_number: i32, name: &str) {
    logger::init_logging();

    let chapter: String = index_number.to_string();
    let mut file_number: i32 = 1;

    // Read all SRT files in the specified directory
    println!("# {}", name);
    match read_files_sequentially(directory, "srt") {
        Ok(contents) => {
            for (filename, content) in contents {
                match clean_title(filename.as_str(), TitleFormat::SmartIgnore3) {
                    Ok(clean_filename) => {
                        debug!("Processing file: {}", filename);
                        println!("## {}.{} {}", chapter, file_number, clean_filename);
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
                            Err(e) => error!("Failed to parse SRT content from '{}': {}", filename, e),
                        }
                    }
                    Err(e) => error!("Failed to clean filename '{}': {}", filename, e),
                }
            }
        }
        Err(e) => error!("Failed to read files from directory '{}': {}", directory, e),
    }
}
