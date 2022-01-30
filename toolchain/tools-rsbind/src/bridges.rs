//!
//! module for generate C bridge files.
//! A C bridge file is exposed to callers of a foreign languages.
//!
use android::bridge as android_bridge;
use ast::contract::desc::*;
use ast::imp::desc::*;
use ast::AstResult;
use bridge::gen;
use errors::*;
use ios::bridge as ios_bridge;
use std::path::PathBuf;

pub(crate) enum BridgeGen<'a> {
    // create_name, ast, bridge_dir
    CGen(String, &'a AstResult, &'a PathBuf),
    // create_name, ast, bridge_dir, namespace
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
                let mod_gen = gen::BridgeModGen {
                    crate_name: crate_name.to_owned(),
                    ast_result,
                    bridge_dir,
                    mod_gen_strategy,
                };
                mod_gen.gen_bridges()?;
            }
            BridgeGen::JavaGen(ref crate_name, ast_result, bridge_dir, ref namespace) => {
                let mod_gen_strategy = JavaGenStrategyImp {
                    namespace: namespace.to_owned(),
                };
                let mod_gen = gen::BridgeModGen {
                    crate_name: crate_name.to_owned(),
                    ast_result,
                    bridge_dir,
                    mod_gen_strategy,
                };
                mod_gen.gen_bridges()?;
            }
        };

        Ok(())
    }
}

struct CGenStrategyImp {}

impl gen::ModGenStrategy for CGenStrategyImp {
    fn mod_name(&self, mod_name: &str) -> String {
        format!("c_{}", mod_name)
    }

    fn sdk_gen(&self, out_dir: &PathBuf, file_name: &str, mod_names: &Vec<String>) -> Result<()> {
        ios_bridge::new_gen(out_dir, &vec![], &vec![], &vec![]).gen_sdk_file(file_name, mod_names)
    }

    fn file_gen(
        &self,
        out_dir: &PathBuf,
        file_name: &str,
        trait_descs: &Vec<TraitDesc>,
        struct_descs: &Vec<StructDesc>,
        imp_desc: &Vec<ImpDesc>,
    ) -> Result<()> {
        ios_bridge::new_gen(out_dir, trait_descs, struct_descs, imp_desc)
            .gen_one_bridge_file(file_name)
    }
}

struct JavaGenStrategyImp {
    namespace: String,
}

impl gen::ModGenStrategy for JavaGenStrategyImp {
    fn mod_name(&self, mod_name: &str) -> String {
        format!("java_{}", mod_name)
    }

    fn sdk_gen(&self, out_dir: &PathBuf, file_name: &str, mod_names: &Vec<String>) -> Result<()> {
        android_bridge::new_gen(out_dir, &vec![], &vec![], &vec![], &self.namespace)
            .gen_sdk_file(file_name, mod_names)
    }

    fn file_gen(
        &self,
        out_dir: &PathBuf,
        file_name: &str,
        trait_descs: &Vec<TraitDesc>,
        struct_descs: &Vec<StructDesc>,
        imp_desc: &Vec<ImpDesc>,
    ) -> Result<()> {
        android_bridge::new_gen(
            out_dir,
            trait_descs,
            struct_descs,
            imp_desc,
            &self.namespace,
        )
        .gen_one_bridge_file(file_name)
    }
}
