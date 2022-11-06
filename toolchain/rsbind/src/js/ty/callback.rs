use crate::ast::types::AstType;
use crate::base::lang::{Convertible, Direction};
use crate::AstResult;
use proc_macro2::TokenStream;
use rstgen::{js, JavaScript, Tokens};

pub(crate) struct Callback {
    pub(crate) ty: AstType,
    pub(crate) ast: AstResult,
}

impl<'a> Convertible<JavaScript<'static>> for Callback {
    fn native_to_transferable(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, JavaScript<'static>> {
        Tokens::new()
    }

    fn transferable_to_native(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, JavaScript<'static>> {
        Tokens::new()
    }

    fn rust_to_transferable(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        quote! {()}
    }

    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        quote! {()}
    }

    fn native_type(&self) -> JavaScript<'static> {
        match self.ty.clone() {
            AstType::Callback(ref base) => js::local(base.origin.clone()),
            _ => {
                panic!("Wrong type in callback.rs")
            }
        }
    }

    fn native_transferable_type(&self, direction: Direction) -> JavaScript<'static> {
        js::local("")
    }

    fn rust_transferable_type(&self, direction: Direction) -> TokenStream {
        quote! {}
    }

    fn quote_common_in_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_in_native(&self) -> Tokens<'static, JavaScript<'static>> {
        let mut tokens = Tokens::new();
        tokens
    }

    fn quote_in_common_rs(&self) -> TokenStream {
        quote! {}
    }
}
