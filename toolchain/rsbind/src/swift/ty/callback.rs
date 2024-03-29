use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::{swift, Tokens};

use crate::ast::types::AstType;
use crate::base::lang::{Convertible, Direction};
use crate::ident;

pub(crate) struct Callback {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<Swift<'a>> for Callback {
    fn native_to_transferable(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        if let AstType::Callback(base) = self.ty.clone() {
            return toks_f!(
                "Internal{}.callbackToModel(callback: {})",
                &base.origin,
                origin
            );
        }
        toks!("")
    }

    fn transferable_to_native(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        if let AstType::Callback(base) = self.ty.clone() {
            return toks_f!("Internal{}.modelToCallback(model:{})", &base.origin, origin);
        }
        toks!("")
    }

    fn rust_to_transferable(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        let callback_to_c_pointers_fn_name =
            ident!(&format!("callback_to_c_pointers_{}", self.ty.origin()));
        quote! {{
            let #origin = callback_index;
            #callback_to_c_pointers_fn_name(#origin)
        }}
    }

    fn transferable_to_rust(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        let c_pointers_to_callback_fn =
            ident!(&format!("c_pointers_to_callback_{}", self.ty.origin()));
        quote! {
             #c_pointers_to_callback_fn(#origin)
        }
    }

    fn native_type(&self) -> Swift<'a> {
        match self.ty.clone() {
            AstType::Callback(origin) => swift::local(origin.origin),
            _ => swift::local(""),
        }
    }

    fn native_transferable_type(&self, _direction: Direction) -> Swift<'a> {
        match self.ty.clone() {
            AstType::Callback(origin) => {
                let callback_str = format!("{}_{}_Model", &origin.mod_name, &origin.origin);
                swift::local(callback_str)
            }
            _ => swift::local(""),
        }
    }

    fn rust_transferable_type(&self, _direction: Direction) -> TokenStream {
        match self.ty.clone() {
            AstType::Callback(origin) => {
                let ident = ident!(&format!("{}_{}_Model", &origin.mod_name, &origin.origin));
                quote! {#ident}
            }
            _ => quote! {},
        }
    }

    fn quote_common_in_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_in_native(&self) -> Tokens<'static, Swift<'a>> {
        Tokens::new()
    }

    fn quote_in_common_rs(&self) -> TokenStream {
        quote! {}
    }
}
