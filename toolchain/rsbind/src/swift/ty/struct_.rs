use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::Tokens;

use crate::ast::types::AstType;
use crate::base::{Convertible, Direction};

pub(crate) struct Struct {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<Swift<'a>> for Struct {
    fn artifact_to_transfer(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        push_f!(body, "{}.intoProxy()", origin);
        body
    }

    fn transfer_to_artifact(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        match self.ty.clone() {
            AstType::Struct(ref base) => {
                nested_f!(body, "{}(proxy: {})", base, origin);
            }
            _ => {}
        }
        body
    }

    fn rust_to_transfer(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        quote! {
            #origin.into()
        }
    }

    fn transfer_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        quote! {
            #origin.into()
        }
    }

    fn quote_common_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_artifact(&self) -> Tokens<'static, Swift<'a>> {
        Tokens::new()
    }
}
