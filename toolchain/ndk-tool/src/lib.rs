use std::error::Error;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use ndk_build::cargo::cargo_ndk;
use ndk_build::target::Target;

use error::*;
use error::ErrorKind::*;

pub mod error;

pub struct BuildConfig {
    pub lib_name: String,
    pub arch_list: Vec<String>,
    pub is_release: bool,
    pub target_dir: String,
    pub project_dir: PathBuf,
    pub sdk_version: u32,
}

pub fn build(config: &BuildConfig) -> Result<()> {
    println!("building android bridge project");

    let ndk = ndk_build::ndk::Ndk::from_env()?;

    let archs = &config.arch_list;

    for arch in archs.iter() {
        let target = Target::from_rust_triple(arch)?;
        let mut cargo = cargo_ndk(&ndk, target, config.sdk_version)?;
        cargo
            .arg("rustc")
            .arg("--target")
            .arg(arch)
            .arg("--lib")
            .arg("--target-dir")
            .arg(&config.target_dir)
            // .arg(&self.config().rustc_param())
            .current_dir(&config.project_dir);

        if config.is_release {
            cargo.arg("--release");
        }

        // Workaround for https://github.com/rust-windowing/android-ndk-rs/issues/149:
        // Rust (1.56 as of writing) still requires libgcc during linking, but this does
        // not ship with the NDK anymore since NDK r23 beta 3.
        // See https://github.com/rust-lang/rust/pull/85806 for a discussion on why libgcc
        // is still required even after replacing it with libunwind in the source.
        // XXX: Add an upper-bound on the Rust version whenever this is not necessary anymore.
        if ndk.build_tag() > 7272597 {
            cargo.arg("--");
            let cargo_apk_link_dir = Path::new(&config.project_dir)
                .join("target")
                .join("cargo-apk-temp-extra-link-libraries");
            std::fs::create_dir_all(&cargo_apk_link_dir)?;
            let libgccfile = cargo_apk_link_dir.join("libgcc.a");
            if !libgccfile.exists() {
                std::fs::write(libgccfile, "INPUT(-lunwind)").expect("Failed to write");
            }

            cargo.arg("-L").arg(
                PathBuf::new()
                    .join("target")
                    .join("cargo-apk-temp-extra-link-libraries"),
            );
        }

        let output = cargo.output()?;

        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;

        let status = cargo.status()?;
        println!("process '{:?}' finished with: {}", cargo, status);

        let debug_release = if config.is_release {
            "release"
        } else {
            "debug"
        };
        let strip = ndk.toolchain_bin("strip", target)?;
        let mut strip_comm = Command::new(strip);
        let strip_output = strip_comm
            .arg("-s")
            .arg(format!(
                "{}/{}/{}/{}",
                &config.target_dir, arch, debug_release, config.lib_name
            ))
            .current_dir(&config.project_dir)
            .output()?;

        io::stdout().write_all(&strip_output.stdout)?;
        io::stderr().write_all(&strip_output.stderr)?;

        let strip_status = strip_comm.status()?;
        println!("process '{:?}' finished with: {}", strip_comm, strip_status);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use ndk_build::target::Target;

    use crate::{build, BuildConfig};

    #[test]
    fn build_works() {
        println!("current dir = {:?}", std::env::current_dir());
        let config = BuildConfig {
            lib_name: "testcode".to_string(),
            arch_list: vec![Target::Arm64V8a.rust_triple().to_owned(),
                            Target::ArmV7a.rust_triple().to_owned(),
                            Target::X86.rust_triple().to_owned(),
                            Target::X86_64.rust_triple().to_owned(),
            ],
            is_release: true,
            target_dir: "target".to_string(),
            project_dir: Path::new(".").to_path_buf(),
            sdk_version: 21,
        };
        build(&config).unwrap()
    }
}
