use proc_macro2::TokenStream;
use rstgen::{java, Java, Tokens};

use crate::base::lang::{Convertible, Direction};

pub(crate) struct Str {}

impl<'a> Convertible<Java<'a>> for Str {
    fn native_to_transferable(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        toks_f!("{}", origin)
    }

    fn transferable_to_native(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        toks_f!("{}", origin)
    }

    fn rust_to_transferable(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        match direction {
            Direction::Down => {
                quote! {
                    env.new_string(#origin).expect("Couldn't new java string!").into_inner()
                }
            }
            Direction::Up => {
                quote! {
                    env.new_string(#origin).expect("Couldn't new java string!").into()
                }
            }
        }
    }

    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        match direction {
            Direction::Down => {
                quote! {
                    env.get_string(#origin).expect("Couldn't get java string!").into()
                }
            }
            Direction::Up => {
                quote! {
                    match #origin {
                        Ok(JValue::Object(value)) => {
                            let jstr = JString::from(value);
                            env.get_string(jstr).unwrap().to_string_lossy().to_string()
                        },
                        _ => panic!("Wrong string type.")
                    }
                }
            }
        }
    }

    fn native_type(&self) -> Java<'a> {
        java::imported("java.lang", "String")
    }

    fn native_transferable_type(&self, _direction: Direction) -> Java<'a> {
        java::imported("java.lang", "String")
    }

    fn rust_transferable_type(&self, direction: Direction) -> TokenStream {
        match direction {
            Direction::Down => {
                quote! {JString}
            }
            Direction::Up => {
                quote! {jstring}
            }
        }
    }

    fn quote_common_in_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_in_native(&self) -> Tokens<'static, Java<'a>> {
        Tokens::new()
    }

    fn quote_in_common_rs(&self) -> TokenStream {
        quote! {}
    }
}
