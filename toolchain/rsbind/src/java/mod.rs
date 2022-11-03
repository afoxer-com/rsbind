use crate::base::bridge::{BaseBridgeGen, FilesGenerator};
use crate::base::lang::{LangGen, LangImp, ModContext};
use crate::errors::*;
use crate::java::artifact::JavaCodeGen;
use crate::AstResult;
use bridge::JavaImp;
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

pub(crate) struct JavaExtra {
    pub(crate) namespace: String,
}

impl LangGen for JavaGen {
    fn gen_bridge(&self, path: &Path) -> Result<()> {
        BaseBridgeGen {
            lang_name: "java".to_string(),
            ast: &self.ast,
            bridge_dir: path,
            crate_name: self.namespace.clone(),
            lang_imp: Box::new(JavaImp {}),
            extra: JavaExtra {
                namespace: self.namespace.clone(),
            },
            generator: FilesGenerator::default(),
        }
        .gen()
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
