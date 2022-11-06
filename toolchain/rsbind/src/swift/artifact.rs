use crate::ast::AstResult;
use crate::base::artifact::{FileNameStyle, NativeCodeGen, NativeGenStrategy};
use crate::errors::*;
use crate::swift::callback::{CallbackGen, InternalCallbackGen};
use crate::swift::internal::TraitGen;
use crate::swift::manager::ManagerGen;
use crate::swift::protocol::ProtocolGen;
use crate::swift::struct_::StructGen;
use crate::swift::wrapper::WrapperGen;
use std::path::PathBuf;

pub(crate) struct SwiftCodeGen<'a> {
    pub swift_gen_dir: &'a PathBuf,
    pub ast: &'a AstResult,
}

impl<'a> SwiftCodeGen<'a> {
    pub fn gen_files(&self) -> Result<()> {
        let strategy = NativeGenStrategy::<()> {
            gen_bridge_callback: Box::new(|ctx, desc| {
                InternalCallbackGen {
                    desc,
                    callbacks: ctx.callbacks.as_slice(),
                }
                .gen()
            }),
            gen_callback: Box::new(|_ctx, desc| CallbackGen { desc }.gen()),
            gen_bridge_trait: Box::new(|ctx, desc| {
                TraitGen {
                    desc,
                    callbacks: &ctx.callbacks,
                }
                .gen()
            }),
            gen_wrapper_trait: Box::new(|_ctx, desc| WrapperGen { desc }.gen()),
            gen_trait: Box::new(|_ctx, desc| ProtocolGen { desc }.gen()),
            gen_struct: Box::new(|_ctx, desc| StructGen { desc }.gen()),
            gen_manager: Box::new(|ctx| ManagerGen { ast: ctx.ast }.gen()),
        };

        let gen = NativeCodeGen {
            gen_dir: self.swift_gen_dir,
            file_ext: "swift".to_string(),
            file_name_style: FileNameStyle::CamelCase,
            ast: self.ast,
            extra: (),
            strategy,
        };

        gen.gen_files()
    }
}
