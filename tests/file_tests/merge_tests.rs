use rust_text_file_utils::file::merge;
use std::fs::{self, File};
use std::io::Write;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_files() {
        let file1_path = "test_file1.txt";
        let file2_path = "test_file2.txt";
        let output_path = "merged_output.txt";

        let contents1 = "Hello, ";
        let contents2 = "world!";

        // Create test files
        let mut file1 = File::create(file1_path).expect("Unable to create test file 1");
        let mut file2 = File::create(file2_path).expect("Unable to create test file 2");
        file1
            .write_all(contents1.as_bytes())
            .expect("Unable to write to test file 1");
        file2
            .write_all(contents2.as_bytes())
            .expect("Unable to write to test file 2");

        // Merge files
        merge::merge_files(output_path, &[file1_path, file2_path]).expect("Unable to merge files");

        // Verify the output
        let merged_contents =
            fs::read_to_string(output_path).expect("Unable to read merged output file");
        assert_eq!(merged_contents, "Hello, world!");

        // Clean up
        fs::remove_file(file1_path).expect("Unable to delete test file 1");
        fs::remove_file(file2_path).expect("Unable to delete test file 2");
        fs::remove_file(output_path).expect("Unable to delete merged output file");
    }
}
