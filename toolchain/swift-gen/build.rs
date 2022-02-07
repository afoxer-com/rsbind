use std::path::Path;
use std::process::Command;
use std::{fs, io};

fn main() {
    // // Tell compile if .out/swift changed, rerun it.
    // println!("cargo:rerun-if-changed=Sources");
    // println!("cargo:rerun-if-changed=swift_gen.xcodeproj");
    //
    // println!("begin building swift_gen in build.rs...");
    // let mut swift = Command::new("swift");
    // swift.arg("build").arg("--configuration").arg("release");
    //
    // let status = swift.status().unwrap();
    // println!("process {:?} result: {:?}", swift, status.code());
    //
    // fs::create_dir_all("../.out").unwrap();
    // let copy = fs::copy("./.build/release/SwiftGen", "../.out/swift_gen").unwrap();
    // println!("copy artifact result: {}", copy)
}
