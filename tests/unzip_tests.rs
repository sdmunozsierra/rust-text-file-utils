use std::fs::{self, File};
use std::path::Path;
    use std::io::Write;
    use tokio::runtime::Runtime;
use rust_text_file_utils::file::unzip;
    
    #[test]
    fn test_unzip_file() {
        let rt = Runtime::new().unwrap();
        
        // Create a sample zip file for testing
        let zip_path = "test.zip";
        let dest_dir = "test_output";

        {
            let file = File::create(zip_path).unwrap();
            let mut zip = zip::ZipWriter::new(file);

            let options = zip::write::FileOptions::default();

            zip.start_file("test.txt", options).unwrap();
            zip.write_all(b"Hello, world!").unwrap();
            zip.finish().unwrap();
        }

        // Run the unzip function
        rt.block_on(async {
            unzip::unzip_file(zip_path, dest_dir).await.unwrap();
        });

        // Verify the output
        let output_file_path = format!("{}/test.txt", dest_dir);
        assert!(Path::new(&output_file_path).exists());

        // Clean up
        fs::remove_file(zip_path).unwrap();
        fs::remove_dir_all(dest_dir).unwrap();
    }
