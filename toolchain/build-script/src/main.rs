use std::fs;
use std::path::Path;

use build_script::dirzip::compress_dir;

fn main() {
    println!("begin zip tempalate...");

    let outdir = ".out";

    //     match std::env::var_os("OUT_DIR") {
    //     None => return,
    //     Some(outdir) => outdir,
    // };

    // Zip all template
    fs::create_dir_all(format!("{}/template", outdir)).unwrap();
    compress_dir(
        Path::new("./template/template-android"),
        Path::new(&format!("{}/template/template-android.zip", outdir)),
    );
    compress_dir(
        Path::new("./template/template-bridge-android"),
        Path::new(&format!("{}/template/template-bridge-android.zip", outdir)),
    );
    compress_dir(
        Path::new("./template/template-ios"),
        Path::new(&format!("{}/template/template-ios.zip", outdir)),
    );
    compress_dir(
        Path::new("./template/template-bridge-ios"),
        Path::new(&format!("{}/template/template-bridge-ios.zip", outdir)),
    );

    compress_dir(
        Path::new("./template/template-mac"),
        Path::new(&format!("{}/template/template-mac.zip", outdir)),
    );
    compress_dir(
        Path::new("./template/template-bridge-mac"),
        Path::new(&format!("{}/template/template-bridge-mac.zip", outdir)),
    );

    compress_dir(
        Path::new("./template/template-jar"),
        Path::new(&format!("{}/template/template-jar.zip", outdir)),
    );
    compress_dir(
        Path::new("./template/template-bridge-jar"),
        Path::new(&format!("{}/template/template-bridge-jar.zip", outdir)),
    );

    fs::copy(
        &format!("{}/template/template-ios.zip", outdir),
        "src/ios/res/template_ios.zip",
    )
    .unwrap();

    fs::copy(
        &format!("{}/template/template-bridge-ios.zip", outdir),
        "src/ios/res/template_bridge_ios.zip",
    )
    .unwrap();

    fs::copy(
        &format!("{}/template/template-bridge-android.zip", outdir),
        "src/android/res/template_bridge_android.zip",
    )
    .unwrap();

    fs::copy(
        &format!("{}/template/template-android.zip", outdir),
        "src/android/res/template_android.zip",
    )
    .unwrap();

    fs::copy(
        &format!("{}/template/template-bridge-mac.zip", outdir),
        "src/mac/res/template_bridge_mac.zip",
    )
    .unwrap();

    fs::copy(
        &format!("{}/template/template-mac.zip", outdir),
        "src/mac/res/template_mac.zip",
    )
    .unwrap();

    fs::copy(
        &format!("{}/template/template-bridge-jar.zip", outdir),
        "src/jar/res/template_bridge_jar.zip",
    )
    .unwrap();

    fs::copy(
        &format!("{}/template/template-jar.zip", outdir),
        "src/jar/res/template_jar.zip",
    )
    .unwrap();
}
