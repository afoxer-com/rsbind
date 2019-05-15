use super::gen;
use ast::AstResult;
use bridges::BridgeGen::CGen;
use bridge::prj::Unpack;
use process::BuildProcess;
use cbindgen;
use cbindgen::{Config, Language};
use config::Config as BuildConfig;
use errors::ErrorKind::*;
use errors::*;
use fs_extra;
use fs_extra::dir::CopyOptions;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use unzip;

const IOS_ARCH: &str = "universal";
const PHONE_ARCHS: [&str; 2] = ["aarch64-apple-ios", "armv7-apple-ios"];
const SIMULATOR_ARCHS: [&str; 2] = ["i386-apple-ios", "x86_64-apple-ios"];

pub(crate) struct IosProcess<'a> {
    origin_prj_path: &'a PathBuf,
    dest_prj_path: &'a PathBuf,
    bridge_prj_path: &'a PathBuf,
    header_path: &'a PathBuf,
    ast_path: &'a PathBuf,
    bin_path: &'a PathBuf,
    host_crate_name: &'a str,
    ast_result: &'a AstResult,
    config: Option<BuildConfig>,
}

impl<'a> IosProcess<'a> {
    pub fn new(
        origin_prj_path: &'a PathBuf,
        dest_prj_path: &'a PathBuf,
        bridge_prj_path: &'a PathBuf,
        header_path: &'a PathBuf,
        ast_path: &'a PathBuf,
        bin_path: &'a PathBuf,
        host_crate_name: &'a str,
        ast_result: &'a AstResult,
        config: Option<BuildConfig>,
    ) -> Self {
        IosProcess {
            origin_prj_path,
            dest_prj_path,
            bridge_prj_path,
            header_path,
            ast_path,
            bin_path,
            host_crate_name,
            ast_result,
            config,
        }
    }
}

impl<'a> IosProcess<'a> {
    pub fn gen_c_header(&self) -> Result<()> {
        if self.header_path.exists() {
            fs::remove_dir_all(&self.header_path)?;
        }
        fs::create_dir_all(&self.header_path)?;

        let output_file = self.header_path.join("ffi.h").display().to_string();

        let config = Config {
            namespace: Some(String::from("ffi")),
            language: Language::C,
            ..Default::default()
        };

        let root_path = self.bridge_prj_path.to_str().unwrap();
        cbindgen::generate_with_config(root_path, config)?.write_to_file(&output_file);
        Ok(())
    }
}

impl<'a> IosProcess<'a> {
    fn lib_name(&self) -> String {
        format!(
            "lib{}_ios_bridge_prj.a",
            &self.host_crate_name.replace("-", "_")
        )
    }

    fn rustc_param(&self) -> String {
        match self.config.clone() {
            Some(config) => match config.ios {
                Some(ios) => match ios.rustc_param {
                    Some(rustc) => rustc,
                    None => "".to_owned(),
                },
                None => "".to_owned(),
            },
            None => "".to_owned(),
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
            Some(config) => match config.ios {
                Some(ios) => match ios.release {
                    Some(is_release) => is_release,
                    None => true,
                },
                None => true,
            },
            None => true,
        }
    }

    fn iphoneos_archs(&self) -> Vec<String> {
        let default_phone_archs = PHONE_ARCHS
            .to_vec()
            .into_iter()
            .map(|a| a.to_owned())
            .collect();
        match self.config.clone() {
            Some(config) => match config.ios {
                Some(ios) => match ios.arch_phone {
                    Some(arch) => arch,
                    None => default_phone_archs,
                },
                None => default_phone_archs,
            },
            None => default_phone_archs,
        }
    }

    fn simulator_archs(&self) -> Vec<String> {
        let default_phone_archs = SIMULATOR_ARCHS
            .to_vec()
            .into_iter()
            .map(|a| a.to_owned())
            .collect();
        match self.config.clone() {
            Some(config) => match config.ios {
                Some(ios) => match ios.arch_simu {
                    Some(arch) => arch,
                    None => default_phone_archs,
                },
                None => default_phone_archs,
            },
            None => default_phone_archs,
        }
    }

    fn features(&self) -> Vec<String> {
        match self.config.clone() {
            Some(config) => match config.ios {
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

impl<'a> BuildProcess for IosProcess<'a> {
    fn unpack(&self) -> Result<()> {
        Ok(())
    }

    fn gen_bridge_src(&self) -> Result<()> {
        println!("begin unzip rust template for ios");
        let buf: &[u8] = include_bytes!("res/template_bridge_ios.zip");
        let unpack = Unpack {
            path: self.bridge_prj_path,
            host_crate: self.host_crate_name,
            buf,
            features: &self.features(),
        };

        unpack.unpack()?;

        let bridge_c_src_path = self.bridge_prj_path.join("src").join("c").join("bridge");
        fs::create_dir_all(&bridge_c_src_path)?;
        CGen(
            self.host_crate_name.to_owned(),
            self.ast_result,
            &bridge_c_src_path,
        )
        .gen_bridges()?;

        let _ = Command::new("cargo")
            .arg("fmt")
            .current_dir(&self.bridge_prj_path)
            .output();

        self.gen_c_header()
    }

    fn build_bridge_prj(&self) -> Result<()> {
        println!("run building rust project for iOS");

        let debug_release = if self.is_release() {
            "release"
        } else {
            "debug"
        };

        let target_path = self
            .bridge_prj_path
            .join(format!("target/universal/{}/", debug_release));
        fs::create_dir_all(&target_path)?;

        let mut build_cmds = String::from("true");
        let mut lipo_cmd = format!(
            "lipo -create -output target/universal/{}/{}",
            debug_release,
            self.lib_name()
        );
        let iphoneos_archs = self.iphoneos_archs();
        for iphoneos_arch in iphoneos_archs.iter() {
            let tmp = format!(
                "cargo rustc --target {}  --lib {} --target-dir {} {}",
                iphoneos_arch,
                self.release_str(),
                "target",
                &self.rustc_param()
            );
            build_cmds = format!("{} && {}", &build_cmds, &tmp);
            lipo_cmd = format!(
                "{} target/{}/{}/{}",
                &lipo_cmd,
                iphoneos_arch,
                debug_release,
                self.lib_name()
            );
        }

        let simulator_archs = self.simulator_archs();
        for simulator_arch in simulator_archs.iter() {
            let tmp = format!(
                "cargo rustc --target {}  --lib {} --target-dir {} {}",
                simulator_arch,
                self.release_str(),
                "target",
                &self.rustc_param()
            );
            build_cmds = format!("{} && {}", &build_cmds, &tmp);
            lipo_cmd = format!(
                "{} target/{}/{}/{}",
                &lipo_cmd,
                simulator_arch,
                debug_release,
                self.lib_name()
            );
        }

        build_cmds = format!("{} && {}", &build_cmds, &lipo_cmd);

        println!("run building => {}", &build_cmds);

        let output = Command::new("sh")
            .arg("-c")
            .arg(build_cmds)
            .current_dir(self.bridge_prj_path)
            .output()?;

        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;

        if !output.status.success() {
            return Err(CommandError(format!("run build rust project build failed.",)).into());
        }

        println!("begin strip lib");
        let strip_result = Command::new("strip")
            .arg("-S")
            .arg(&format!(
                "target/universal/{}/{}",
                debug_release,
                self.lib_name()
            ))
            .current_dir(self.bridge_prj_path)
            .output();

        match strip_result {
            Err(err) => println!("strip error, err = {:?}", err),
            Ok(output) => {
                io::stdout().write_all(&output.stdout)?;
                io::stderr().write_all(&output.stderr)?;
            }
        }

        Ok(())
    }

    fn copy_bridge_outputs(&self) -> Result<()> {
        println!("copy output files to swift project.");

        let header_file = self.header_path.join("ffi.h");
        let header_dest = self.dest_prj_path.join("rustlib");
        let options = CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 1024,
            copy_inside: true,
            depth: 65535,
        };

        let debug_release = if self.is_release() {
            "release"
        } else {
            "debug"
        };

        fs_extra::copy_items(&vec![header_file], &header_dest, &options)
            .map_err(|e| FileError(format!("move header file error. {:?}", e)))?;

        let lib_file = self
            .bridge_prj_path
            .join("target")
            .join(IOS_ARCH)
            .join(debug_release)
            .join(&self.lib_name());

        let lib_dest = self.dest_prj_path.join("rustlib");
        fs_extra::copy_items(&vec![lib_file], &lib_dest, &options)
            .map_err(|e| FileError(format!("move lib file error. {:?}", e)))?;

        fs::rename(&lib_dest.join(&self.lib_name()), &lib_dest.join("ffi.a"))
            .map_err(|e| FileError(format!("rename ffi.a failed. {:?}", e)))?;

        println!("copy output files to swift project over.");

        Ok(())
    }

    fn gen_bind_code(&self) -> Result<()> {
        println!("begin unzip ios template");
        if self.dest_prj_path.exists() {
            fs::remove_dir_all(&self.dest_prj_path)?;
        }
        fs::create_dir_all(&self.dest_prj_path)?;
        let ios_template_buf: &[u8] = include_bytes!("res/template_ios.zip");
        unzip::unzip_to(ios_template_buf, &self.dest_prj_path)?;

        gen::gen_swift_code(&self.dest_prj_path, &self.ast_path, &self.bin_path)?;

        Ok(())
    }

    fn build_dest_prj(&self) -> Result<()> {
        println!("run building swift project");

        // prj file
        let prj_file = self.dest_prj_path.join("rustlib.xcodeproj");
        let prj_file_path = prj_file.canonicalize()?;
        let prj_file_str = prj_file_path
            .to_str()
            .ok_or(FileError("get xcodeproj path string error".to_string()))?;

        // simulator output dir
        let simu_output_dir = PathBuf::from(&self.origin_prj_path)
            .join("target")
            .join("iphone_simulator");
        if simu_output_dir.exists() {
            fs::remove_dir_all(&simu_output_dir)?;
        }
        fs::create_dir_all(&simu_output_dir)?;
        let simu_output_dir_path = simu_output_dir.canonicalize().unwrap();
        let simu_output_dir_str = simu_output_dir_path
            .to_str()
            .ok_or(FileError(format!("can't get ios outupt file string")))?;

        // iphoneos output dir
        let iphone_output_dir = PathBuf::from(&self.origin_prj_path)
            .join("target")
            .join("iphoneos");
        if iphone_output_dir.exists() {
            fs::remove_dir_all(&iphone_output_dir)?;
        }
        fs::create_dir_all(&iphone_output_dir)?;
        let iphone_output_dir_path = iphone_output_dir.canonicalize().unwrap();
        let iphone_output_dir_str = iphone_output_dir_path
            .to_str()
            .ok_or(FileError(format!("can't get ios outupt file string")))?;

        // universal output dir
        let universal_output_dir = PathBuf::from(&self.origin_prj_path)
            .join("target")
            .join("universal");
        if universal_output_dir.exists() {
            fs::remove_dir_all(&universal_output_dir)?;
        }
        fs::create_dir_all(&universal_output_dir)?;
        let universal_output_dir_path = universal_output_dir.canonicalize().unwrap();
        let universal_output_dir_str = universal_output_dir_path
            .to_str()
            .ok_or(FileError(format!("can't get ios outupt file string")))?;

        println!("archive swift path: {}", simu_output_dir_str);
        println!("archive swift path: {}", iphone_output_dir_str);
        println!("archive swift path: {}", universal_output_dir_str);

        let build_cmd1 = format!("xcodebuild -scheme rustlib -project {} -sdk iphonesimulator  -configuration Release CONFIGURATION_BUILD_DIR={} clean build", prj_file_str, simu_output_dir_str);
        let build_cmd2 = format!("xcodebuild -scheme rustlib -project {} -sdk iphoneos -configuration Release CONFIGURATION_BUILD_DIR={} clean build", prj_file_str, iphone_output_dir_str);
        let copy_cmd = format!(
            "cp -R {}/rustlib.framework {}",
            simu_output_dir_str, universal_output_dir_str
        );
        let build_cmd3 = format!("lipo -create \"{}/rustlib.framework/rustlib\" \"{}/rustlib.framework/rustlib\" -output \"{}/rustlib.framework/rustlib\"", simu_output_dir_str, iphone_output_dir_str, universal_output_dir_str);

        let build_cmd = format!(
            "{} && {} && {} && {}",
            &build_cmd1, &build_cmd2, &copy_cmd, &build_cmd3
        );

        let output = Command::new("sh")
            .arg("-c")
            .arg(&build_cmd)
            .current_dir(self.dest_prj_path)
            .output()?;

        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;

        if !output.status.success() {
            return Err(
                CommandError(format!("run archiving swift project build failed. ",)).into(),
            );
        }
        Ok(())
    }
}
