use crate::ast::contract::desc::TraitDesc;
use crate::base::lang::LangGen;
use crate::errors::*;
use crate::swift::artifact::SwiftCodeGen;

use crate::AstResult;
use bridge::BridgeCodeGen;

use std::path::Path;

mod artifact;
mod bridge;
mod callback;
mod converter;
mod internal;
mod manager;
mod mapping;
mod protocol;
mod struct_;
mod ty;
mod types;
mod wrapper;

pub(crate) struct SwiftGen {
    pub(crate) crate_name: String,
    pub(crate) ast: AstResult,
}

impl LangGen for SwiftGen {
    fn gen_bridge(&self, path: &Path) -> Result<()> {
        BridgeCodeGen {
            crate_name: self.crate_name.clone(),
            ast: &self.ast,
            bridge_dir: path,
        }
        .gen_files()
    }

    fn gen_native(&self, path: &Path) -> Result<()> {
        SwiftCodeGen {
            swift_gen_dir: &path.to_path_buf(),
            ast: &self.ast,
        }
        .gen_files()
    }
}
