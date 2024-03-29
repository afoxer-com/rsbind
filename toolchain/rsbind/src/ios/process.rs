use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use cbindgen::{Config, Language};
use fs_extra::dir::CopyOptions;

use crate::ast::AstResult;
use crate::base::lang::LangGen;
use crate::base::process::BuildProcess;
use crate::bridge::prj::Unpack;
use crate::errors::ErrorKind::*;
use crate::errors::*;
use crate::swift::SwiftGen;
use crate::unzip;

use super::config::Ios;

const IOS_ARCH: &str = "universal";

pub(crate) struct IosProcess<'a> {
    origin_prj_path: &'a Path,
    artifact_prj_path: &'a Path,
    bridge_prj_path: &'a Path,
    header_path: &'a Path,
    host_crate_name: &'a str,
    ast_result: &'a AstResult,
    config: Option<Ios>,
}

impl<'a> IosProcess<'a> {
    pub fn new(
        origin_prj_path: &'a Path,
        dest_prj_path: &'a Path,
        bridge_prj_path: &'a Path,
        header_path: &'a Path,
        host_crate_name: &'a str,
        ast_result: &'a AstResult,
        config: Option<Ios>,
    ) -> Self {
        IosProcess {
            origin_prj_path,
            artifact_prj_path: dest_prj_path,
            bridge_prj_path,
            header_path,
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
            &self.host_crate_name.replace('-', "_")
        )
    }

    fn config(&self) -> Ios {
        match self.config {
            Some(ref config) => config.to_owned(),
            None => Ios::default(),
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
            features: &self.config().features(),
        };

        unpack.unpack()?;

        let bridge_c_src_path = self.bridge_prj_path.join("src");
        fs::create_dir_all(&bridge_c_src_path)?;
        SwiftGen {
            crate_name: self.host_crate_name.to_string(),
            ast: self.ast_result.clone(),
        }
        .gen_bridge(&bridge_c_src_path)?;

        let _ = Command::new("cargo")
            .arg("fmt")
            .current_dir(&self.bridge_prj_path)
            .output();

        self.gen_c_header()
    }

    fn build_bridge_prj(&self) -> Result<()> {
        println!("run building rust project for iOS");

        let debug_release = if self.config().is_release() {
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
        let archs = self.config().archs();
        for arch in archs.iter() {
            let tmp = format!(
                "cargo rustc --target {}  --lib {} --target-dir {} {}",
                arch,
                self.config().release_str(),
                "target",
                &self.config().rustc_param()
            );
            build_cmds = format!("{} && {}", &build_cmds, &tmp);
            lipo_cmd = format!(
                "{} target/{}/{}/{}",
                &lipo_cmd,
                arch,
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
            return Err(CommandError("run build rust project build failed.".to_string()).into());
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
        let header_dest = self.artifact_prj_path.join("rustlib").join("Classes");
        let options = CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 1024,
            copy_inside: true,
            content_only: false,
            depth: 65535,
        };

        let debug_release = if self.config().is_release() {
            "release"
        } else {
            "debug"
        };

        fs_extra::copy_items(&[header_file], &header_dest, &options)
            .map_err(|e| FileError(format!("move header file error. {:?}", e)))?;

        let lib_file = self
            .bridge_prj_path
            .join("target")
            .join(IOS_ARCH)
            .join(debug_release)
            .join(&self.lib_name());

        let lib_artifact = self.artifact_prj_path.join("rustlib").join("Libraries");
        if !lib_artifact.exists() {
            fs::create_dir_all(&lib_artifact)?;
        }
        fs_extra::copy_items(&[lib_file], &lib_artifact, &options)
            .map_err(|e| FileError(format!("move lib file error. {:?}", e)))?;

        fs::rename(
            &lib_artifact.join(&self.lib_name()),
            &lib_artifact.join("libFfi.a"),
        )
        .map_err(|e| FileError(format!("rename libFfi.a failed. {:?}", e)))?;

        println!("copy output files to swift project over.");

        Ok(())
    }

    fn gen_artifact_code(&self) -> Result<()> {
        println!("begin unzip ios template");
        if self.artifact_prj_path.exists() {
            fs::remove_dir_all(&self.artifact_prj_path)?;
        }
        fs::create_dir_all(&self.artifact_prj_path)?;
        let ios_template_buf: &[u8] = include_bytes!("res/template_ios.zip");
        unzip::unzip_to(ios_template_buf, self.artifact_prj_path)?;

        let parent = self
            .artifact_prj_path
            .parent()
            .ok_or_else(|| FileError("can't find parent dir for swift".to_string()))?;
        let swift_gen_path = parent.join("swift_gen");
        if swift_gen_path.exists() {
            fs::remove_dir_all(&swift_gen_path)?;
        }
        fs::create_dir_all(&swift_gen_path)?;

        SwiftGen {
            crate_name: self.host_crate_name.to_string(),
            ast: self.ast_result.clone(),
        }
        .gen_native(&swift_gen_path)?;

        // get the output dir string
        println!("get output dir string");
        let output_dir = self.artifact_prj_path.join("rustlib").join("Classes");
        if output_dir.exists() {
            fs::remove_dir_all(&output_dir).unwrap();
        }
        fs::create_dir_all(&output_dir).unwrap();

        let options = CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 1024,
            copy_inside: true,
            content_only: false,
            depth: 65535,
        };
        let dir = fs::read_dir(&swift_gen_path)?;
        for file in dir {
            let path = file?.path();
            fs_extra::copy_items(&[path], &output_dir, &options)
                .map_err(|e| FileError(format!("copy iOS swift outputs failed. {:?}", e)))
                .unwrap();
        }

        Ok(())
    }

    /// No more framework any more.
    fn build_artifact_prj(&self) -> Result<()> {
        println!("run building swift project");

        // prj file
        let prj_file = self.artifact_prj_path.join("rustlib.xcodeproj");
        let prj_file_path = prj_file.canonicalize()?;
        let prj_file_str = prj_file_path
            .to_str()
            .ok_or_else(|| FileError("get xcodeproj path string error".to_string()))?;

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
            .ok_or_else(|| FileError("can't get ios outupt file string".to_string()))?;

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
            .ok_or_else(|| FileError("can't get ios outupt file string".to_string()))?;

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
            .ok_or_else(|| FileError("can't get ios outupt file string".to_string()))?;

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
            .current_dir(self.artifact_prj_path)
            .output()?;

        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;

        if !output.status.success() {
            return Err(
                CommandError("run archiving swift project build failed. ".to_string()).into(),
            );
        }
        Ok(())
    }
}
