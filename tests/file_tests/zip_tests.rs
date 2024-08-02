use rust_text_file_utils::file::zip;
use rust_text_file_utils::file::unzip::unzip_file;
use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::Path;
use tokio::runtime::Runtime;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zip_files() {
        let rt = Runtime::new().unwrap();

        // Create sample files for testing
        let file_paths = vec!["test1.txt", "test2.txt"];
        let zip_file_path = "test_output/test.zip";

        for path in &file_paths {
            let mut file = File::create(path).unwrap();
            writeln!(file, "Hello, world!").unwrap();
        }

        // Run the zip_files function
        rt.block_on(async {
            zip::zip_files(file_paths.clone(), zip_file_path).await.unwrap();
        });

        // Verify the zip file was created
        assert!(Path::new(zip_file_path).exists());

        // Clean up the sample files
        for path in &file_paths {
            fs::remove_file(path).unwrap();
        }

        // Unzip the created zip file to verify its contents
        let unzip_dest = "unzip_output";
        rt.block_on(async {
            unzip_file(zip_file_path, unzip_dest).await.unwrap();
        });

        // Verify the output files
        for path in &file_paths {
            let output_file_path = format!("{}/{}", unzip_dest, path);
            assert!(Path::new(&output_file_path).exists());

            // Verify the content of the unzipped files
            let mut file = File::open(&output_file_path).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            assert_eq!(content.trim(), "Hello, world!");
        }

        // Clean up
        fs::remove_file(zip_file_path).unwrap();
        fs::remove_dir_all(unzip_dest).unwrap();
        fs::remove_dir_all("test_output").unwrap();
    }
}
