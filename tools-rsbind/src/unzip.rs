use errors::*;
use errors::ErrorKind::*;
use std::fs;
use std::fs::File;
use std::io::Cursor;
use std::io::Write;
use std::path::PathBuf;

use std::io::Read;
use zip::ZipArchive;

pub(crate) fn unzip_to(buf: &[u8], path: &PathBuf) -> Result<()> {
    let reader = Cursor::new(buf);
    let mut archive = ZipArchive::new(reader).map_err(|e| ZipError(e.to_string()))?;

    println!("begin unzip every file. len = {}", archive.len());
    for i in 0..archive.len() {
        let mut zip_file = archive
            .by_index(i)
            .map_err(|e| ZipError(e.to_string()))?;

        println!("unzip file name = {}", &zip_file.name());
        let file_path = path.join(&zip_file.name());
        if zip_file.name().ends_with("/") {
            if file_path.exists() {
                fs::remove_dir_all(&file_path)?;
            }
            fs::create_dir_all(&file_path)?;
            continue;
        }

        let parent_path = file_path.parent().ok_or(ZipError(format!(
            "can't find parent path for {:?}",
            &file_path
        )))?;

        fs::create_dir_all(&parent_path)?;

        let mut file = File::create(&file_path).map_err(|e| {
            ZipError(format!(
                "can't create file for {:?}, with error => {:?}",
                &file_path, e
            ))
        })?;

        for byte in zip_file.bytes() {
            let byte = byte.map_err(|e| {
                ZipError(format!(
                    "read bytes from zip error, file = {:?}, error => {:?}",
                    &file_path, e
                ))
            })?;
            file.write_all(&[byte])?;
        }
    }

    Ok(())
}
