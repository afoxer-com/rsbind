use super::dest;
use ast::AstResult;
use bridges::BridgeGen::JavaGen;
use bridge::prj::Unpack;
use process::BuildProcess;
use config::Config as BuildConfig;
use errors::ErrorKind::*;
use errors::*;
use fs_extra;
use fs_extra::dir::CopyOptions;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use unzip;

const PHONE_ARCHS: [&str; 2] = ["armv7-linux-androideabi", "arm-linux-androideabi"];
const PHONE64_ARCHS: [&str; 1] = ["aarch64-linux-android"];
const X86_ARCHS: [&str; 1] = ["i686-linux-android"];
const NAMESPACE: &str = "com.afoxer.xxx.ffi";

const MAGIC_NUM: &'static str = "*521%";

pub(crate) struct AndroidProcess<'a> {
    origin_prj_path: &'a PathBuf,
    dest_prj_path: &'a PathBuf,
    bridge_prj_path: &'a PathBuf,
    ast_path: &'a PathBuf,
    bin_path: &'a PathBuf,
    host_crate_name: &'a str,
    ast_result: &'a AstResult,
    config: Option<BuildConfig>,
}

impl<'a> AndroidProcess<'a> {
    pub fn new(
        origin_prj_path: &'a PathBuf,
        dest_prj_path: &'a PathBuf,
        bridge_prj_path: &'a PathBuf,
        ast_path: &'a PathBuf,
        bin_path: &'a PathBuf,
        host_crate_name: &'a str,
        ast_result: &'a AstResult,
        config: Option<BuildConfig>,
    ) -> Self {
        AndroidProcess {
            origin_prj_path,
            dest_prj_path,
            bridge_prj_path,
            ast_path,
            bin_path,
            host_crate_name,
            ast_result,
            config,
        }
    }
}

impl<'a> AndroidProcess<'a> {
    fn lib_name(&self) -> String {
        return format!(
            "lib{}_android_bridge_prj.so",
            &self.host_crate_name.replace("-", "_")
        );
    }

    fn namespace(&self) -> String {
        match self.config.clone() {
            Some(config) => match config.android {
                Some(android) => match android.namespace {
                    Some(ref namespace) => namespace.to_owned(),
                    None => NAMESPACE.to_owned(),
                },
                None => NAMESPACE.to_owned(),
            },
            None => NAMESPACE.to_owned(),
        }
    }

    fn rustc_param(&self) -> String {
        let init = "--features rsbind";
        match self.config.clone() {
            Some(config) => match config.android {
                Some(android) => match android.rustc_param {
                    Some(rustc) => format!("{} {}", &rustc, init),
                    None => init.to_owned(),
                },
                None => init.to_owned(),
            },
            None => init.to_owned(),
        }
    }

    fn release_str(&self) -> String {
        if self.is_release() {
            "--release".to_owned()
        } else {
            "".to_owned()
        }
    }

    fn is_release(&self) -> bool {
        match self.config.clone() {
            Some(config) => match config.android {
                Some(android) => match android.release {
                    Some(is_release) => is_release,
                    None => true,
                },
                None => true,
            },
            None => true,
        }
    }

    fn phone_archs(&self) -> Vec<String> {
        let default_phone_archs = PHONE_ARCHS
            .to_vec()
            .into_iter()
            .map(|a| a.to_owned())
            .collect();
        match self.config.clone() {
            Some(config) => match config.android {
                Some(android) => match android.arch {
                    Some(arch) => arch,
                    None => default_phone_archs,
                },
                None => default_phone_archs,
            },
            None => default_phone_archs,
        }
    }

    fn phone64_archs(&self) -> Vec<String> {
        let default_phone64_archs = PHONE64_ARCHS
            .to_vec()
            .into_iter()
            .map(|a| a.to_owned())
            .collect();
        match self.config.clone() {
            Some(config) => match config.android {
                Some(android) => match android.arch_64 {
                    Some(arch) => arch,
                    None => default_phone64_archs,
                },
                None => default_phone64_archs,
            },
            None => default_phone64_archs,
        }
    }

    fn x86_archs(&self) -> Vec<String> {
        let default_x86_archs = X86_ARCHS
            .to_vec()
            .into_iter()
            .map(|a| a.to_owned())
            .collect();
        match self.config.clone() {
            Some(config) => match config.android {
                Some(android) => match android.arch_x86 {
                    Some(arch) => arch,
                    None => default_x86_archs,
                },
                None => default_x86_archs,
            },
            None => default_x86_archs,
        }
    }

    fn so_name(&self) -> String {
        match self.config.clone() {
            Some(config) => match config.android {
                Some(android) => match android.so_name {
                    Some(so_name) => so_name,
                    None => "ffi".to_owned(),
                },
                None => "ffi".to_owned(),
            },
            None => "ffi".to_owned(),
        }
    }

    fn ext_libs(&self) -> String {
        let ext_libs = match self.config.clone() {
            Some(config) => match config.android {
                Some(android) => match android.ext_lib {
                    Some(ext_lib) => ext_lib,
                    None => vec![],
                },
                None => vec![],
            },
            None => vec![],
        };

        let mut result = String::new();
        let mut index = 0;
        for ext_lib in ext_libs.iter() {
            if index == 0 {
                result = ext_lib.to_owned();
            } else if index < ext_libs.len() {
                result = format!("{},{}", &result, ext_lib)
            }
            index = index + 1;
        }

        result
    }

    fn features(&self) -> Vec<String> {
        match self.config.clone() {
            Some(config) => match config.android {
                Some(ios) => match ios.features_def {
                    Some(features) => features,
                    None => vec![],
                },
                None => vec![],
            },
            None => vec![],
        }
    }
}

impl<'a> BuildProcess for AndroidProcess<'a> {
    fn unpack(&self) -> Result<()> {
        Ok(())
    }

    fn gen_bridge_src(&self) -> Result<()> {
        println!("begin unzip rust template for android");
        // unpack the bridge project.
        {
            let buf: &[u8] = include_bytes!("res/template_bridge_android.zip");
            let unpack = Unpack {
                path: self.bridge_prj_path,
                host_crate: self.host_crate_name,
                buf,
                features: &self.features(),
            };

            unpack.unpack()?;
        }

        let bridge_c_src_path = self.bridge_prj_path.join("src").join("java").join("bridge");
        fs::create_dir_all(&bridge_c_src_path)?;
        JavaGen(
            self.host_crate_name.to_owned(),
            self.ast_result,
            &bridge_c_src_path,
            self.namespace(),
        )
        .gen_bridges()
        .unwrap();

        let _ = Command::new("cargo")
            .arg("fmt")
            .current_dir(&self.bridge_prj_path)
            .output();

        Ok(())
    }

    fn build_bridge_prj(&self) -> Result<()> {
        println!("building android bridge project");

        let phone_archs = self.phone_archs();
        let mut build_cmds = String::from("true");
        for phone_arch in phone_archs.iter() {
            let tmp = format!(
                "cargo rustc --target {}  --lib {} --target-dir {} {}",
                phone_arch,
                self.release_str(),
                "target",
                &self.rustc_param()
            );
            build_cmds = format!("{} && {}", &build_cmds, &tmp);
        }

        let phone64_archs = self.phone64_archs();
        for phone64_arch in phone64_archs.iter() {
            let tmp = format!(
                "cargo rustc --target {}  --lib {} --target-dir {} {}",
                phone64_arch,
                self.release_str(),
                "target",
                &self.rustc_param()
            );
            build_cmds = format!("{} && {}", &build_cmds, &tmp);
        }

        let x86_archs = self.x86_archs();
        for x86_arch in x86_archs.iter() {
            let tmp = format!(
                "cargo rustc --target {}  --lib {} --target-dir {} {}",
                x86_arch,
                self.release_str(),
                "target",
                &self.rustc_param()
            );
            build_cmds = format!("{} && {}", &build_cmds, &tmp);
        }

        let debug_release = if self.is_release() {
            "release"
        } else {
            "debug"
        };

        let mut strip_cmds = String::from("true");
        for phone_arch in phone_archs.iter() {
            let tmp = format!(
                "arm-linux-androideabi-strip -s target/{}/{}/{}",
                phone_arch,
                debug_release,
                self.lib_name()
            );
            strip_cmds = format!("{} && {}", &strip_cmds, &tmp);
        }

        for phone64_arch in phone64_archs.iter() {
            let tmp = format!(
                "aarch64-linux-android-strip -s target/{}/{}/{}",
                phone64_arch,
                debug_release,
                self.lib_name()
            );
            strip_cmds = format!("{} && {}", &strip_cmds, &tmp);
        }

        let cmds = format!("{} && {}", &build_cmds, &strip_cmds);

        println!("run building => {}", &cmds);

        let output = Command::new("sh")
            .arg("-c")
            .arg(cmds)
            .current_dir(self.bridge_prj_path)
            .output()?;

        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;

        if !output.status.success() {
            return Err(
                CommandError(format!("run build android rust project build failed. ")).into(),
            );
        }

        Ok(())
    }

    fn copy_bridge_outputs(&self) -> Result<()> {
        let mut copy_map = HashMap::new();
        copy_map.insert("arm-linux-androideabi", "armeabi");
        copy_map.insert("armv7-linux-androideabi", "armeabi-v7a");
        copy_map.insert("aarch64-linux-android", "arm64-v8a");
        copy_map.insert("i686-linux-android", "x86");

        let options = CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 1024,
            copy_inside: true,
            depth: 65535,
        };

        println!("copy output files to android project.");

        let debug_release = if self.is_release() {
            "release"
        } else {
            "debug"
        };

        for entry in copy_map.iter() {
            let armeabi_src = self
                .bridge_prj_path
                .join("target")
                .join(entry.0)
                .join(debug_release)
                .join(&self.lib_name());

            if !armeabi_src.exists() {
                continue;
            }

            let armeabi_dest = self
                .dest_prj_path
                .join("rustlib")
                .join("src")
                .join("main")
                .join("jniLibs")
                .join(entry.1);
            fs_extra::copy_items(&vec![armeabi_src], &armeabi_dest, &options)
                .map_err(|e| FileError(format!("copy android bridge outputs failed. {:?}", e)))?;
            fs::rename(
                &armeabi_dest.join(&self.lib_name()),
                &armeabi_dest.join(format!("lib{}.so", &self.so_name())),
            )?;
        }

        Ok(())
    }

    fn gen_bind_code(&self) -> Result<()> {
        // unpack the dest java project
        {
            println!("begin unzip android template");
            if self.dest_prj_path.exists() {
                fs::remove_dir_all(&self.dest_prj_path).unwrap();
            }
            fs::create_dir_all(&self.dest_prj_path).unwrap();
            let android_template_buf: &[u8] = include_bytes!("res/template_android.zip");
            unzip::unzip_to(android_template_buf, &self.dest_prj_path).unwrap();

            let manifest_path = self
                .dest_prj_path
                .join("rustlib")
                .join("src")
                .join("main")
                .join("AndroidManifest.xml");
            let manifest_text = fs::read_to_string(&manifest_path).map_err(|e| {
                FileError(format!(
                    "read android dest project AndroidManifest.xml error: {:?}",
                    e
                ))
            })?;
            let replaced =
                manifest_text.replace(&format!("$({}-namespace)", MAGIC_NUM), &self.namespace());
            fs::write(manifest_path, replaced).map_err(|e| {
                FileError(format!(
                    "write android dest project AndroidManifest  error {:?}",
                    e
                ))
            })?;
        }

        // unpack the javabind bin
        fs::create_dir_all(&self.bin_path)?;
        let bin_buf: &[u8] = include_bytes!("res/javabind.zip");
        unzip::unzip_to(bin_buf, &self.bin_path).unwrap();

        println!("generate java code.");
        dest::gen_java_code(
            self.origin_prj_path,
            &self.dest_prj_path,
            &self.ast_path,
            &self.bin_path,
            self.namespace(),
            self.so_name(),
            self.ext_libs(),
        )
        .unwrap();
        Ok(())
    }

    fn build_dest_prj(&self) -> Result<()> {
        println!("build java dest project.");

        let build_cmd = format!("chmod a+x ./gradlew && ./gradlew aR");

        let output = Command::new("sh")
            .arg("-c")
            .arg(&build_cmd)
            .current_dir(self.dest_prj_path)
            .output()?;

        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;

        if !output.status.success() {
            return Err(CommandError(format!("run building java dest project failed.")).into());
        }

        let options = CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 1024,
            copy_inside: true,
            depth: 65535,
        };

        let src_arr = self
            .dest_prj_path
            .join("rustlib")
            .join("build")
            .join("outputs")
            .join("aar")
            .join("rustlib-release.aar");
        let target = self.origin_prj_path.join("target").join("android");
        if target.exists() {
            fs::remove_dir_all(&target).unwrap();
        }
        fs::create_dir_all(&target).unwrap();

        fs_extra::copy_items(&vec![src_arr], &target, &options)
            .map_err(|e| FileError(format!("copy android bridge outputs failed. {:?}", e)))
            .unwrap();

        Ok(())
    }
}
