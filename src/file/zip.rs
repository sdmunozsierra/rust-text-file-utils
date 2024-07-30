use std::fs::File;
use std::io::{self, Write, Read};
use std::path::Path;
use tokio::fs::create_dir_all;
use zip::write::FileOptions;
use zip::ZipWriter;

pub async fn zip_files(file_paths: Vec<&str>, zip_file_path: &str) -> io::Result<()> {
    let zip_path = Path::new(zip_file_path);

    // Create the destination directory if it doesn't exist
    if let Some(parent) = zip_path.parent() {
        create_dir_all(parent).await?;
    }

    let zip_file = File::create(zip_path)?;
    let mut zip = ZipWriter::new(zip_file);

    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for file_path in file_paths {
        let path = Path::new(file_path);
        if path.is_file() {
            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            zip.start_file(path.file_name().unwrap().to_str().unwrap(), options)?;
            zip.write_all(&buffer)?;
        }
    }

    zip.finish()?;
    Ok(())
}