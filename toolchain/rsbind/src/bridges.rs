//!
//! module for generate C bridge files.
//! A C bridge file is exposed to callers of a foreign languages.
//!
use proc_macro2::{Ident, TokenStream};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::ast::contract::desc::*;
use crate::ast::imp::desc::*;
use crate::ast::AstResult;
use crate::errors::*;
use crate::ident;
use crate::java::bridge as android_bridge;
use crate::java::bridge::JavaGenStrategyImp;
use crate::swift::bridge as ios_bridge;
use crate::swift::bridge::CGenStrategyImp;

pub(crate) enum BridgeGen<'a> {
    /// create_name, ast, bridge_dir
    CGen(String, &'a AstResult, &'a PathBuf),
    /// create_name, ast, bridge_dir, namespace
    JavaGen(String, &'a AstResult, &'a PathBuf, String),
}

impl<'a> BridgeGen<'a> {
    ///
    /// parse the src/contract files then generate the c & java bridge files into c/bridge & java/bridge
    ///
    pub(crate) fn gen_bridges(&self) -> Result<()> {
        match *self {
            BridgeGen::CGen(ref crate_name, ast_result, bridge_dir) => {
                let mod_gen_strategy = CGenStrategyImp {};
                let mod_gen = BridgeModGen {
                    crate_name: crate_name.to_owned(),
                    ast_result,
                    bridge_dir,
                    mod_gen_strategy,
                };
                mod_gen.gen_files()?;
            }
            BridgeGen::JavaGen(ref crate_name, ast_result, bridge_dir, ref namespace) => {
                let mod_gen_strategy = JavaGenStrategyImp {
                    namespace: namespace.to_owned(),
                };
                let mod_gen = BridgeModGen {
                    crate_name: crate_name.to_owned(),
                    ast_result,
                    bridge_dir,
                    mod_gen_strategy,
                };
                mod_gen.gen_files()?;
            }
        };

        Ok(())
    }
}

///
/// Different strategy on generating a bridge mod.
///
pub(crate) trait ModGenStrategy {
    fn mod_name(&self, mod_name: &str) -> String;
    fn sdk_file_gen(&self, mod_names: &[String]) -> Result<TokenStream>;
    fn common_file_gen(&self) -> Result<TokenStream>;
    fn bridge_file_gen(
        &self,
        traits: &[TraitDesc],
        structs: &[StructDesc],
        imps: &[ImpDesc],
    ) -> Result<TokenStream>;
}

///
/// The executor for generating a bridge mod
///
pub(crate) struct BridgeModGen<'a, T: ModGenStrategy> {
    pub ast_result: &'a AstResult,
    pub bridge_dir: &'a PathBuf,
    pub mod_gen_strategy: T,
    pub crate_name: String,
}

impl<'a, T: ModGenStrategy> BridgeModGen<'a, T> {
    ///
    /// generate the bridge files
    ///
    pub(crate) fn gen_files(&self) -> Result<()> {
        let empty_vec = vec![];

        let traits = &self.ast_result.traits;
        let structs = &self.ast_result.structs;
        let imps = &self.ast_result.imps;

        let mut bridges: Vec<String> = vec![];
        for (mod_name, trait_vec) in traits {
            let struct_vec = structs.get(mod_name).unwrap_or(&empty_vec);

            // generate bridge files.
            let out_mod_name = self.mod_gen_strategy.mod_name(mod_name);
            let out_file_name = format!("{}.rs", &out_mod_name);

            let tokens = self
                .mod_gen_strategy
                .bridge_file_gen(trait_vec, struct_vec, imps)?;

            let out_file_path = self.bridge_dir.join(out_file_name);
            let mut f = File::create(&out_file_path)?;
            f.write_all(&tokens.to_string().into_bytes())?;

            bridges.push(out_mod_name)
        }

        // generate sdk.rs
        let tokens = self.mod_gen_strategy.sdk_file_gen(&bridges)?;
        let out_file_path = self.bridge_dir.join("sdk.rs");
        let mut f = File::create(&out_file_path)?;
        f.write_all(&tokens.to_string().into_bytes())?;
        bridges.push("sdk".to_owned());

        // generate common.rs
        let tokens = self.mod_gen_strategy.common_file_gen()?;
        let file_path = self.bridge_dir.join("common.rs");
        let mut file = File::create(&file_path)?;
        file.write_all(&tokens.to_string().into_bytes())?;

        // generate bridge/mod.rs
        self.gen_bridge_mod_code(self.bridge_dir, &bridges)?;

        // generate _gen/mod.rs
        self.gen_mode_code(self.bridge_dir)?;

        Ok(())
    }

    ///
    /// generate mod.rs in [c/java]/bridge dir.
    ///
    fn gen_bridge_mod_code(&self, out_dir: &Path, bridges: &[String]) -> Result<()> {
        let bridge_ident = bridges
            .iter()
            .map(|bridge| ident!(bridge))
            .collect::<Vec<Ident>>();

        let bridge_mod_tokens = quote! {
            # (pub mod #bridge_ident;)*
            pub mod common;
        };

        let bridget_mod_file_path = out_dir.join("mod.rs");
        let mut bridge_mod_file = File::create(&bridget_mod_file_path).unwrap();
        bridge_mod_file
            .write_all(&bridge_mod_tokens.to_string().into_bytes())
            .unwrap();

        Ok(())
    }

    ///
    /// generate the mode.rs in src/[c/java] directory.
    ///
    fn gen_mode_code(&self, out_dir: &Path) -> Result<()> {
        let gen_mod_tokens = quote! {
            pub mod bridge;
        };

        let gen_mod_path = out_dir.parent().unwrap().join("mod.rs");
        let mut gen_mod_file = File::create(&gen_mod_path).unwrap();
        gen_mod_file
            .write_all(&gen_mod_tokens.to_string().into_bytes())
            .unwrap();
        Ok(())
    }
}
