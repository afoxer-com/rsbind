use std::fs;
use std::path::Path;
use build_script::dirzip::compress_dir;

fn main() {
    println!("cargo:rerun-if-changed=.out");
    println!("cargo:rerun-if-changed=template-android");
    println!("cargo:rerun-if-changed=template-ios");
    println!("cargo:rerun-if-changed=template-bridge-android");
    println!("cargo:rerun-if-changed=template-bridge-ios");

    println!("begin zip tempalate...");

    fs::create_dir_all(".out").unwrap();
    compress_dir(
        Path::new("./template/template-android"),
        Path::new(".out/template-android.zip"),
    );
    compress_dir(
        Path::new("./template/template-ios"),
        Path::new(".out/template-ios.zip"),
    );
    compress_dir(
        Path::new("./template/template-bridge-android"),
        Path::new(".out/template-bridge-android.zip"),
    );
    compress_dir(
        Path::new("./template/template-bridge-ios"),
        Path::new(".out/template-bridge-ios.zip"),
    );

    if !Path::new(".out/template-android.zip").exists() {
        panic!(".out/template-android.zip doesn't exist.")
    }
    fs::copy(
        ".out/template-android.zip",
        "src/android/res/template_android.zip",
    )
    .unwrap();

    if !Path::new(".out/template-ios.zip").exists() {
        panic!(".out/template-ios.zip doesn't exist.")
    }
    fs::copy(".out/template-ios.zip", "src/ios/res/template_ios.zip").unwrap();

    if !Path::new(".out/template-bridge-android.zip").exists() {
        panic!(".out/template-bridge-android.zip doesn't exist.")
    }
    fs::copy(
        ".out/template-bridge-android.zip",
        "src/android/res/template_bridge_android.zip",
    )
    .unwrap();

    if !Path::new(".out/template-bridge-ios.zip").exists() {
        panic!("../.out/template-bridge-ios.zip doesn't exist.")
    }
    fs::copy(
        ".out/template-bridge-ios.zip",
        "src/ios/res/template_bridge_ios.zip",
    )
    .unwrap();
}
