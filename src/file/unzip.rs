use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::Path;
use tokio::fs::create_dir_all;
use zip::read::ZipArchive;

pub async fn unzip_file(zip_path: &str, dest_dir: &str) -> io::Result<()> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(BufReader::new(file))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        let outpath = Path::new(dest_dir).join(outpath);

        if (*file.name()).ends_with('/') {
            create_dir_all(&outpath).await?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    create_dir_all(&p).await?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // Set the permissions if the file has any
        #[cfg(unix)]
        {
            use std::fs::Permissions;
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                let permissions = Permissions::from_mode(mode);
                tokio::fs::set_permissions(&outpath, permissions).await?;
            }
        }
    }

    Ok(())
}
