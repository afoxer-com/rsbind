use crate::ast::contract::desc::TraitDesc;
use crate::base::lang::LangGen;
use crate::errors::*;
use crate::java::artifact::JavaCodeGen;
use crate::java::bridge::BridgeCodeGen;
use crate::AstResult;
use std::path::Path;

mod artifact;
mod bridge;
mod callback;
mod converter;
mod interface;
mod internal;
mod manager;
mod struct_;
mod ty;
mod types;
mod wrapper;

pub(crate) struct JavaGen {
    pub(crate) crate_name: String,
    pub(crate) ast: AstResult,
    pub(crate) namespace: String,
    pub(crate) so_name: String,
    pub(crate) ext_libs: String,
}

impl LangGen for JavaGen {
    fn gen_bridge(&self, path: &Path) -> Result<()> {
        BridgeCodeGen {
            crate_name: self.crate_name.clone(),
            ast: &self.ast,
            bridge_dir: path,
            namespace: self.namespace.to_string(),
        }
        .gen_files()
    }

    fn gen_native(&self, path: &Path) -> Result<()> {
        JavaCodeGen {
            java_gen_dir: &path.to_path_buf(),
            ast: &self.ast,
            namespace: self.namespace.clone(),
            so_name: self.so_name.clone(),
            ext_libs: self.ext_libs.clone(),
        }
        .gen_files()
    }
}
