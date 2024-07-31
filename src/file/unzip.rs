use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tokio::task;

pub async fn unzip_file(zip_path: &str, dest_dir: &str) -> io::Result<()> {
    task::block_in_place(|| -> io::Result<()> {
        let fname = Path::new(zip_path);
        let file = fs::File::open(fname)?;

        let mut archive = zip::ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = match file.enclosed_name() {
                Some(path) => {
                    let mut full_path = Path::new(dest_dir).join(path).to_owned();
                    // Normalize the path to remove any redundant separators
                    full_path = full_path.components().collect::<PathBuf>();
                    full_path
                }
                None => continue,
            };

            if file.is_dir() {
                println!("Directory {} extracted to \"{}\"", i, outpath.display());
                if let Err(e) = fs::create_dir_all(&outpath) {
                    println!("Error creating directory {}: {}", outpath.display(), e);
                }
            } else {
                println!(
                    "File {} extracted to \"{}\" ({} bytes)",
                    i,
                    outpath.display(),
                    file.size()
                );
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        if let Err(e) = fs::create_dir_all(p) {
                            println!("Error creating directory {}: {}", p.display(), e);
                        }
                    }
                }
                let mut outfile = match fs::File::create(&outpath) {
                    Ok(f) => f,
                    Err(e) => {
                        println!("Error creating file {}: {}", outpath.display(), e);
                        continue;
                    }
                };
                if let Err(e) = io::copy(&mut file, &mut outfile) {
                    println!("Error copying file to {}: {}", outpath.display(), e);
                }
            }

            // Get and set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    if let Err(e) = fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)) {
                        println!("Error setting permissions for {}: {}", outpath.display(), e);
                    }
                }
            }
        }
        Ok(())
    })
}