#![recursion_limit = "128"]
extern crate cbindgen;
#[macro_use]
extern crate error_chain;
extern crate fs_extra;
extern crate ndk_build;
extern crate ndk_tool;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate rstgen;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate core;
extern crate serde_json;
extern crate syn;
extern crate toml;
extern crate zip;
extern crate heck;

use std::fs;
use std::path::PathBuf;

use crate::android::config::Android;
use crate::android::process::AndroidProcess;
use crate::ast::AstResult;
use crate::base::process::*;
use crate::errors::*;
use crate::ios::config::Ios;
use crate::ios::process::IosProcess;
use crate::jar::config::Jar;
use crate::jar::process::JarProcess;
use crate::mac::config::Mac;
use crate::mac::process::MacProcess;

mod android;
mod ast;
mod base;
mod bridge;
mod bridges;
mod cargo;
mod config;
mod errors;
mod ios;
mod jar;
mod java;
mod mac;
mod swift;
mod test;
mod unzip;

const GEN_DIR_NAME: &str = "_gen";
const HEADER_NAME: &str = "header";
const AST_DIR: &str = "ast";
const IOS_PROJ: &str = "ios_artifact";
const IOS_BRIDGE_PROJ: &str = "ios_bridge";
const MAC_PROJ: &str = "mac_artifact";
const MAC_BRIDGE_PROJ: &str = "mac_bridge";
const ANDROID_BRIDGE_PROJ: &str = "android_bridge";
const ANDROID_PROJ: &str = "android_artifact";
const JAR_BRIDGE_PROJ: &str = "jar_bridge";
const JAR_PROJ: &str = "jar_artifact";

pub struct Bind {
    prj_path: PathBuf,
    ios_artifact_path: PathBuf,
    ios_bridge_path: PathBuf,
    mac_artifact_path: PathBuf,
    mac_bridge_path: PathBuf,
    android_bridge_path: PathBuf,
    android_artifact_path: PathBuf,
    jar_bridge_path: PathBuf,
    jar_artifact_path: PathBuf,
    header_path: PathBuf,
    ast_path: PathBuf,
    target: Target,
    action: Action,
}

pub enum Target {
    Android,
    Ios,
    Mac,
    Jar,
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

        // ./_gen/header/
        let header_path = root.join(GEN_DIR_NAME).join(HEADER_NAME);

        // ./_gen/ios_artifact/
        let ios_artifact_path = root.join(GEN_DIR_NAME).join(IOS_PROJ);

        // ./_gen/ios_bridge
        let ios_bridge_path = root.join(GEN_DIR_NAME).join(IOS_BRIDGE_PROJ);

        // ./_gen/mac_artifact/
        let mac_artifact_path = root.join(GEN_DIR_NAME).join(MAC_PROJ);

        // ./_gen/mac_bridge
        let mac_bridge_path = root.join(GEN_DIR_NAME).join(MAC_BRIDGE_PROJ);

        // ./_gen/android_bridge
        let android_bridge_path = root.join(GEN_DIR_NAME).join(ANDROID_BRIDGE_PROJ);

        let android_artifact_path = root.join(GEN_DIR_NAME).join(ANDROID_PROJ);

        // ./_gen/jar_bridge
        let jar_bridge_path = root.join(GEN_DIR_NAME).join(JAR_BRIDGE_PROJ);

        let jar_artifact_path = root.join(GEN_DIR_NAME).join(JAR_PROJ);

        Bind {
            prj_path: root,
            ios_artifact_path,
            ios_bridge_path,
            mac_artifact_path,
            mac_bridge_path,
            android_bridge_path,
            android_artifact_path,
            jar_bridge_path,
            jar_artifact_path,
            header_path,
            ast_path,
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
            self.parse_ast(crate_name)?;
            return Ok(());
        }

        let ast = &self.get_ast_if_need(crate_name.clone())?;
        match self.target {
            Target::Ios => {
                self.gen_for_ios(&crate_name, ast, config)?;
            }
            Target::Android => {
                self.gen_for_android(&crate_name, ast, config)?;
            }
            Target::Mac => {
                self.gen_for_mac(&crate_name, ast, config)?;
            }
            Target::Jar => {
                self.gen_for_jar(&crate_name, ast, config)?;
            }
            Target::All => {
                self.gen_for_ios(&crate_name, ast, config.clone())?;
                self.gen_for_android(&crate_name, ast, config.clone())?;
                self.gen_for_mac(&crate_name, ast, config.clone())?;
                self.gen_for_jar(&crate_name, ast, config)?;
            }
        };
        Ok(())
    }

    fn get_ast_if_need(&self, crate_name: String) -> Result<AstResult> {
        match self.action {
            Action::GenBridge | Action::GenArtifactCode | Action::All => self.parse_ast(crate_name),
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
    /// generate the jar framework
    fn gen_for_jar(
        &self,
        crate_name: &str,
        ast_result: &AstResult,
        config: Option<config::Config>,
    ) -> Result<()> {
        let jar = match config {
            Some(ref config) => config.jar.clone(),
            None => Some(Jar::default()),
        };

        let jar_process = JarProcess::new(
            &self.prj_path,
            &self.jar_artifact_path,
            &self.jar_bridge_path,
            crate_name,
            ast_result,
            jar,
        );

        match self.action {
            Action::GenAst => (),
            Action::GenBridge => jar_process.gen_bridge_src()?,
            Action::GenArtifactCode => jar_process.gen_artifact_code()?,
            Action::GenCHeader => {}
            Action::BuildArtifact => {
                jar_process.build_bridge_prj()?;
                jar_process.copy_bridge_outputs()?;
                jar_process.build_artifact_prj()?;
            }
            Action::All => {
                jar_process.gen_bridge_src()?;
                jar_process.gen_artifact_code()?;
                jar_process.build_bridge_prj()?;
                jar_process.copy_bridge_outputs()?;
                jar_process.build_artifact_prj()?;
            }
        }

        Ok(())
    }

    ///
    /// generate the mac framework
    fn gen_for_mac(
        &self,
        crate_name: &str,
        ast_result: &AstResult,
        config: Option<config::Config>,
    ) -> Result<()> {
        let mac = match config {
            Some(ref config) => config.mac.clone(),
            None => Some(Mac::default()),
        };

        let mac_process = MacProcess::new(
            &self.prj_path,
            &self.mac_artifact_path,
            &self.mac_bridge_path,
            &self.header_path,
            crate_name,
            ast_result,
            mac,
        );

        match self.action {
            Action::GenAst => (),
            Action::GenBridge => mac_process.gen_bridge_src()?,
            Action::GenArtifactCode => mac_process.gen_artifact_code()?,
            Action::GenCHeader => mac_process.gen_c_header()?,
            Action::BuildArtifact => {
                mac_process.build_bridge_prj()?;
                mac_process.copy_bridge_outputs()?;
                // we don't generate artifact now. TODO generate cocoapods lib
                // ios_process.build_artifact_prj()?;
            }
            Action::All => {
                mac_process.gen_bridge_src()?;
                mac_process.gen_artifact_code()?;
                mac_process.build_bridge_prj()?;
                mac_process.copy_bridge_outputs()?;
                // we don't generate artifact now. TODO generate cocoapods lib
                // ios_process.build_artifact_prj()?;
            }
        }

        Ok(())
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
            crate_name,
            ast_result,
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
            &self.android_artifact_path,
            &self.android_bridge_path,
            crate_name,
            ast_result,
            android,
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
