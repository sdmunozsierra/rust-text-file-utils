use std::fs::File;
use std::io::{self};
use zip::read::ZipArchive;

pub fn list_files_in_zip(zip_file_path: &str, recursive: bool) -> io::Result<Vec<String>> {
    let file = File::open(zip_file_path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut file_list = Vec::new();

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let file_name = file.name().to_string();

        if file.is_dir() {
            if !recursive {
                file_list.push(file_name);
            }
        } else if file.is_file() {
            file_list.push(file_name);
        }
    }

    if recursive {
        file_list.sort();
    }

    Ok(file_list)
}

pub fn get_zip_info(zip_file_path: &str) -> io::Result<ZipInfo> {
    let file = File::open(zip_file_path)?;
    let mut archive = ZipArchive::new(file)?;

    let mut total_size = 0;
    let mut total_files = 0;
    let mut total_dirs = 0;

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        if file.is_file() {
            total_files += 1;
            total_size += file.size();
        } else if file.is_dir() {
            total_dirs += 1;
        }
    }

    Ok(ZipInfo {
        total_files,
        total_dirs,
        total_size,
    })
}

pub struct ZipInfo {
    pub total_files: usize,
    pub total_dirs: usize,
    pub total_size: u64,
}

impl std::fmt::Display for ZipInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Total files: {}\nTotal directories: {}\nTotal size: {} bytes",
            self.total_files, self.total_dirs, self.total_size
        )
    }
}
