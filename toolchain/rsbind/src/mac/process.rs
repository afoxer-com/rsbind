use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use cbindgen::{Config, Language};

use fs_extra::dir::CopyOptions;

use crate::ast::AstResult;
use crate::base::process::BuildProcess;
use crate::bridge::prj::Unpack;
use crate::bridges::BridgeGen::CGen;
use crate::errors::ErrorKind::*;
use crate::errors::*;
use crate::swift::artifact::SwiftCodeGen;
use crate::unzip;

use super::config::Mac;

pub(crate) struct MacProcess<'a> {
    origin_prj_path: &'a Path,
    artifact_prj_path: &'a Path,
    bridge_prj_path: &'a Path,
    header_path: &'a Path,
    host_crate_name: &'a str,
    ast_result: &'a AstResult,
    config: Option<Mac>,
}

impl<'a> MacProcess<'a> {
    pub fn new(
        origin_prj_path: &'a Path,
        dest_prj_path: &'a Path,
        bridge_prj_path: &'a Path,
        header_path: &'a Path,
        host_crate_name: &'a str,
        ast_result: &'a AstResult,
        config: Option<Mac>,
    ) -> Self {
        MacProcess {
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

impl<'a> MacProcess<'a> {
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

impl<'a> MacProcess<'a> {
    fn lib_name(&self) -> String {
        format!(
            "lib{}_mac_bridge_prj.a",
            &self.host_crate_name.replace('-', "_")
        )
    }

    fn config(&self) -> Mac {
        match self.config {
            Some(ref config) => config.to_owned(),
            None => Mac::default(),
        }
    }
}

impl<'a> BuildProcess for MacProcess<'a> {
    fn unpack(&self) -> Result<()> {
        Ok(())
    }

    fn gen_bridge_src(&self) -> Result<()> {
        println!("begin unzip rust template for mac");
        let buf: &[u8] = include_bytes!("res/template_bridge_mac.zip");
        let unpack = Unpack {
            path: self.bridge_prj_path,
            host_crate: self.host_crate_name,
            buf,
            features: &self.config().features(),
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
        println!("run building rust project for Mac");

        let debug_release = if self.config().is_release() {
            "release"
        } else {
            "debug"
        };

        let build_cmds = format!(
            "cargo build --lib {} --target-dir {} {}",
            self.config().release_str(),
            "target",
            &self.config().rustc_param()
        );

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
        println!("begin unzip mac template");
        if self.artifact_prj_path.exists() {
            fs::remove_dir_all(&self.artifact_prj_path)?;
        }
        fs::create_dir_all(&self.artifact_prj_path)?;
        let mac_template_buf: &[u8] = include_bytes!("res/template_mac.zip");
        unzip::unzip_to(mac_template_buf, self.artifact_prj_path)?;

        let parent = self
            .artifact_prj_path
            .parent()
            .ok_or_else(|| FileError("can't find parent dir for swift".to_string()))?;
        let swift_gen_path = parent.join("swift_gen");
        if swift_gen_path.exists() {
            fs::remove_dir_all(&swift_gen_path)?;
        }
        fs::create_dir_all(&swift_gen_path)?;

        SwiftCodeGen {
            swift_gen_dir: &swift_gen_path,
            ast: self.ast_result,
        }
        .gen_swift_code()?;
        // artifact::gen_swift_code(&self.artifact_prj_path, &self.ast_path, &self.bin_path)?;

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
                .map_err(|e| FileError(format!("copy Mac swift outputs failed. {:?}", e)))
                .unwrap();
        }

        Ok(())
    }

    fn build_artifact_prj(&self) -> Result<()> {
        Ok(())
    }
}
