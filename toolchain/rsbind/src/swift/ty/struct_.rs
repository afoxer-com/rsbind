use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::{swift, Tokens};

use crate::ast::types::AstType;
use crate::base::lang::{Convertible, Direction};

pub(crate) struct Struct {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<Swift<'a>> for Struct {
    fn native_to_transferable(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        toks_f!("{}.intoProxy()", origin)
    }

    fn transferable_to_native(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        if let AstType::Struct(ref base) = self.ty.clone() {
            return toks_f!("{}(proxy: {})", base, origin);
        }
        toks!("")
    }

    fn rust_to_transferable(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        quote! {
            #origin.into()
        }
    }

    fn transferable_to_rust(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        quote! {
            #origin.into()
        }
    }

    fn native_type(&self) -> Swift<'a> {
        match self.ty.clone() {
            AstType::Struct(origin) => swift::local(origin.clone()),
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
