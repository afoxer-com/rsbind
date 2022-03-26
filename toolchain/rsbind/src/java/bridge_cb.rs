use heck::ToUpperCamelCase;
use proc_macro2::{Ident, Literal, Span, TokenStream};

use crate::ast::contract::desc::*;
use crate::ast::types::*;
use crate::bridge::file::*;
use crate::errors::*;

pub struct JavaCallbackStrategy {
    pub(crate) java_namespace: String,
}

impl CallbackGenStrategy for JavaCallbackStrategy {
    fn arg_convert(
        &self,
        arg: &ArgDesc,
        _trait_desc: &TraitDesc,
        callbacks: &[&TraitDesc],
    ) -> Result<TokenStream> {
        println!(
            "[bridge] 🔆  begin quote callback argument in method convert => {}.{}",
            &arg.name,
            &arg.ty.origin()
        );
        let rust_arg_name = Ident::new(
            &format!("{}_{}", TMP_ARG_PREFIX, &arg.name),
            Span::call_site(),
        );

        for callback in callbacks.iter() {
            if arg.ty.origin() == callback.name {
                let arg_name_ident = Ident::new(&arg.name, Span::call_site());
                let index_to_cb_fn_name = Ident::new(&format!("index_to_callback_{}", &callback.name), Span::call_site());
                return Ok(quote! {
                    let #rust_arg_name = #index_to_cb_fn_name(#arg_name_ident);
                });
            }
        }

        Ok(quote! {})
    }

    fn return_convert(
        &self,
        ret_ty: &AstType,
        _trait_desc: &TraitDesc,
        _callbacks: &[&TraitDesc],
    ) -> Result<TokenStream> {
        let cb_to_index_fn_name = Ident::new(
            &format!("callback_to_index_{}", &ret_ty.origin()),
            Span::call_site(),
        );
        Ok(quote! {
            #cb_to_index_fn_name(result)
        })
    }
}
