use crate::base::bridge::{BaseBridgeGen, FilesGenerator};
use crate::base::lang::LangGen;
use crate::js::artifact::JSCodeGen;
use crate::js::bridge::JsImp;
use crate::AstResult;
use proc_macro2::{Ident, Literal};
use rstgen::JavaScript;
use std::path::Path;

mod artifact;
mod bridge;
mod ty;

pub(crate) struct JsGen {
    pub(crate) crate_name: String,
    pub(crate) ast: AstResult,
}

impl LangGen for JsGen {
    fn gen_bridge(&self, path: &Path) -> crate::Result<()> {
        let mut generator = FilesGenerator::<JavaScript<'static>, ()>::default();
        let old_arg_convert = generator
            .bridge_file_generator
            .bridge_code_generator
            .trait_generator
            .trait_method_generator
            .quote_method_header;
        generator
            .bridge_file_generator
            .bridge_code_generator
            .trait_generator
            .trait_method_generator
            .quote_method_header = Box::new(move |ctx| {
            let arg_idents = ctx
                .method
                .args
                .iter()
                .map(|arg| ident!(&arg.name))
                .collect::<Vec<Ident>>();
            let arg_index = ctx
                .method
                .args
                .iter()
                .enumerate()
                .map(|(index, _)| proc_macro2::Literal::usize_unsuffixed(index))
                .collect::<Vec<Literal>>();
            let arg_count = ctx.method.args.len();
            let count = proc_macro2::Literal::usize_unsuffixed(arg_count);

            let old_header = (*old_arg_convert)(ctx)?;
            Ok(quote! {
                let mut argc = #count;
                #[cfg(all(target_os = "windows", target_arch = "x86"))]
                let mut raw_args = vec![ptr::null_mut(); #count];
                #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
                let mut raw_args = [ptr::null_mut(); #count];
                let mut raw_this = ptr::null_mut();

                unsafe {
                    let status = sys::napi_get_cb_info(
                        env,
                        cb_info,
                        &mut argc,
                        raw_args.as_mut_ptr(),
                        &mut raw_this,
                        ptr::null_mut(),
                    );
                }

                #(let #arg_idents = raw_args[#arg_index];)*
                #old_header
            })
        });

        BaseBridgeGen {
            lang_name: "js".to_string(),
            ast: &self.ast,
            bridge_dir: path,
            crate_name: self.crate_name.clone(),
            lang_imp: Box::new(JsImp {}),
            extra: (),
            generator,
        }
        .gen()
    }

    fn gen_native(&self, path: &Path) -> crate::Result<()> {
        JSCodeGen {
            js_gen_dir: &path.to_path_buf(),
            ast: &self.ast,
        }
        .gen_files()
    }
}
