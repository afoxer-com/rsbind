use proc_macro2::TokenStream;
use rstgen::{java, Java, Tokens};

use crate::base::lang::{Convertible, Direction};

pub(crate) struct Void {}

impl<'a> Convertible<Java<'a>> for Void {
    fn native_to_transferable(
        &self,
        _origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        Tokens::new()
    }

    fn transferable_to_native(
        &self,
        _origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        Tokens::new()
    }

    fn rust_to_transferable(&self, _origin: TokenStream, _direction: Direction) -> TokenStream {
        quote! {}
    }

    fn transferable_to_rust(&self, _origin: TokenStream, _direction: Direction) -> TokenStream {
        quote! {}
    }

    fn native_type(&self) -> Java<'a> {
        java::VOID
    }

    fn quote_common_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_artifact(&self) -> Tokens<'static, Java<'a>> {
        Tokens::new()
    }
}
