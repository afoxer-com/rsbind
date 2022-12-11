use crate::ast::AstResult;
use crate::base::artifact::{FileNameStyle, NativeCodeGen, NativeGenStrategy};
use crate::errors::*;
use crate::java::callback::{CallbackGen, InnerCallbackGen};
use crate::java::interface::InterfaceGen;
use crate::java::internal::InnerTraitGen;
use crate::java::manager::ManagerGen;
use crate::java::struct_::StructGen;
use crate::java::wrapper::WrapperGen;
use std::path::PathBuf;

pub(crate) struct JavaCodeGen<'a> {
    pub java_gen_dir: &'a PathBuf,
    pub ast: &'a AstResult,
    pub namespace: String,
    pub so_name: String,
    pub ext_libs: String,
}

#[derive(Clone)]
pub(crate) struct JavaExtra {
    pub namespace: String,
    pub so_name: String,
    pub ext_libs: String,
}

impl<'a> JavaCodeGen<'a> {
    pub(crate) fn gen_files(&self) -> Result<()> {
        let strategy = NativeGenStrategy::<JavaExtra> {
            gen_bridge_callback: Box::new(|ctx, desc| {
                InnerCallbackGen {
                    desc,
                    pkg: ctx.extra.namespace.clone(),
                }
                .gen()
            }),
            gen_callback: Box::new(|ctx, desc| {
                CallbackGen {
                    desc,
                    pkg: ctx.extra.namespace.clone(),
                }
                .gen()
            }),
            gen_bridge_trait: Box::new(|ctx, desc| {
                InnerTraitGen {
                    desc,
                    pkg: ctx.extra.namespace.clone(),
                    callbacks: ctx.callbacks.clone(),
                }
                .gen()
            }),
            gen_wrapper_trait: Box::new(|ctx, desc| {
                WrapperGen {
                    desc,
                    pkg: ctx.extra.namespace.clone(),
                }
                .gen()
            }),
            gen_trait: Box::new(|ctx, desc| {
                InterfaceGen {
                    desc,
                    pkg: ctx.extra.namespace.clone(),
                }
                .gen()
            }),
            gen_struct: Box::new(|ctx, desc| {
                StructGen {
                    desc,
                    pkg: ctx.extra.namespace.clone(),
                }
                .gen()
            }),
            gen_manager: Box::new(|ctx| {
                ManagerGen {
                    ast: ctx.ast,
                    pkg: ctx.extra.namespace.clone(),
                    so_name: ctx.extra.so_name.clone(),
                    ext_libs: ctx.extra.ext_libs.clone(),
                }
                .gen()
            }),
        };

        let gen = NativeCodeGen {
            gen_dir: self.java_gen_dir,
            file_ext: "java".to_string(),
            file_name_style: FileNameStyle::CamelCase,
            ast: self.ast,
            extra: JavaExtra {
                namespace: self.namespace.clone(),
                so_name: self.so_name.clone(),
                ext_libs: self.ext_libs.clone(),
            },
            strategy,
        };

        gen.gen_files()
    }
}
