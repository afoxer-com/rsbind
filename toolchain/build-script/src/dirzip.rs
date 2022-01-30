use std::fs;
use std::fs::File;
use std::io::{copy, Read, Seek, Write};
use std::path::Path;
use std::str;
use walkdir::{DirEntry, WalkDir};
use zip::write::FileOptions;

pub fn compress_dir(src_dir: &Path, target: &Path) {
    let zipfile = std::fs::File::create(target).unwrap();
    let dir = WalkDir::new(src_dir);
    zip_dir(
        &mut dir.into_iter().filter_map(|e| e.ok()),
        src_dir.to_str().unwrap(),
        zipfile,
    );
}
fn compress_file(src_dir: &Path, target: &Path) {
    let zipfile = std::fs::File::create(target).unwrap();
    let dir = WalkDir::new(src_dir);

    // src_dir.parent().map( |p| println!("{:?}",p.to_str().unwrap()));
    let prefix = src_dir
        .parent()
        .map_or_else(|| "/", |p| p.to_str().unwrap());
    zip_dir(&mut dir.into_iter().filter_map(|e| e.ok()), prefix, zipfile);
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        // .compression_method(zip::CompressionMethod::Bzip2)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();
        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {:?} as {:?} ...", path, name);
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if name.as_os_str().len() != 0 {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {:?} as {:?} ...", path, name);
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

pub fn extract(test: &Path, mut target: &Path) {
    let zipfile = std::fs::File::open(&test).unwrap();
    let mut zip = zip::ZipArchive::new(zipfile).unwrap();

    if !target.exists() {
        fs::create_dir_all(target).map_err(|e| {
            println!("{}", e);
        });
    }
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        println!("Filename: {} {:?}", file.name(), file.sanitized_name());
        if file.is_dir() {
            println!("file utf8 path {:?}", file.name_raw());
            let target = target.join(Path::new(&file.name().replace("\\", "")));
            fs::create_dir_all(target).unwrap();
        } else {
            let file_path = target.join(Path::new(file.name()));
            let mut target_file = if !file_path.exists() {
                println!("file path {}", file_path.to_str().unwrap());
                fs::File::create(file_path).unwrap()
            } else {
                fs::File::open(file_path).unwrap()
            };
            std::io::copy(&mut file, &mut target_file);
            // target_file.write_all(file.read_bytes().into());
        }
    }
}

fn create_dir(path: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(path)
}
