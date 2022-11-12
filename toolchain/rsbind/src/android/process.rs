use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

use fs_extra::dir::CopyOptions;
use syn::__private::str;

use crate::ast::AstResult;
use crate::base::lang::LangGen;
use crate::base::process::BuildProcess;
use crate::bridge::prj::Unpack;
use crate::errors::ErrorKind::*;
use crate::errors::*;
use crate::java::JavaGen;
use crate::ndk_tool::{build, BuildConfig};
use crate::unzip;

use super::config::Android;

const MAGIC_NUM: &str = "*521%";

pub(crate) struct AndroidProcess<'a> {
    origin_prj_path: &'a Path,
    artifact_prj_path: &'a Path,
    bridge_prj_path: &'a Path,
    host_crate_name: &'a str,
    ast_result: &'a AstResult,
    config: Option<Android>,
}

impl<'a> AndroidProcess<'a> {
    pub fn new(
        origin_prj_path: &'a Path,
        artifact_prj_path: &'a Path,
        bridge_prj_path: &'a Path,
        host_crate_name: &'a str,
        ast_result: &'a AstResult,
        config: Option<Android>,
    ) -> Self {
        AndroidProcess {
            origin_prj_path,
            artifact_prj_path,
            bridge_prj_path,
            host_crate_name,
            ast_result,
            config,
        }
    }
}

impl<'a> AndroidProcess<'a> {
    fn lib_name(&self) -> String {
        format!(
            "lib{}_android_bridge_prj.so",
            &self.host_crate_name.replace('-', "_")
        )
    }

    fn config(&self) -> Android {
        match self.config {
            Some(ref config) => config.to_owned(),
            None => Android::default(),
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
                features: &self.config().features(),
            };

            unpack.unpack()?;
        }

        let bridge_c_src_path = self.bridge_prj_path.join("src");
        fs::create_dir_all(&bridge_c_src_path)?;

        JavaGen {
            crate_name: self.host_crate_name.to_string(),
            ast: self.ast_result.clone(),
            namespace: self.config().namespace(),
            so_name: self.config().so_name(),
            ext_libs: self.config().ext_libs(),
        }
        .gen_bridge(&bridge_c_src_path)?;

        let _ = Command::new("cargo")
            .arg("fmt")
            .current_dir(&self.bridge_prj_path)
            .output();

        Ok(())
    }

    fn build_bridge_prj(&self) -> Result<()> {
        println!("building android bridge project");
        let _ndk = ndk_build::ndk::Ndk::from_env()?;
        let archs = self.config().archs();

        let config = BuildConfig {
            lib_name: self.lib_name(),
            arch_list: archs,
            is_release: self.config().is_release(),
            target_dir: "target".to_string(),
            project_dir: self.bridge_prj_path.to_path_buf(),
            sdk_version: 21,
            rustc_param: self.config().rustc_param(),
            features_def: self.config().features(),
        };

        build(&config)?;

        Ok(())
    }

    fn copy_bridge_outputs(&self) -> Result<()> {
        let mut copy_map = HashMap::new();
        copy_map.insert("arm-linux-androideabi", "armeabi");
        copy_map.insert("armv7-linux-androideabi", "armeabi-v7a");
        copy_map.insert("aarch64-linux-android", "arm64-v8a");
        copy_map.insert("i686-linux-android", "x86");
        copy_map.insert("x86_64-linux-android", "x86_64");

        let options = CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 1024,
            copy_inside: true,
            content_only: false,
            depth: 65535,
        };

        println!("copy output files to android project.");

        let debug_release = if self.config().is_release() {
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

            let armeabi_artifact = self
                .artifact_prj_path
                .join("rustlib")
                .join("src")
                .join("main")
                .join("jniLibs")
                .join(entry.1);
            if !armeabi_artifact.exists() {
                std::fs::create_dir_all(&armeabi_artifact)?;
            }

            println!("copying {:?} --> {:?}", armeabi_src, armeabi_artifact);

            fs_extra::copy_items(&[armeabi_src], &armeabi_artifact, &options)
                .map_err(|e| FileError(format!("copy android bridge outputs failed. {:?}", e)))?;
            fs::rename(
                &armeabi_artifact.join(&self.lib_name()),
                &armeabi_artifact.join(format!("lib{}.so", &self.config().so_name())),
            )?;
        }

        Ok(())
    }

    fn gen_artifact_code(&self) -> Result<()> {
        // unpack the artifact java project
        {
            println!("begin unzip android template");
            if self.artifact_prj_path.exists() {
                fs::remove_dir_all(&self.artifact_prj_path).unwrap();
            }
            fs::create_dir_all(&self.artifact_prj_path).unwrap();
            let android_template_buf: &[u8] = include_bytes!("res/template_android.zip");
            unzip::unzip_to(android_template_buf, self.artifact_prj_path).unwrap();

            let manifest_path = self
                .artifact_prj_path
                .join("rustlib")
                .join("src")
                .join("main")
                .join("AndroidManifest.xml");
            let manifest_text = fs::read_to_string(&manifest_path).map_err(|e| {
                FileError(format!(
                    "read android artifact project AndroidManifest.xml error: {:?}",
                    e
                ))
            })?;
            let replaced = manifest_text.replace(
                &format!("$({}-namespace)", MAGIC_NUM),
                &self.config().namespace(),
            );
            fs::write(manifest_path, replaced).map_err(|e| {
                FileError(format!(
                    "write android artifact project AndroidManifest  error {:?}",
                    e
                ))
            })?;
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

        JavaGen {
            crate_name: self.host_crate_name.to_string(),
            ast: self.ast_result.clone(),
            namespace: self.config().namespace(),
            so_name: self.config().so_name(),
            ext_libs: self.config().ext_libs(),
        }
        .gen_native(&java_gen_path)?;

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

        let build_cmd = "chmod a+x ./gradlew && ./gradlew aR".to_string();

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
            .join("outputs")
            .join("aar")
            .join("rustlib-release.aar");
        let target = self.origin_prj_path.join("target").join("android");
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
