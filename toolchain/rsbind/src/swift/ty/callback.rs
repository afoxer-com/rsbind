use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::{swift, Tokens};

use crate::ast::types::AstType;
use crate::base::{Convertible, Direction};
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
        let mut body = Tokens::new();
        if let AstType::Callback(base) = self.ty.clone() {
            push_f!(
                body,
                "Internal{}.callbackToModel(callback: {})",
                base,
                origin
            );
        }
        body
    }

    fn transferable_to_native(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        if let AstType::Callback(base) = self.ty.clone() {
            nested_f!(body, "Internal{}.modelToCallback(model:{})", base, origin);
        }
        body
    }

    fn rust_to_transferable(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        let box_to_model_fn_name = ident!(&format!("box_to_model_{}", self.ty.origin()));
        quote! {{
            let #origin = callback_index;
            #box_to_model_fn_name(#origin)
        }}
    }

    fn transferable_to_rust(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        let model_to_box_fn = ident!(&format!("model_to_box_{}", self.ty.origin()));
        quote! {
             #model_to_box_fn(#origin)
        }
    }

    fn native_type(&self) -> Swift<'a> {
        match self.ty.clone() {
            AstType::Callback(origin) => swift::local(origin.clone()),
            _ => swift::local(""),
        }
    }

    fn quote_common_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_artifact(&self) -> Tokens<'static, Swift<'a>> {
        Tokens::new()
    }
}
