use proc_macro2::{Ident, Span, TokenStream};

use crate::ast::contract::desc::*;
use crate::ast::types::*;
use crate::bridge::file::*;
use crate::errors::*;
use crate::swift::mapping::RustMapping;

pub struct CCallbackStrategy {}

impl CallbackGenStrategy for CCallbackStrategy {
    fn arg_convert(
        &self,
        arg: &ArgDesc,
        trait_desc: &TraitDesc,
        callbacks: &[&TraitDesc],
    ) -> Result<TokenStream> {
        Ok(quote!())
    }

    fn return_convert(
        &self,
        ret_ty: &AstType,
        trait_desc: &TraitDesc,
        callbacks: &[&TraitDesc],
    ) -> Result<TokenStream> {
        Ok(quote! {})
    }
}

impl CCallbackStrategy {
    pub(crate) fn quote_callback_struct(
        &self,
        callback: &TraitDesc,
        callbacks: &[&TraitDesc],
        name: &str,
    ) -> Result<TokenStream> {
        crate::swift::bridge_s2r::quote_callback_struct(callback, callbacks, name)
    }
}
