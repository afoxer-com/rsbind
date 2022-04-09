use proc_macro2::TokenStream;
use rstgen::{Java, Tokens};

use crate::base::{Convertible, Direction};

pub(crate) struct Str {}

impl<'a> Convertible<Java<'a>> for Str {
    fn artifact_to_transfer(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        let mut body = Tokens::new();
        push_f!(body, "{}", origin);
        body
    }

    fn transfer_to_artifact(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        let mut body = Tokens::new();
        push_f!(body, "{}", origin);
        body
    }

    fn rust_to_transfer(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        match direction {
            Direction::Invoke => {
                quote! {
                    env.new_string(#origin).expect("Couldn't new java string!").into_inner()
                }
            }
            Direction::Push => {
                quote! {
                    env.new_string(#origin).expect("Couldn't new java string!").into()
                }
            }
        }
    }

    fn transfer_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        match direction {
            Direction::Invoke => {
                quote! {
                    env.get_string(#origin).expect("Couldn't get java string!").into()
                }
            }
            Direction::Push => {
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

    fn quote_common_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_artifact(&self) -> Tokens<'static, Java<'a>> {
        Tokens::new()
    }
}
