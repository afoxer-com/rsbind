use proc_macro2::TokenStream;
use rstgen::{Java, Tokens};

use crate::ast::types::AstType;
use crate::base::{Convertible, Direction};
use crate::ident;

pub(crate) struct Callback {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<Java<'a>> for Callback {
    fn artifact_to_transfer(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        let mut body = Tokens::new();
        if let AstType::Callback(base) = self.ty.clone() {
            push!(body, "Internal", base, ".pushGlobalCallback(", origin, ")");
        }

        body
    }

    fn transfer_to_artifact(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        let mut body = Tokens::new();
        if let AstType::Callback(base) = self.ty.clone() {
            push!(
                body,
                "new Internal",
                base,
                ".J2R",
                base,
                "Wrapper(",
                origin,
                ")"
            );
        }
        body
    }

    fn rust_to_transfer(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        match self.ty.clone() {
            AstType::Callback(ref base) => {
                let cb_to_index_fn_name = ident!(&format!("callback_to_index_{}", base));
                quote! {
                    #cb_to_index_fn_name(#origin)
                }
            }
            _ => {
                quote! {}
            }
        }
    }

    fn transfer_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        match self.ty.clone() {
            AstType::Callback(ref base) => {
                let index_to_callback_fn = ident!(&format!("index_to_callback_{}", base));
                let value_get = match direction {
                    Direction::Invoke => {
                        quote! {}
                    }
                    Direction::Push => {
                        quote! {
                            let #origin = match #origin {
                                Ok(JValue::Long(value)) => value,
                                _ => panic!("Wrong callback type.")
                            };
                        }
                    }
                };
                let index_to_cb_fn_name = ident!(&format!("index_to_callback_{}", base));
                quote! {{
                    #value_get
                    #index_to_cb_fn_name(#origin)
                }}
            }
            _ => {
                quote! {}
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
