use errors::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn gen_swift_code(prj_dir: &PathBuf, ast_dir: &PathBuf, bin_dir: &PathBuf) -> Result<()> {
    print!("gen_swift_code");

    fs::create_dir_all(&bin_dir)
        .or_else(|e| Err(Error::FileError("create bin dir failed.".to_string())))
        .unwrap();

    let swift_gen_bin_buf: &[u8] = include_bytes!("res/swift_gen");
    let bin_file_path = bin_dir.join("swift_gen");
    if bin_file_path.exists() {
        fs::remove_file(&bin_file_path)
            .or_else(|e| Err(Error::FileError("remove old swift bin failed.".to_string())))
            .unwrap();
    }
    let _ = fs::File::create(&bin_file_path)
        .or_else(|e| {
            Err(Error::FileError(format!(
                "create new swift bin failed. cause = {}",
                e
            )))
        })
        .unwrap();
    fs::write(&bin_file_path, swift_gen_bin_buf).unwrap();

    let ast_dir_tmp = ast_dir.canonicalize().unwrap();
    let ast_dir_str = ast_dir_tmp
        .to_str()
        .ok_or(Error::FileError(
            "get ast dir path string error".to_string(),
        ))
        .unwrap();

    let parent = prj_dir
        .parent()
        .ok_or(Error::FileError(
            "can't find parent dir for swift".to_string(),
        ))
        .unwrap();
    let swift_gen_path = parent.join("swift_gen");
    if swift_gen_path.exists() {
        fs::remove_dir_all(&swift_gen_path).unwrap();
    }
    fs::create_dir_all(&swift_gen_path).unwrap();

    let swift_gen_dir_tmp = swift_gen_path.canonicalize().unwrap();
    let swift_gen_dir_str = swift_gen_dir_tmp
        .to_str()
        .ok_or(Error::FileError("get swift gen dir str wrong.".to_string()))
        .unwrap();

    let output_dir_tmp = prj_dir.join("rustlib").canonicalize().unwrap();
    let output_dir_str = output_dir_tmp
        .to_str()
        .ok_or(Error::FileError(
            "get swift dir path string error.".to_string(),
        ))
        .unwrap();

    println!(
        "generating swift code, ast dir = {}, out dir = {}",
        ast_dir_str, swift_gen_dir_str
    );

    let command = format!(
        "chmod a+x ./swift_gen && ./swift_gen {} rustlib.ffi {} && cp {}/* {}",
        ast_dir_str, swift_gen_dir_str, swift_gen_dir_str, output_dir_str
    );

    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .current_dir(bin_dir)
        .output()
        .unwrap();

    print!("gen_swift_code over");

    if !output.status.success() {
        return Err(Error::CommandError(
            format!("execute swift gen error. {:?}", output).to_string(),
        ));
    }

    Ok(())
}
