use std::fs;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use zip::ZipArchive;

use crate::errors::*;
use crate::errors::ErrorKind::*;

pub(crate) fn unzip_to(buf: &[u8], path: &Path) -> Result<()> {
    let reader = Cursor::new(buf);
    let mut archive = ZipArchive::new(reader).map_err(|e| ZipError(e.to_string()))?;

    println!("begin unzip every file. len = {}", archive.len());
    for i in 0..archive.len() {
        let zip_file = archive.by_index(i).map_err(|e| ZipError(e.to_string()))?;

        let file_path = path.join(&zip_file.name());
        println!(
            "unzip file name = {} ==> {:?}",
            &zip_file.name(),
            &file_path
        );

        if zip_file.is_dir() {
            if !file_path.exists() {
                fs::create_dir_all(&file_path)?;
            }
            continue;
        }

        let parent_path = file_path
            .parent()
            .ok_or_else(|| ZipError(format!("can't find parent path for {:?}", &file_path)))?;

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
