#[cfg(feature = "cli")]
use clap::{Arg, ArgAction, Command};

#[cfg(feature = "cli")]
use crate::compound::srt_processing::process_directory;
use crate::file::merge::merge_files;
use crate::file::read::{read_file, read_files_sequentially};
use crate::file::unzip::unzip_file;
use crate::file::write::write_file;
use crate::text::clean::clean_title;
use crate::text::replace::replace;
use crate::text::search::find;

#[cfg(feature = "cli")]
#[tokio::main]
pub async fn run_cli() {
    let matches = Command::new("TextFileUtils CLI")
        .version("0.1.0")
        .author("Your Name <ss@sergio.com.ai>")
        .about("Provides text and file utilities")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("file")
                .about("File operations")
                .subcommand(
                    Command::new("merge")
                        .about("Merge multiple files")
                        .arg(
                            Arg::new("files")
                                .help("List of files to merge")
                                .long("files")
                                .num_args(1..)
                                .action(ArgAction::Set),
                        )
                        .arg(
                            Arg::new("output")
                                .help("Output file")
                                .long("output")
                                .action(ArgAction::Set),
                        ),
                )
                .subcommand(
                    Command::new("read")
                        .about("Read a file")
                        .arg(
                            Arg::new("file")
                                .help("File to read")
                                .long("file")
                                .conflicts_with("files_sequentially")
                                .action(ArgAction::Set),
                        )
                        .arg(
                            Arg::new("files_sequentially")
                                .help("Read files sequentially in a directory")
                                .long("dir")
                                .conflicts_with("file")
                                .action(ArgAction::Set),
                        )
                        .arg(
                            Arg::new("extension")
                                .help("File extension to read")
                                .long("extension")
                                .conflicts_with("file")
                                .action(ArgAction::Set),
                        ),
                )
                .subcommand(
                    Command::new("write")
                        .about("Write to a file")
                        .arg(
                            Arg::new("file")
                                .help("File to write to")
                                .long("file")
                                .action(ArgAction::Set),
                        )
                        .arg(
                            Arg::new("content")
                                .help("Content to write")
                                .long("content")
                                .action(ArgAction::Set),
                        ),
                )
                .subcommand(
                    Command::new("unzip")
                        .about("Unzip a file")
                        .arg(
                            Arg::new("file")
                                .help("File to unzip")
                                .long("file")
                                .action(ArgAction::Set),
                        )
                        .arg(
                            Arg::new("directory")
                                .help("Directory to unzip")
                                .long("directory")
                                .action(ArgAction::Set),
                        ),
                ),
        )
        .subcommand(
            Command::new("text")
                .about("Text operations")
                .subcommand(
                    Command::new("clean_title").about("Clean title").arg(
                        Arg::new("input")
                            .help("Title to clean")
                            .action(ArgAction::Set),
                    ),
                )
                .subcommand(
                    Command::new("replace")
                        .about("Replace text")
                        .arg(
                            Arg::new("input")
                                .help("Text to process")
                                .long("text")
                                .action(ArgAction::Set),
                        )
                        .arg(
                            Arg::new("search")
                                .help("Text to search for")
                                .long("search")
                                .action(ArgAction::Set),
                        )
                        .arg(
                            Arg::new("replace")
                                .help("Text to replace with")
                                .long("replace")
                                .action(ArgAction::Set),
                        ),
                )
                .subcommand(
                    Command::new("search")
                        .about("Search text")
                        .arg(
                            Arg::new("search")
                                .help("Text to search for")
                                .long("search")
                                .action(ArgAction::Set),
                        )
                        .arg(
                            Arg::new("text")
                                .help("Text to process")
                                .long("text")
                                .action(ArgAction::Set),
                        ),
                ),
        )
        .subcommand(
            Command::new("flatten_srt_directory")
                .about("Flatten a directory")
                .arg(
                    Arg::new("directory")
                        .help("Directory to flatten")
                        .action(ArgAction::Set),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("file", file_matches)) => match file_matches.subcommand() {
            Some(("merge", merge_matches)) => {
                let files: Vec<_> = merge_matches
                    .get_many::<String>("files")
                    .expect("contains_id")
                    .map(|s| s.as_str())
                    .collect();
                let output = merge_matches
                    .get_one::<String>("output")
                    .expect("contains_id")
                    .as_str();
                match merge_files(output, &files) {
                    Ok(_) => println!("Files merged successfully into {}", output),
                    Err(e) => eprintln!("Error merging files: {}", e),
                }
            }
            Some(("read", read_matches)) => {
                if read_matches.contains_id("file") {
                    let file = read_matches.get_one::<String>("file").unwrap();
                    println!("Reading file: {}", file);
                    match read_file(file) {
                        Ok(v) => println!("File read successfully:\n{}", v),
                        Err(e) => eprintln!("Error reading file: {}", e),
                    }
                }

                if read_matches.contains_id("files_sequentially") {
                    let directory = read_matches
                        .get_one::<String>("files_sequentially")
                        .unwrap();
                    let extension = read_matches
                        .get_one::<String>("extension")
                        .unwrap_or(&"".to_string())
                        .clone();
                    println!(
                        "Reading directory: {} with extension: {}",
                        directory, extension
                    );
                    match read_files_sequentially(directory, &extension) {
                        Ok(v) => println!("Read directory successfully:\n{:?}", v),
                        Err(e) => eprintln!("Error reading directory: {}", e),
                    }
                }
            }
            Some(("write", write_matches)) => {
                let file = write_matches.get_one::<String>("file").unwrap();
                let content = write_matches.get_one::<String>("content").unwrap();
                println!("Writing to file: {} with content: {}", file, content);
                match write_file(file, content) {
                    Ok(v) => println!("Write file successfully:\n{:?}", v),
                    Err(e) => eprintln!("Error writing file: {}", e),
                }
            }
            Some(("unzip", unzip_matches)) => {
                let file = unzip_matches.get_one::<String>("file").unwrap();
                let directory = unzip_matches.get_one::<String>("directory").unwrap();
                match unzip_file(file, directory).await {
                    Ok(_) => println!("Unzip successful!"),
                    Err(e) => eprintln!("Error unzipping file: {}", e),
                }
            }
            _ => eprintln!("Unknown file operation"),
        },
        Some(("text", text_matches)) => match text_matches.subcommand() {
            Some(("clean_title", clean_matches)) => {
                let input = clean_matches.get_one::<String>("input").unwrap();
                println!("Cleaning text: {}", input);
                match clean_title(input) {
                    Ok(v) => println!("{}", v),
                    Err(e) => eprintln!("Error cleaning title: {}", e),
                }
            }
            Some(("replace", replace_matches)) => {
                let input = replace_matches.get_one::<String>("input").unwrap();
                let search = replace_matches.get_one::<String>("search").unwrap();
                let pattern = replace_matches.get_one::<String>("replace").unwrap();
                println!(
                    "Replacing '{}' with '{}' in text: {}",
                    search, pattern, input
                );
                match replace(search, pattern, input) {
                    Ok(v) => println!("{}", v),
                    Err(e) => eprintln!("Error replacing: {}", e),
                }
            }
            Some(("search", search_matches)) => {
                let search = search_matches.get_one::<String>("search").unwrap();
                let text = search_matches.get_one::<String>("text").unwrap();
                println!("Searching '{}' in text: '{}'", search, text);
                match find(search, text) {
                    Ok(Some(index)) => println!("Pattern found at index: {}", index),
                    Ok(None) => println!("Pattern not found"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            _ => eprintln!("Unknown text operation"),
        },
        Some(("flatten_srt_directory", sub_m)) => {
            let directory = sub_m.get_one::<String>("directory").unwrap();
            process_directory(directory);
        }
        _ => eprintln!("Unknown command"),
    }
}
