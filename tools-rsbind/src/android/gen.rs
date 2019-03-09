use errors::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const BIN_DIR: &str = "javabind-1.0-SNAPSHOT";

pub fn gen_java_code(
    origin_prj: &PathBuf,
    prj_dir: &PathBuf,
    ast_dir: &PathBuf,
    bin_dir: &PathBuf,
    namespace: String,
    so_name: String,
    ext_libs: String,
) -> Result<()> {
    // get the ast path string
    println!("get ast path string");
    let ast_dir_tmp = ast_dir.canonicalize()?;
    let ast_dir_str = ast_dir_tmp.to_str().ok_or(Error::FileError(
        "get ast dir path string error".to_string(),
    ))?;

    // get the java_gen dir string
    println!("get java_gen dir string");
    let parent = prj_dir.parent().ok_or(Error::FileError(
        "can't find parent dir for java".to_string(),
    ))?;
    let java_gen_path = parent.join("java_gen");
    if java_gen_path.exists() {
        fs::remove_dir_all(&java_gen_path)?;
    }
    fs::create_dir_all(&java_gen_path)?;
    let java_gen_dir_tmp = java_gen_path.canonicalize()?;
    let java_gen_dir_str = java_gen_dir_tmp
        .to_str()
        .ok_or(Error::FileError("get swift gen dir str wrong.".to_string()))?;

    // get the output dir string
    println!("get output dir string");
    let output_dir_tmp = prj_dir
        .join("rustlib")
        .join("src")
        .join("main")
        .join("java")
        .canonicalize()?;
    let output_dir_str = output_dir_tmp.to_str().ok_or(Error::FileError(
        "get java dir path string error.".to_string(),
    ))?;

    // get the bin file path string
    println!("get javabind bin string.");
    let bin_path = bin_dir.join(BIN_DIR).join("bin").join("javabind");
    let tmp_bin_path = bin_path.canonicalize()?;
    let bin_path_str = tmp_bin_path.to_str().ok_or(Error::FileError(
        "can't get javabind path string.".to_owned(),
    ))?;

    let cmd = format!(
        "chmod a+x {} && {} \"{}\" \"{}\" \"{}\" \"{}\" \"{}\" && cp -rf {}/* {}",
        bin_path_str,
        bin_path_str,
        &namespace,
        &so_name,
        &ext_libs,
        ast_dir_str,
        java_gen_dir_str,
        java_gen_dir_str,
        output_dir_str
    );
    println!("execute {}", &cmd);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .current_dir(origin_prj)
        .output()
        .map_err(|e| Error::CommandError(format!("run javabin cmd error, err = {:?}", e)))?;

    if !output.status.success() {
        return Err(Error::CommandError(
            format!("execute java gen error. {:?}", output).to_string(),
        ));
    }

    Ok(())
}
