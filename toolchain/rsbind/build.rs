use std::fs;
use std::path::Path;
use build_script::dirzip::compress_dir;

fn main() {
    println!("cargo:rerun-if-changed=template");
    println!("cargo:rerun-if-changed=template-android");
    println!("cargo:rerun-if-changed=template-ios");
    println!("cargo:rerun-if-changed=template-bridge-android");
    println!("cargo:rerun-if-changed=template-bridge-ios");

    println!("begin zip tempalate...");

    let outdir = match std::env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };

    fs::create_dir_all(format!("{}/template", outdir.to_string_lossy())).unwrap();
    compress_dir(
        Path::new("./template/template-android"),
        Path::new(&format!("{}/template/template-android.zip", outdir.to_string_lossy())),
    );
    compress_dir(
        Path::new("./template/template-ios"),
        Path::new(&format!("{}/template/template-ios.zip", outdir.to_string_lossy())),
    );
    compress_dir(
        Path::new("./template/template-bridge-android"),
        Path::new(&format!("{}/template/template-bridge-android.zip", outdir.to_string_lossy())),
    );
    compress_dir(
        Path::new("./template/template-bridge-ios"),
        Path::new(&format!("{}/template/template-bridge-ios.zip", outdir.to_string_lossy())),
    );

    if !Path::new(&format!("{}/template/template-android.zip", outdir.to_string_lossy())).exists() {
        panic!("/template/template-android.zip doesn't exist.")
    }
    // fs::copy(
    //     &format!("{}/template/template-android.zip", outdir.to_string_lossy()),
    //     "src/android/res/template_android.zip",
    // )
    // .unwrap();
    //
    // if !Path::new(&format!("{}/template/template-ios.zip", outdir.to_string_lossy())).exists() {
    //     panic!("template/template-ios.zip doesn't exist.")
    // }
    // fs::copy(&format!("{}/template/template-ios.zip", outdir.to_string_lossy()), "src/ios/res/template_ios.zip").unwrap();
    //
    // if !Path::new(&format!("{}/template/template-bridge-android.zip", outdir.to_string_lossy())).exists() {
    //     panic!("template/template-bridge-android.zip doesn't exist.")
    // }
    // fs::copy(
    //     &format!("{}/template/template-bridge-android.zip", outdir.to_string_lossy()),
    //     "src/android/res/template_bridge_android.zip",
    // )
    // .unwrap();
    //
    // if !Path::new(&format!("{}/template/template-bridge-ios.zip", outdir.to_string_lossy())).exists() {
    //     panic!("template/template-bridge-ios.zip doesn't exist.")
    // }
    // fs::copy(
    //     &format!("{}/template/template-bridge-ios.zip", outdir.to_string_lossy()),
    //     "src/ios/res/template_bridge_ios.zip",
    // )
    // .unwrap();
}
