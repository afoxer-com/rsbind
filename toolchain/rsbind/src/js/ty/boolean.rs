use crate::ast::types::AstType;
use crate::base::lang::{Convertible, Direction};
use proc_macro2::{Ident, TokenStream};
use rstgen::{js, JavaScript, Tokens};

pub(crate) struct Boolean {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<JavaScript<'static>> for Boolean {
    fn native_to_transferable(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, JavaScript<'static>> {
        toks_f!("{} ? 1 : 0", origin)
    }

    fn transferable_to_native(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, JavaScript<'static>> {
        toks_f!("{} > 0 ? true : false", origin)
    }

    fn rust_to_transferable(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        quote! {unsafe {
            let mut j_result = ptr::null_mut();
            let arg = if #origin {1} else {0};
            let ok = napi_sys::napi_create_int32(env, arg, &mut j_result);
            if ok == 0 {
                j_result
            } else {
                panic!("Can't create int32 for boolean.");
            }
        }}
    }

    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        quote! {unsafe {
            let mut result = 0;
            let ok = napi_sys::napi_get_value_int32(env, #origin, &mut result);
            if ok == 0 {
                if result > 0 {true} else {false}
            } else {
                panic!("Can't get int32 for boolean.");
            }
        }}
    }

    fn native_type(&self) -> JavaScript<'static> {
        js::local("boolean")
    }

    fn native_transferable_type(&self, direction: Direction) -> JavaScript<'static> {
        js::local("Number")
    }

    fn rust_transferable_type(&self, direction: Direction) -> TokenStream {
        quote! {napi_sys::napi_value}
    }

    fn quote_common_in_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_in_native(&self) -> Tokens<'static, JavaScript<'static>> {
        toks!()
    }

    fn quote_in_common_rs(&self) -> TokenStream {
        quote! {}
    }
}
