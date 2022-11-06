use crate::ast::types::{AstBaseType, AstType};
use crate::ast::AstResult;
use crate::base::lang::{Convertible, Direction};
use crate::js::ty::boolean::Boolean;
use crate::js::ty::converter::JsConverter;
use crate::js::ty::number::Number;
use crate::js::ty::string::Str;
use proc_macro2::{Ident, TokenStream};
use rstgen::{js, JavaScript, Tokens};

pub(crate) struct Array {
    pub(crate) ty: AstType,
    pub(crate) ast: AstResult,
}

impl<'a> Convertible<JavaScript<'static>> for Array {
    fn native_to_transferable(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, JavaScript<'static>> {
        match self.ty.clone() {
            AstType::Vec(AstBaseType::Boolean) => {
                let mut tokens = Tokens::new();
                push!(tokens, "function() {");
                nested!(tokens, |ft| {
                    nested!(ft, "let converted: Number[] = [];");
                    nested_f!(ft, "for (const v of {}) {{", origin);
                    nested!(ft, |t| { nested!(t, "converted.push(v ? 1 : 0);") });
                    nested!(ft, "}");
                    nested!(ft, "return converted;");
                });

                nested!(tokens, "}()");
                tokens
            }
            _ => toks_f!("{}", origin),
        }
    }

    fn transferable_to_native(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, JavaScript<'static>> {
        match self.ty.clone() {
            AstType::Vec(AstBaseType::Boolean) => {
                let mut tokens = Tokens::new();
                push!(tokens, "function() {");
                nested!(tokens, |ft| {
                    nested!(ft, "let converted: boolean[] = [];");
                    nested_f!(ft, "for (const v of {}) {{", origin);
                    nested!(ft, |t| {
                        nested!(t, "converted.push(v > 0 ? true : false);")
                    });
                    nested!(ft, "}");
                    nested!(ft, "return converted;");
                });

                nested!(tokens, "}()");
                tokens
            }
            _ => toks_f!("{}", origin),
        }
    }

    fn rust_to_transferable(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        let element_convert = match self.ty.clone() {
            AstType::Vec(ref base) => match AstType::from(base.clone()) {
                AstType::Byte(_)
                | AstType::Int(_)
                | AstType::Short(_)
                | AstType::Long(_)
                | AstType::Float(_)
                | AstType::Double(_) => Number {
                    ty: AstType::from(base.clone()),
                }
                .rust_to_transferable(quote! {value}, direction),
                AstType::Boolean => Boolean {
                    ty: AstType::from(base.clone()),
                }
                .rust_to_transferable(quote! {value}, direction),
                AstType::String => Str {
                    ty: AstType::from(base.clone()),
                }
                .rust_to_transferable(quote! {value}, direction),
                _ => {
                    panic!("unsupported type in Vec: {:?}", base.clone())
                }
            },
            _ => {
                panic!("unsupported types in array.")
            }
        };
        quote! {{
            let mut j_result = ptr::null_mut();
            let len = #origin.len();

            let ok = unsafe {napi_sys::napi_create_array_with_length(env, len, &mut j_result)};
            if ok == 0 {
                for (index, value) in #origin.into_iter().enumerate() {
                    let converted = #element_convert;
                    let ok = unsafe {sys::napi_set_element(env, j_result, index as u32, converted)};
                    if ok != 0 {
                        panic!("Can't set element for array.");
                    }
                }
                j_result
            } else {
                panic!("Can't create string for string.js.");
            }
        }}
    }

    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        let element_convert = match self.ty.clone() {
            AstType::Vec(ref base) => match AstType::from(base.clone()) {
                AstType::Byte(_)
                | AstType::Int(_)
                | AstType::Short(_)
                | AstType::Long(_)
                | AstType::Float(_)
                | AstType::Double(_) => Number {
                    ty: AstType::from(base.clone()),
                }
                .transferable_to_rust(quote! {value}, direction),
                AstType::Boolean => Boolean {
                    ty: AstType::from(base.clone()),
                }
                .transferable_to_rust(quote! {value}, direction),
                AstType::String => Str {
                    ty: AstType::from(base.clone()),
                }
                .transferable_to_rust(quote! {value}, direction),
                _ => {
                    panic!("unsupported type in Vec: {:?}", base.clone())
                }
            },
            _ => {
                panic!("unsupported types in array.")
            }
        };

        quote! {{
            // first, get the array length.
            let mut len = 0;
            let ok = unsafe {sys::napi_get_array_length(env, #origin, &mut len)};
            if ok == 0 {
                let mut result = Vec::with_capacity(len as usize);

                for index in 0..len {
                    let mut value = ptr::null_mut();
                    let ok = unsafe {sys::napi_get_element(env, #origin, index as u32, &mut value)};
                    if ok == 0 {
                        let converted = #element_convert;
                        result.push(converted);
                    } else {
                        panic!("get element error in array.")
                    }
                }

                result
            } else {
                panic!("Can't get utf8 string for string.");
            }
        }}
    }

    fn native_type(&self) -> JavaScript<'static> {
        let base = match self.ty.clone() {
            AstType::Vec(ref base) => {
                let base_type = AstType::from(base.clone());
                JsConverter {
                    ty: base_type,
                    ast: self.ast.clone(),
                }
                .native_type()
            }
            _ => panic!("Wrong types in array."),
        };
        js::local(format!("Array<{}>", base))
    }

    fn native_transferable_type(&self, direction: Direction) -> JavaScript<'static> {
        let base = match self.ty.clone() {
            AstType::Vec(ref base) => {
                let base_type = AstType::from(base.clone());
                JsConverter {
                    ty: base_type,
                    ast: self.ast.clone(),
                }
                .native_transferable_type(direction)
            }
            _ => panic!("Wrong types in array."),
        };
        js::local(format!("Array<{}>", base))
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
