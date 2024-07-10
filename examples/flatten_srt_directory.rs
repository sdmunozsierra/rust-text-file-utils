use rust_text_file_utils::file::read::read_files_sequentially;
use rust_text_file_utils::parser::srt::{self, OutputFormat};
use rust_text_file_utils::text::clean::clean_title;
use rust_text_file_utils::text::flatten::flatten_text;

fn main() {
    // Specify the directory containing the SRT files
    let test_dir = "/home/adminlenovo/Data/Learning/Udacity/Introduction_to_LLMs/1_GenAI";

    // Read all SRT files in the specified directory
    let result = read_files_sequentially(test_dir, "srt");
    match result {
        Ok(contents) => {
            for (filename, content) in contents {
                let clean_filename = clean_title(filename.as_str());
                println!("## {}", clean_filename);
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
        }
        Err(e) => eprintln!("Failed to read files: {}", e),
    }
}
