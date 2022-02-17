#![recursion_limit = "128"]
extern crate syn;
#[macro_use]
extern crate quote;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate cbindgen;
extern crate fs_extra;
extern crate proc_macro2;
extern crate serde;
extern crate toml;
extern crate zip;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate rsgen;
extern crate ndk_build;
extern crate ndk_tool;

mod android;
mod ast;
mod bridge;
mod bridges;
mod cargo;
mod config;
mod errors;
mod ios;
mod process;
mod test;
mod unzip;

use android::config::Android;
use android::process::AndroidProcess;
use ast::AstResult;
use errors::*;
use ios::config::Ios;
use ios::process::IosProcess;
use process::*;
use std::fs;
use std::path::PathBuf;

const GEN_DIR_NAME: &str = "_gen";
const HEADER_NAME: &str = "header";
const AST_DIR: &str = "ast";
const IOS_PROJ: &str = "ios_artifact";
const IOS_BRIDGE_PROJ: &str = "ios_bridge";
const ANDROID_BRIDGE_PROJ: &str = "android_bridge";
const ANDROID_PROJ: &str = "android_artifact";
const BIN_DIR: &str = "bin";

pub struct Bind {
    prj_path: PathBuf,
    ios_artifact_path: PathBuf,
    ios_bridge_path: PathBuf,
    android_bridge_path: PathBuf,
    android_dest_path: PathBuf,
    header_path: PathBuf,
    ast_path: PathBuf,
    bin_path: PathBuf,
    target: Target,
    action: Action,
}

pub enum Target {
    Android,
    Ios,
    All,
}

pub enum Action {
    /// Parse src/contract and src/imp, generate simplified ast json file to _gen/ast.
    GenAst,
    /// Generate rust bridge code to _gen/ios_bridge or _gen/android_bridge, for iOS it's c ffi functions, for android it's jni ffi functions.
    GenBridge,
    /// Generate code in artifact(iOS framework or android aar)
    GenArtifactCode,
    /// Generate c header file
    GenCHeader,
    /// Build and generate artifact.
    BuildArtifact,
    /// Do all the process and generate artifacts.
    All,
}

impl Bind {
    ///
    /// crate the object for binding.
    /// * prject_path: the rust project we need to bind
    /// * target: which target we want to generate, android or iOS
    ///
    pub fn from(prj_path: String, target: Target, action: Action) -> Bind {
        let root = PathBuf::from(&prj_path);

        // ./_gen/ast
        let ast_path = root.join(GEN_DIR_NAME).join(AST_DIR);

        // ./_gen/bin/
        let bin_path = root.join(GEN_DIR_NAME).join(BIN_DIR);

        // ./_gen/header/
        let header_path = root.join(GEN_DIR_NAME).join(HEADER_NAME);

        // ./_gen/ios_artifact/
        let ios_artifact_path = root.join(GEN_DIR_NAME).join(IOS_PROJ);

        // ./_gen/ios_bridge
        let ios_bridge_path = root.join(GEN_DIR_NAME).join(IOS_BRIDGE_PROJ);

        // ./_gen/android_bridge
        let android_bridge_path = root.join(GEN_DIR_NAME).join(ANDROID_BRIDGE_PROJ);

        let android_artifact_path = root.join(GEN_DIR_NAME).join(ANDROID_PROJ);

        Bind {
            prj_path: root,
            ios_artifact_path,
            ios_bridge_path,
            android_bridge_path,
            android_dest_path: android_artifact_path,
            header_path,
            ast_path,
            bin_path,
            target,
            action,
        }
    }

    ///
    /// generate the ios framework and android aar as per the target config
    ///
    pub fn gen_all(&self) -> Result<()> {
        let config = config::parse(&self.prj_path);
        println!("rsbind config in {:?} is {:?}", &self.prj_path, config);

        let crate_name = self.parse_crate_name()?;

        if let Action::GenAst = self.action {
            self.parse_ast(crate_name.clone())?;
            return Ok(());
        }

        match self.target {
            Target::Ios => {
                let ast = &self.get_ast_if_need(crate_name.clone())?;
                self.gen_for_ios(&crate_name, ast, config.clone())?;
            }
            Target::Android => {
                let ast = &self.get_ast_if_need(crate_name.clone())?;
                self.gen_for_android(&crate_name, ast, config.clone())?;
            }
            Target::All => {
                let ast_result = self.get_ast_if_need(crate_name.clone())?;
                self.gen_for_ios(&crate_name, &ast_result, config.clone())?;
                self.gen_for_android(&crate_name, &ast_result, config.clone())?;
            }
        };
        Ok(())
    }

    fn get_ast_if_need(&self, crate_name: String) -> Result<AstResult> {
        match self.action {
            Action::GenBridge | Action::GenArtifactCode | Action::All => {
                self.parse_ast(crate_name.clone())
            }
            _ => {
                use std::collections::HashMap;
                let ast_result = AstResult {
                    trait_descs: HashMap::new(),
                    struct_descs: HashMap::new(),
                    imp_desc: vec![],
                };
                Ok(ast_result)
            }
        }
    }

    fn parse_ast(&self, crate_name: String) -> Result<AstResult> {
        let prj_path = PathBuf::from(&self.prj_path);
        if self.ast_path.exists() {
            fs::remove_dir_all(&self.ast_path)?;
        }
        fs::create_dir_all(&self.ast_path)?;
        ast::AstHandler::new(crate_name)
            .parse(&prj_path)?
            .flush(&self.ast_path)
    }

    ///
    /// generate the ios framework
    fn gen_for_ios(
        &self,
        crate_name: &str,
        ast_result: &AstResult,
        config: Option<config::Config>,
    ) -> Result<()> {
        let ios = match config {
            Some(ref config) => config.ios.clone(),
            None => Some(Ios::default()),
        };

        let ios_process = IosProcess::new(
            &self.prj_path,
            &self.ios_artifact_path,
            &self.ios_bridge_path,
            &self.header_path,
            &self.ast_path,
            &self.bin_path,
            crate_name,
            &ast_result,
            ios,
        );

        match self.action {
            Action::GenAst => (),
            Action::GenBridge => ios_process.gen_bridge_src()?,
            Action::GenArtifactCode => ios_process.gen_artifact_code()?,
            Action::GenCHeader => ios_process.gen_c_header()?,
            Action::BuildArtifact => {
                ios_process.build_bridge_prj()?;
                ios_process.copy_bridge_outputs()?;
                // we don't generate artifact now. TODO generate cocoapods lib
                // ios_process.build_artifact_prj()?;
            }
            Action::All => {
                ios_process.gen_bridge_src()?;
                ios_process.gen_artifact_code()?;
                ios_process.build_bridge_prj()?;
                ios_process.copy_bridge_outputs()?;
                // we don't generate artifact now. TODO generate cocoapods lib
                // ios_process.build_artifact_prj()?;
            }
        }

        Ok(())
    }

    ///
    /// generate the android aar
    ///
    fn gen_for_android(
        &self,
        crate_name: &str,
        ast_result: &AstResult,
        config: Option<config::Config>,
    ) -> Result<()> {
        let android = match config {
            Some(ref config) => config.android.clone(),
            None => Some(Android::default()),
        };

        let android_process = AndroidProcess::new(
            &self.prj_path,
            &self.android_dest_path,
            &self.android_bridge_path,
            &self.ast_path,
            &self.bin_path,
            crate_name,
            ast_result,
            android,
            ast_result,
        );

        match self.action {
            Action::GenAst => (),
            Action::GenBridge => android_process.gen_bridge_src()?,
            Action::GenArtifactCode => android_process.gen_artifact_code()?,
            Action::GenCHeader => (),
            Action::BuildArtifact => {
                android_process.build_bridge_prj()?;
                android_process.copy_bridge_outputs()?;
                android_process.build_artifact_prj()?;
            }
            Action::All => {
                android_process.gen_bridge_src()?;
                android_process.gen_artifact_code()?;
                android_process.build_bridge_prj()?;
                android_process.copy_bridge_outputs()?;
                android_process.build_artifact_prj()?;
            }
        };

        Ok(())
    }

    ///
    /// parse the crate name of origin project from Cargo.toml
    ///
    fn parse_crate_name(&self) -> Result<String> {
        let toml_path = PathBuf::from(&self.prj_path).join("Cargo.toml");
        let manifest = cargo::manifest(toml_path.as_path())?;
        println!("parse project name = {}", manifest.package.name);
        Ok(manifest.package.name)
    }
}
