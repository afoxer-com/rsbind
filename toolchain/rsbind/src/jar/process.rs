use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

use fs_extra::dir::CopyOptions;
use syn::__private::str;

use crate::ast::AstResult;
use crate::base::process::BuildProcess;
use crate::bridge::prj::Unpack;
use crate::bridges::BridgeGen::JavaGen;
use crate::errors::ErrorKind::*;
use crate::errors::*;
use crate::jar::arch::Arch;
use crate::jar::config::Jar;
use crate::java::artifact::JavaCodeGen;
use crate::ndk_tool::{build, BuildConfig};
use crate::unzip;

const MAGIC_NUM: &str = "*521%";

pub(crate) struct JarProcess<'a> {
    origin_prj_path: &'a Path,
    artifact_prj_path: &'a Path,
    bridge_prj_path: &'a Path,
    host_crate_name: &'a str,
    ast_result: &'a AstResult,
    config: Option<Jar>,
}

impl<'a> JarProcess<'a> {
    pub fn new(
        origin_prj_path: &'a Path,
        artifact_prj_path: &'a Path,
        bridge_prj_path: &'a Path,
        host_crate_name: &'a str,
        ast_result: &'a AstResult,
        config: Option<Jar>,
    ) -> Self {
        JarProcess {
            origin_prj_path,
            artifact_prj_path,
            bridge_prj_path,
            host_crate_name,
            ast_result,
            config,
        }
    }
}

impl<'a> JarProcess<'a> {
    fn lib_extension_name(&self) -> String {
        if cfg!(target_os = "windows") {
            "dll"
        } else if cfg!(target_os = "macos") {
            "dylib"
        } else {
            "so"
        }
        .to_string()
    }

    fn lib_name(&self) -> String {
        format!(
            "lib{}_jar_bridge_prj.{}",
            &self.host_crate_name.replace('-', "_"),
            self.lib_extension_name()
        )
    }

    fn config(&self) -> Jar {
        match self.config {
            Some(ref config) => config.to_owned(),
            None => Jar::default(),
        }
    }
}

impl<'a> BuildProcess for JarProcess<'a> {
    fn unpack(&self) -> Result<()> {
        Ok(())
    }

    fn gen_bridge_src(&self) -> Result<()> {
        println!("begin unzip rust template for android");
        // unpack the bridge project.
        {
            let buf: &[u8] = include_bytes!("res/template_bridge_jar.zip");
            let unpack = Unpack {
                path: self.bridge_prj_path,
                host_crate: self.host_crate_name,
                buf,
                features: &self.config().features(),
            };

            unpack.unpack()?;
        }

        let bridge_c_src_path = self.bridge_prj_path.join("src").join("java").join("bridge");
        fs::create_dir_all(&bridge_c_src_path)?;
        JavaGen(
            self.host_crate_name.to_owned(),
            self.ast_result,
            &bridge_c_src_path,
            self.config().namespace(),
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
        println!("building jar bridge project");
        let mut build_cmds = format!(
            "cargo build --lib {} --target-dir {} {}",
            self.config().release_str(),
            "target",
            &self.config().rustc_param()
        );

        let features = self.config().features();
        if !features.is_empty() {
            let feat = features.join(",");
            build_cmds = format!("{} --features {}", &build_cmds, &feat);
        }

        if let Some(rustc_param) = self.config().rustc_param {
            build_cmds = format!(
                "{} {}",
                &build_cmds,
                &rustc_param
            );
        }

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
        let options = CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 1024,
            copy_inside: true,
            content_only: false,
            depth: 65535,
        };

        println!("copy output files to jar project.");

        let debug_release = if self.config().is_release() {
            "release"
        } else {
            "debug"
        };

        let lib_file = self
            .bridge_prj_path
            .join("target")
            .join(debug_release)
            .join(&self.lib_name());

        let lib_artifact = self
            .artifact_prj_path
            .join("rustlib")
            .join("src")
            .join("main")
            .join("resources")
            .join("natives")
            .join(Arch::from_env().to_string());

        if !lib_artifact.exists() {
            std::fs::create_dir_all(&lib_artifact)?;
        }

        println!("copying {:?} --> {:?}", lib_file, lib_artifact);

        fs_extra::copy_items(&[lib_file], &lib_artifact, &options)
            .map_err(|e| FileError(format!("copy android bridge outputs failed. {:?}", e)))?;
        fs::rename(
            &lib_artifact.join(&self.lib_name()),
            &lib_artifact.join(format!(
                "lib{}.{}",
                &self.config().dylib_name(),
                self.lib_extension_name()
            )),
        )?;

        Ok(())
    }

    fn gen_artifact_code(&self) -> Result<()> {
        // unpack the artifact java project
        {
            println!("begin unzip jar template");
            if self.artifact_prj_path.exists() {
                fs::remove_dir_all(&self.artifact_prj_path).unwrap();
            }
            fs::create_dir_all(&self.artifact_prj_path).unwrap();
            let jar_template_buf: &[u8] = include_bytes!("res/template_jar.zip");
            unzip::unzip_to(jar_template_buf, self.artifact_prj_path).unwrap();
        }

        println!("generate java code.");
        let parent = self
            .artifact_prj_path
            .parent()
            .ok_or_else(|| FileError("can't find parent dir for java".to_string()))?;
        let java_gen_path = parent.join("java_gen");
        if java_gen_path.exists() {
            fs::remove_dir_all(&java_gen_path)?;
        }
        fs::create_dir_all(&java_gen_path)?;

        JavaCodeGen {
            java_gen_dir: &java_gen_path,
            ast: self.ast_result,
            namespace: self.config().namespace(),
            so_name: self.config().dylib_name(),
            ext_libs: self.config().ext_libs(),
        }
        .gen_java_code()?;

        // get the output dir string
        println!("get output dir string");
        let mut output_dir = self
            .artifact_prj_path
            .join("rustlib")
            .join("src")
            .join("main")
            .join("java");

        fs::create_dir_all(&output_dir).unwrap();

        let namespace = self.config().namespace();
        let pkg_split = namespace.split('.').collect::<Vec<&str>>();
        for pkg_part in pkg_split.iter() {
            output_dir = output_dir.join(pkg_part);
        }

        if output_dir.exists() {
            fs::remove_dir_all(&output_dir).unwrap();
        }

        let options = CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 1024,
            copy_inside: true,
            content_only: false,
            depth: 65535,
        };

        fs_extra::copy_items(&[java_gen_path], &output_dir, &options)
            .map_err(|e| FileError(format!("copy android bridge outputs failed. {:?}", e)))
            .unwrap();

        Ok(())
    }

    fn build_artifact_prj(&self) -> Result<()> {
        println!("build java artifact project.");

        let build_cmd = "chmod a+x ./gradlew && ./gradlew assemble".to_string();

        let output = Command::new("sh")
            .arg("-c")
            .arg(&build_cmd)
            .current_dir(self.artifact_prj_path)
            .output()?;

        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;

        if !output.status.success() {
            return Err(
                CommandError("run building java artifact project failed.".to_string()).into(),
            );
        }

        let options = CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 1024,
            copy_inside: true,
            content_only: false,
            depth: 65535,
        };

        let src_arr = self
            .artifact_prj_path
            .join("rustlib")
            .join("build")
            .join("libs")
            .join("rustlib.jar");
        let target = self.origin_prj_path.join("target").join("jar");
        if target.exists() {
            fs::remove_dir_all(&target).unwrap();
        }
        fs::create_dir_all(&target).unwrap();

        fs_extra::copy_items(&[src_arr], &target, &options)
            .map_err(|e| FileError(format!("copy android bridge outputs failed. {:?}", e)))
            .unwrap();

        Ok(())
    }
}
