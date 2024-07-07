use rust_text_file_utils::file::read;
use rust_text_file_utils::parser::srt::{self, OutputFormat};

fn main() {
    let test_path = "/home/adminlenovo/Data/Documents/udacity_lesson_1/1 - CD13317 GenAI C2 L1 A01 Meet Your Instructor V1 - lang_en-us.srt";

    // Read the file content
    let result = read::read_file(test_path);

    match result {
        Ok(content) => {
            // Parse the SRT content with text only format
            match srt::parse_srt(&content, OutputFormat::TextOnly) {
                Ok(subtitles) => {
                    // Print only the text content of the subtitles
                    for subtitle in subtitles {
                        println!("{}", subtitle.text);
                    }
                }
                Err(e) => eprintln!("Error parsing SRT file: {}", e),
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
