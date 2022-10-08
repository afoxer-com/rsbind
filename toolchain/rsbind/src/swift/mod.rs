use crate::ast::contract::desc::TraitDesc;
use crate::base::lang::{BridgeContext, Convertible, Direction, LangGen, LangImp, ModContext};
use crate::errors::*;
use crate::swift::artifact::SwiftCodeGen;

use crate::{ident, AstResult};

use crate::ast::types::AstType;
use crate::base::bridge::{BaseBridgeGen, FilesGenerator};
use crate::swift::converter::SwiftConvert;
use bridge::SwiftImp;
use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use std::path::Path;

mod artifact;
mod bridge;
mod callback;
mod converter;
mod internal;
mod manager;
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
        let mut generator = FilesGenerator::<Swift<'static>, ()>::default();
        let old_arg_convert = generator
            .bridge_file_generator
            .bridge_code_generator
            .trait_generator
            .trait_method_generator
            .quote_method_return_convert;

        generator
            .bridge_file_generator
            .bridge_code_generator
            .trait_generator
            .trait_method_generator
            .quote_method_return_convert = Box::new(move |ctx| {
            let obtain_index = if let AstType::Callback(_) = ctx.method.return_type.clone() {
                quote! {
                    let callback_index = {
                        let mut global_index = CALLBACK_INDEX.write().unwrap();
                        let mut index = *global_index;
                        if index == i64::MAX {
                            *global_index = 0;
                            index = 0;
                        } else {
                            *global_index = index + 1;
                            index = index + 1;
                        }
                        index
                    };
                }
            } else {
                quote! {}
            };

            let convert = (*old_arg_convert)(ctx)?;
            let return_convert = quote! {
                let r_result = #convert;
            };

            let insert_callback = if let AstType::Callback(ref origin) =
                ctx.method.return_type.clone()
            {
                let callback_ident = ident!(&origin.origin);
                quote! {
                    (*CALLBACK_HASHMAP.write().unwrap()).insert(callback_index, CallbackEnum::#callback_ident(result));
                }
            } else {
                quote! {}
            };

            Ok(quote! {
                #obtain_index
                #return_convert
                #insert_callback
                r_result
            })
        });

        BaseBridgeGen {
            lang_name: "swift".to_string(),
            ast: &self.ast,
            bridge_dir: path,
            crate_name: self.crate_name.clone(),
            lang_imp: Box::new(SwiftImp {}),
            extra: (),
            generator,
        }
        .gen()
    }

    fn gen_native(&self, path: &Path) -> Result<()> {
        SwiftCodeGen {
            swift_gen_dir: &path.to_path_buf(),
            ast: &self.ast,
        }
        .gen_files()
    }
}
