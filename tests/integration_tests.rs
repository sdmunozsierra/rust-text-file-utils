use rust_text_file_utils::file::{merge, read, write};
use rust_text_file_utils::text::{flatten, replace, search};
use std::fs::{self, File};
use std::io::Write;

#[test]
fn test_search() {
    let result = search::find("world", "Hello, world!");
    assert_eq!(result, Some(7));
}

#[test]
fn test_replace() {
    let result = replace::replace("world", "Rust", "Hello, world!");
    assert_eq!(result, "Hello, Rust!");
}

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

#[test]
fn test_read_file_success() {
    // Create a test file with some content
    let test_path = "test_success.txt";
    let mut file = File::create(test_path).unwrap();
    writeln!(file, "This is a test file.").unwrap();

    let result = read::read_file(test_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "This is a test file.\n");

    // Clean up
    std::fs::remove_file(test_path).unwrap();
}

#[test]
fn test_read_file_not_found() {
    let result = read::read_file("non_existent_file.txt");
    assert!(result.is_err());
    let error = result.unwrap_err();
    println!("Error: {:?}", error);
}

#[test]
fn test_read_file_read_error() {
    // Assuming there's a file that exists but has read permission denied
    let result = read::read_file("/root/protected_file.txt");
    assert!(result.is_err());
    let error = result.unwrap_err();
    println!("Error: {:?}", error);
}

#[test]
fn test_write_file() {
    let file_path = "test_write.txt";
    let contents = "Hello, world!";

    // Write to file
    let result = write::write_file(file_path, contents);
    assert!(
        result.is_ok(),
        "Failed to write to file: {:?}",
        result.err()
    );

    // Verify the contents
    let written_contents = fs::read_to_string(file_path).expect("Unable to read test file");
    assert_eq!(written_contents, contents);

    // Clean up
    fs::remove_file(file_path).expect("Unable to delete test file");
}

#[test]
fn test_write_file_fail() {
    let invalid_file_path = "/invalid_path/test_write.txt";
    let contents = "Hello, world!";

    // Attempt to write to an invalid path
    let result = write::write_file(invalid_file_path, contents);
    assert!(result.is_err(), "Expected an error but got Ok");

    // Verify the error message
    if let Err(e) = result {
        assert!(
            e.to_string().contains("Failed to create file"),
            "Unexpected error message: {}",
            e
        );
    }
}

#[test]
fn test_flatten_text() {
    let text = "This is a line.\nThis is another line.\nAnd another one.";
    let expected = "This is a line. This is another line. And another one.";
    assert_eq!(flatten::flatten_text(text), expected);

    let text_with_no_period = "This is a line without period\nstill no period\nfinally a period.";
    let expected_with_no_period = "This is a line without period still no period finally a period.";
    assert_eq!(
        flatten::flatten_text(text_with_no_period),
        expected_with_no_period
    );
}
