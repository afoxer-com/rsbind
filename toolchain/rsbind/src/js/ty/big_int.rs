use crate::ast::types::AstType;
use crate::base::lang::{Convertible, Direction};
use proc_macro2::{Ident, TokenStream};
use rstgen::{js, JavaScript, Tokens};

pub(crate) struct BigInt {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<JavaScript<'static>> for BigInt {
    fn native_to_transferable(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, JavaScript<'static>> {
        toks_f!("{}", origin)
    }

    fn transferable_to_native(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, JavaScript<'static>> {
        toks_f!("{}", origin)
    }

    fn rust_to_transferable(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        let (napi_create_fn, type_ident) = match self.ty.clone() {
            AstType::Long(ref base) => (ident!("napi_create_int64"), ident!("i64")),
            _ => panic!("Wrong type in big int."),
        };

        quote! {{
            let mut j_result = ptr::null_mut();
            let ok = unsafe {napi_sys::#napi_create_fn(env, #origin as #type_ident, &mut j_result)};
            if ok == 0 {
                j_result
            } else {
                panic!("Can't get big int. code = {}", ok);
            }
        }}
    }

    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        let (napi_get_fn, default_value, ty_ident) = match self.ty.clone() {
            AstType::Long(ref base) => (
                ident!("napi_get_value_int64"),
                proc_macro2::Literal::i64_suffixed(0),
                ident!(base),
            ),
            _ => {
                panic!("Wrong type in big int")
            }
        };
        quote! {{
            let mut result = #default_value;
            let ok = unsafe {napi_sys::#napi_get_fn(env, #origin, &mut result) };
            if ok == 0 {
                result as #ty_ident
            } else {
                panic!("Can't get big int. code = {}", ok);
            }
        }}
    }

    fn native_type(&self) -> JavaScript<'static> {
        js::local("bigint")
    }

    fn native_transferable_type(&self, direction: Direction) -> JavaScript<'static> {
        js::local("bigint")
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
