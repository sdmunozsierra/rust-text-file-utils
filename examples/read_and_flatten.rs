use rust_text_file_utils::file::read;
use rust_text_file_utils::text::flatten;

fn main() {
    let test_path = "/home/adminlenovo/Data/Programming/rust-srt-extractor/concatenated_text.txt";
    let result = read::read_file(test_path);
    let flatten = flatten::flatten_text(result.unwrap().as_str());
    println!("{}", flatten);
}
