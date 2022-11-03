use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::{swift, Tokens};

use crate::base::lang::{Convertible, Direction};

pub(crate) struct Void {}

impl<'a> Convertible<Swift<'a>> for Void {
    fn native_to_transferable(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        toks!(origin)
    }

    fn transferable_to_native(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        toks!(origin)
    }

    fn rust_to_transferable(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        quote! {
            #origin
        }
    }

    fn transferable_to_rust(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        quote! {#origin}
    }

    fn native_type(&self) -> Swift<'a> {
        swift::local("()")
    }

    fn native_transferable_type(&self, _direction: Direction) -> Swift<'a> {
        swift::VOID
    }

    fn rust_transferable_type(&self, _direction: Direction) -> TokenStream {
        quote! {()}
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
