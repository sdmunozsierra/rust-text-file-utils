#![allow(unused_variables)]
#![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Seek, Write};
use std::path::Path;
use anyhow::Context;
use tokio::task;
use walkdir::{DirEntry, WalkDir};
use zip::write::{FileOptions, SimpleFileOptions};
use log::{error, info, debug};


use crate::config::logger;

// Initialize logging for the entire module
#[ctor::ctor]
fn init() {
    logger::init_logging();
}

pub async fn zip_files(src_dirs: Vec<&str>, dst_file: &str) -> io::Result<()> {
    let method = zip::CompressionMethod::Deflated;

    task::block_in_place(|| -> io::Result<()> {
        let path = Path::new(dst_file);
        let file = File::create(path)?;

        let mut zip = zip::ZipWriter::new(file);
        let options = SimpleFileOptions::default().compression_method(method);

        let mut buffer = Vec::new();

        for src_dir in src_dirs {
            let src_path = Path::new(src_dir);
            debug!("Checking path: {:?}", src_path);
            if !src_path.exists() {
                error!("Path does not exist: {:?}", src_path);
                return Err(io::Error::new(io::ErrorKind::NotFound, format!("Source directory or file not found: {}", src_dir)));
            }

            let base_path = src_path.file_name().unwrap().to_str().unwrap().to_owned();

            if src_path.is_dir() {
                debug!("Processing directory: {:?}", src_path);
                let walkdir = WalkDir::new(src_path);
                let it = walkdir.into_iter();
                zip_dir(&mut it.filter_map(|e| e.ok()), src_path, &mut zip, &mut buffer, &options, &base_path)
                    .map_err(|e| {
                        error!("Error zipping directory: {:?}", e);
                        io::Error::new(io::ErrorKind::Other, e.to_string())
                    })?;
            } else if src_path.is_file() {
                debug!("Processing file: {:?}", src_path);
                let name = format!("{}/{}", base_path, src_path.file_name().unwrap().to_str().unwrap());
                debug!("Adding file {src_path:?} as {name:?} ...");
                zip.start_file(name, options.clone())?;
                let mut f = File::open(src_path)?;

                f.read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;
                buffer.clear();
            } else {
                error!("Invalid path (neither file nor directory): {:?}", src_path);
                return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Source path is neither a file nor a directory: {}", src_dir)));
            }
        }

        zip.finish()?;
        info!("Zip file created successfully at {:?}", dst_file);
        Ok(())
    })
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &Path,
    zip: &mut zip::ZipWriter<T>,
    buffer: &mut Vec<u8>,
    options: &FileOptions<()>,
    base_path: &str,
) -> anyhow::Result<()>
where
    T: Write + Seek,
{
    let prefix = prefix.canonicalize().unwrap();
    debug!("Canonicalized prefix: {:?}", prefix);
    for entry in it {
        let path = entry.path();
        debug!("Processing entry: {:?}", path);
        let name = path.strip_prefix(&prefix).unwrap();
        let path_as_string = format!("{}/{}", base_path, name.to_str().map(str::to_owned).with_context(|| format!("{name:?} is a non-UTF-8 path"))?);

        if path.is_file() {
            debug!("Adding file {path:?} as {path_as_string:?} ...");
            zip.start_file(path_as_string, options.clone())?;
            let mut f = File::open(path)?;

            f.read_to_end(buffer)?;
            zip.write_all(buffer)?;
            buffer.clear();
        } else if path.is_dir() && !path_as_string.is_empty() {
            let dir_name = format!("{}/", path_as_string);
            debug!("Adding directory {dir_name:?} as {path_as_string:?} ...");
            zip.add_directory(dir_name, options.clone())?;
        }
    }
    debug!("Finished processing directory {:?}", prefix);
    Ok(())
}
