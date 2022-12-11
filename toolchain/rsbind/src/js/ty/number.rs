use crate::ast::types::AstType;
use crate::base::lang::{Convertible, Direction};
use proc_macro2::{Ident, TokenStream};
use rstgen::{js, JavaScript, Tokens};

pub(crate) struct Number {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<JavaScript<'static>> for Number {
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
        match self.ty.clone() {
            AstType::Byte(ref base)
            | AstType::Short(ref base)
            | AstType::Int(ref base)
            | AstType::Long(ref base)
            | AstType::Float(ref base)
            | AstType::Double(ref base) => {
                let (napi_create_fn, type_ident) = match self.ty.clone() {
                    AstType::Byte(_) | AstType::Int(_) | AstType::Short(_) => {
                        (ident!("napi_create_int32"), ident!("i32"))
                    }
                    AstType::Long(_) => (ident!("napi_create_int64"), ident!("i64")),
                    AstType::Float(_) | AstType::Double(_) => {
                        (ident!("napi_create_double"), ident!("f64"))
                    }
                    _ => panic!(),
                };

                quote! {{
                    let mut j_result = ptr::null_mut();
                    let ok = unsafe {napi_sys::#napi_create_fn(env, #origin as #type_ident, &mut j_result)};
                    if ok == 0 {
                        j_result
                    } else {
                        panic!("Can't create int32/int64/double.");
                    }
                }}
            }
            _ => {
                panic!("Wrong type in basic.")
            }
        }
    }

    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        match self.ty.clone() {
            AstType::Byte(ref base)
            | AstType::Short(ref base)
            | AstType::Int(ref base)
            | AstType::Long(ref base)
            | AstType::Float(ref base)
            | AstType::Double(ref base) => {
                let (napi_get_fn, default_value) = match self.ty.clone() {
                    AstType::Byte(_) | AstType::Int(_) | AstType::Short(_) => (
                        ident!("napi_get_value_int32"),
                        proc_macro2::Literal::i32_suffixed(0),
                    ),
                    AstType::Long(_) => (
                        ident!("napi_get_value_int64"),
                        proc_macro2::Literal::i64_suffixed(0),
                    ),
                    AstType::Float(_) | AstType::Double(_) => (
                        ident!("napi_get_value_double"),
                        proc_macro2::Literal::f64_suffixed(0f64),
                    ),
                    _ => panic!(),
                };

                let ty_ident = ident!(base);
                quote! {{
                    let mut result = #default_value;
                    let ok = unsafe {napi_sys::#napi_get_fn(env, #origin, &mut result) };
                    if ok == 0 {
                        result as #ty_ident
                    } else {
                        panic!("Can't get int32/int64/double.");
                    }
                }}
            }
            _ => {
                panic!("Wrong type in basic")
            }
        }
    }

    fn native_type(&self) -> JavaScript<'static> {
        match self.ty.clone() {
            AstType::Byte(ref base)
            | AstType::Short(ref base)
            | AstType::Int(ref base)
            | AstType::Long(ref base)
            | AstType::Float(ref base)
            | AstType::Double(ref base) => js::local("number"),
            _ => {
                panic!("Wrong type in basic")
            }
        }
    }

    fn native_transferable_type(&self, direction: Direction) -> JavaScript<'static> {
        match self.ty.clone() {
            AstType::Byte(ref base)
            | AstType::Short(ref base)
            | AstType::Int(ref base)
            | AstType::Long(ref base)
            | AstType::Float(ref base)
            | AstType::Double(ref base) => js::local("number"),
            _ => {
                panic!("Wrong type in basic")
            }
        }
    }

    fn rust_transferable_type(&self, direction: Direction) -> TokenStream {
        match self.ty.clone() {
            AstType::Byte(ref base)
            | AstType::Short(ref base)
            | AstType::Int(ref base)
            | AstType::Long(ref base)
            | AstType::Float(ref base)
            | AstType::Double(ref base) => {
                quote! {napi_sys::napi_value}
            }
            _ => {
                panic!("Wrong type in basic")
            }
        }
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
