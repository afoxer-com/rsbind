extern crate build_script;

use build_script::dirzip::compress_dir;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=./template-android");
    println!("cargo:rerun-if-changed=./template-ios");
    println!("cargo:rerun-if-changed=./template-bridge-android");
    println!("cargo:rerun-if-changed=./template-bridge-ios");

    println!("begin zip tempalate...");

    fs::create_dir_all("../.out").unwrap();
    compress_dir(
        Path::new("./template-android"),
        Path::new("../.out/template-android.zip"),
    );
    compress_dir(
        Path::new("./template-ios"),
        Path::new("../.out/template-ios.zip"),
    );
    compress_dir(
        Path::new("./template-bridge-android"),
        Path::new("../.out/template-bridge-android.zip"),
    );
    compress_dir(
        Path::new("./template-bridge-ios"),
        Path::new("../.out/template-bridge-ios.zip"),
    );
}
