use crate::base::lang::LangGen;
use crate::bridge::prj::Unpack;
use crate::errors::ErrorKind::CommandError;
use crate::js::JsGen;
use crate::nodejs::config::NodeJS;
use crate::ErrorKind::FileError;
use crate::{unzip, AstResult, BuildProcess};
use fs_extra::dir::CopyOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::{fs, io};

const MAGIC_NUM: &str = "*521%";

pub(crate) struct NodeJSProcess<'a> {
    pub(crate) origin_prj_path: &'a Path,
    pub(crate) artifact_prj_path: &'a Path,
    pub(crate) bridge_prj_path: &'a Path,
    pub(crate) host_crate_name: &'a str,
    pub(crate) ast_result: &'a AstResult,
    pub(crate) config: Option<NodeJS>,
}

impl<'a> NodeJSProcess<'a> {
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
            "lib{}_nodejs_bridge_prj.{}",
            &self.host_crate_name.replace('-', "_"),
            self.lib_extension_name()
        )
    }
}

impl<'a> BuildProcess for NodeJSProcess<'a> {
    fn gen_bridge_src(&self) -> crate::Result<()> {
        println!("begin unzip rust template for android");
        // unpack the bridge project.
        {
            let buf: &[u8] = include_bytes!("res/template_bridge_nodejs.zip");
            let features: Vec<String> = Vec::new();
            let unpack = Unpack {
                path: self.bridge_prj_path,
                host_crate: self.host_crate_name,
                buf,
                features: &features,
            };

            unpack.unpack()?;
        }

        let bridge_c_src_path = self.bridge_prj_path.join("src");
        fs::create_dir_all(&bridge_c_src_path)?;

        JsGen {
            crate_name: self.host_crate_name.to_string(),
            ast: self.ast_result.clone(),
        }
        .gen_bridge(&bridge_c_src_path)?;

        let _ = Command::new("cargo")
            .arg("fmt")
            .current_dir(&self.bridge_prj_path)
            .output();

        Ok(())
    }

    fn build_bridge_prj(&self) -> crate::Result<()> {
        println!("building node.js bridge project");
        let mut build_cmds = format!("cargo build --lib --release --target-dir target");
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

    fn copy_bridge_outputs(&self) -> crate::Result<()> {
        let options = CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 1024,
            copy_inside: true,
            content_only: false,
            depth: 65535,
        };

        println!("copy output files to node.js project.");

        let lib_file = self
            .bridge_prj_path
            .join("target")
            .join("release")
            .join(&self.lib_name());

        let lib_artifact = self.artifact_prj_path.join("dist");

        if !lib_artifact.exists() {
            std::fs::create_dir_all(&lib_artifact)?;
        }

        println!("copying {:?} --> {:?}", lib_file, lib_artifact);

        fs_extra::copy_items(&[lib_file], &lib_artifact, &options)
            .map_err(|e| FileError(format!("copy node.js bridge outputs failed. {:?}", e)))?;
        fs::rename(
            &lib_artifact.join(&self.lib_name()),
            &lib_artifact.join(format!("rustlib.node")),
        )?;

        Ok(())
    }

    fn gen_artifact_code(&self) -> crate::Result<()> {
        // unpack the artifact nodejs project
        {
            println!("begin unzip nodejs template");
            if self.artifact_prj_path.exists() {
                fs::remove_dir_all(&self.artifact_prj_path).unwrap();
            }
            fs::create_dir_all(&self.artifact_prj_path).unwrap();
            let nodejs_template_buf: &[u8] = include_bytes!("res/template_nodejs.zip");
            unzip::unzip_to(nodejs_template_buf, self.artifact_prj_path).unwrap();

            let manifest_path = self.artifact_prj_path.join("package.json");
            let manifest_text = fs::read_to_string(&manifest_path).map_err(|e| {
                FileError(format!(
                    "read nodejs artifact project AndroidManifest.xml error: {:?}",
                    e
                ))
            })?;
            let replaced =
                manifest_text.replace(&format!("$({}-crate)", MAGIC_NUM), &self.host_crate_name);
            fs::write(manifest_path, replaced).map_err(|e| {
                FileError(format!(
                    "write nodejs artifact project AndroidManifest  error {:?}",
                    e
                ))
            })?;
        }

        let js_gen_path = self.artifact_prj_path.join("src");
        if !js_gen_path.exists() {
            std::fs::create_dir_all(&js_gen_path)?;
        }
        JsGen {
            crate_name: self.host_crate_name.to_string(),
            ast: self.ast_result.clone(),
        }
        .gen_native(&js_gen_path)?;
        Ok(())
    }

    fn build_artifact_prj(&self) -> crate::Result<()> {
        println!("build node.js artifact project.");

        let build_cmd = "npm install && npm run build".to_string();

        let output = Command::new("sh")
            .arg("-c")
            .arg(&build_cmd)
            .current_dir(self.artifact_prj_path)
            .output()?;

        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;

        if !output.status.success() {
            return Err(
                CommandError("run building node.js artifact project failed.".to_string()).into(),
            );
        }

        Ok(())
    }
}
