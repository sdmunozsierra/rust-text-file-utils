use rust_text_file_utils::file::write;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

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
}
