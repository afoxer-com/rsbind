use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::{swift, Tokens};

use crate::ast::types::AstType;
use crate::base::lang::{Convertible, Direction};
use crate::ident;

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
            return toks_f!("{}(proxy: {})", &base.origin, origin);
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
            AstType::Struct(origin) => swift::local(origin.origin.clone()),
            _ => swift::local(""),
        }
    }

    fn native_transferable_type(&self, direction: Direction) -> Swift<'a> {
        match self.ty.clone() {
            AstType::Struct(origin) => swift::local(format!("Proxy{}", &origin.origin)),
            _ => swift::local(""),
        }
    }

    fn rust_transferable_type(&self, direction: Direction) -> TokenStream {
        match self.ty.clone() {
            AstType::Struct(ref origin) => {
                let struct_ident = ident!(&format!("Proxy{}", &origin.origin));
                quote!(#struct_ident)
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
