use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=../.out");

    if !Path::new("../.out/swift_gen").exists() {
        panic!("../.out/swift_gen doesn't exist.")
    }
    fs::copy("../.out/swift_gen", "src/ios/res/swift_gen").unwrap();

    if !Path::new("../.out/template-android.zip").exists() {
        panic!("../.out/template-android.zip doesn't exist.")
    }
    fs::copy(
        "../.out/template-android.zip",
        "src/android/res/template_android.zip",
    )
    .unwrap();

    if !Path::new("../.out/template-ios.zip").exists() {
        panic!("../.out/template-ios.zip doesn't exist.")
    }
    fs::copy("../.out/template-ios.zip", "src/ios/res/template_ios.zip").unwrap();

    if !Path::new("../.out/template-bridge-android.zip").exists() {
        panic!("../.out/template-bridge-android.zip doesn't exist.")
    }
    fs::copy(
        "../.out/template-bridge-android.zip",
        "src/android/res/template_bridge_android.zip",
    )
    .unwrap();

    if !Path::new("../.out/template-bridge-ios.zip").exists() {
        panic!("../.out/template-bridge-ios.zip doesn't exist.")
    }
    fs::copy(
        "../.out/template-bridge-ios.zip",
        "src/ios/res/template_bridge_ios.zip",
    )
    .unwrap();
}
