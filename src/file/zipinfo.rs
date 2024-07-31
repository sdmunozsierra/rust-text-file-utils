// zipinfo.rs

use std::fs::File;
use std::io::{self, ErrorKind};
use zip::read::ZipArchive;
use log::{error, info};

use crate::config::logger;

// Initialize logging for the entire module
#[ctor::ctor]
fn init() {
    logger::init_logging();
}

pub fn list_files_in_zip(zip_file_path: &str, recursive: bool) -> io::Result<Vec<String>> {
    let file = match File::open(zip_file_path) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to open zip file {}: {}", zip_file_path, e);
            return Err(e);
        }
    };

    let mut archive = match ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(e) => {
            error!("Failed to read zip archive from file {}: {}", zip_file_path, e);
            return Err(io::Error::new(ErrorKind::InvalidData, e));
        }
    };

    let mut file_list = Vec::new();

    for i in 0..archive.len() {
        match archive.by_index(i) {
            Ok(file) => {
                let file_name = file.name().to_string();
                if file.is_dir() {
                    if !recursive {
                        file_list.push(file_name);
                    }
                } else if file.is_file() {
                    file_list.push(file_name);
                }
            }
            Err(e) => {
                error!("Failed to read file at index {} in zip archive {}: {}", i, zip_file_path, e);
                return Err(io::Error::new(ErrorKind::InvalidData, e));
            }
        }
    }

    if recursive {
        file_list.sort();
    }

    info!("Successfully listed files in zip file {}", zip_file_path);
    Ok(file_list)
}

pub fn get_zip_info(zip_file_path: &str) -> io::Result<ZipInfo> {
    let file = match File::open(zip_file_path) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to open zip file {}: {}", zip_file_path, e);
            return Err(e);
        }
    };

    let mut archive = match ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(e) => {
            error!("Failed to read zip archive from file {}: {}", zip_file_path, e);
            return Err(io::Error::new(ErrorKind::InvalidData, e));
        }
    };

    let mut total_size = 0;
    let mut total_files = 0;
    let mut total_dirs = 0;

    for i in 0..archive.len() {
        match archive.by_index(i) {
            Ok(file) => {
                if file.is_file() {
                    total_files += 1;
                    total_size += file.size();
                } else if file.is_dir() {
                    total_dirs += 1;
                }
            }
            Err(e) => {
                error!("Failed to read file at index {} in zip archive {}: {}", i, zip_file_path, e);
                return Err(io::Error::new(ErrorKind::InvalidData, e));
            }
        }
    }

    info!("Successfully retrieved zip info for file {}", zip_file_path);
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