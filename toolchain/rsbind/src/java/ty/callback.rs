use proc_macro2::TokenStream;
use rstgen::{java, Java, Tokens};

use crate::ast::types::AstType;
use crate::base::{Convertible, Direction};
use crate::ident;

pub(crate) struct Callback {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<Java<'a>> for Callback {
    fn native_to_transferable(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        if let AstType::Callback(base) = self.ty.clone() {
            return toks_f!("Internal{}.pushGlobalCallback({})", base, origin);
        }

        toks!("")
    }

    fn transferable_to_native(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        if let AstType::Callback(base) = self.ty.clone() {
            return toks_f!("new Internal{}.J2R{}Wrapper({})", base, base, origin);
        }
        toks!("")
    }

    fn rust_to_transferable(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
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

    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        match self.ty.clone() {
            AstType::Callback(ref base) => {
                let _index_to_callback_fn = ident!(&format!("index_to_callback_{}", base));
                let value_get = match direction {
                    Direction::Down => {
                        quote! {}
                    }
                    Direction::Up => {
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

    fn native_type(&self) -> Java<'a> {
        match self.ty.clone() {
            AstType::Callback(ref origin) => java::local(origin.to_string()),
            _ => java::local(""),
        }
    }

    fn quote_common_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_artifact(&self) -> Tokens<'static, Java<'a>> {
        Tokens::new()
    }
}
