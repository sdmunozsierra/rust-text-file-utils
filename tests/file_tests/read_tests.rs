use rust_text_file_utils::file::read;
use std::fs::File;
use std::io::Write;

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_read_files_sequentially() {
        // Create a test directory with some test files
        let test_dir = "test_dir";
        std::fs::create_dir(test_dir).unwrap();

        let file_names = vec!["1_first.txt", "2_second.txt", "10_third.txt"];
        let file_contents = vec!["First file.", "Second file.", "Third file."];

        for (name, content) in file_names.iter().zip(&file_contents) {
            let mut file = File::create(format!("{}/{}", test_dir, name)).unwrap();
            writeln!(file, "{}", content).unwrap();
        }

        let result = read::read_files_sequentially(test_dir, "txt");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result,
            vec![
                ("1_first.txt".to_string(), "First file.\n".to_string()),
                ("2_second.txt".to_string(), "Second file.\n".to_string()),
                ("10_third.txt".to_string(), "Third file.\n".to_string())
            ]
        );

        // Clean up
        for name in file_names {
            std::fs::remove_file(format!("{}/{}", test_dir, name)).unwrap();
        }
        std::fs::remove_dir(test_dir).unwrap();
    }
}
