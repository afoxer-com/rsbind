use proc_macro2::TokenStream;
use rstgen::{java, Java, Tokens};

use crate::ast::types::AstType;
use crate::base::lang::{Convertible, Direction};
use crate::ident;

pub(crate) struct Struct {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<Java<'a>> for Struct {
    fn native_to_transferable(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        let json_cls = java::imported("com.google.gson", "Gson");
        toks!("new ", json_cls, "().toJson(", origin, ")")
    }

    fn transferable_to_native(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        let json_cls = java::imported("com.google.gson", "Gson");
        toks!(
            "new ",
            json_cls,
            "().fromJson(",
            origin,
            ",",
            self.ty.origin(),
            ".class)"
        )
    }

    fn rust_to_transferable(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        match self.ty.clone() {
            AstType::Struct(ref base) => {
                let proxy_struct = ident!(&format!("Proxy{}", &base.origin));
                match direction {
                    Direction::Down => {
                        quote! {{
                            let json = serde_json::to_string(&#proxy_struct::from(#origin));
                            env.new_string(json.unwrap()).expect("Couldn't create java string").into_inner()
                        }}
                    }
                    Direction::Up => {
                        quote! {{
                            let json = serde_json::to_string(&#proxy_struct::from(#origin));
                            env.new_string(json.unwrap()).expect("Couldn't create java string").into()
                        }}
                    }
                }
            }
            _ => {
                quote! {}
            }
        }
    }

    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        let value_get = match direction {
            Direction::Down => {
                quote! {}
            }
            Direction::Up => {
                quote! {
                    let #origin = match #origin {
                        Ok(JValue::Object(value)) => JString::from(value),
                        _ => panic!("Wrong struct type.")
                    };
                }
            }
        };

        match self.ty.clone() {
            AstType::Struct(ref base) => {
                let proxy_struct = ident!(&format!("Proxy{}", &base.origin));
                quote! {{
                    #value_get
                    let json: String = env.get_string(#origin).expect("Couldn't get java string!").into();
                    let proxy: #proxy_struct = serde_json::from_str(&json).unwrap();
                    proxy.into()
                }}
            }
            _ => {
                quote! {}
            }
        }
    }

    fn native_type(&self) -> Java<'a> {
        match self.ty.clone() {
            AstType::Struct(ref origin) => java::local(origin.origin.clone()),
            _ => java::local(""),
        }
    }

    fn native_transferable_type(&self, direction: Direction) -> Java<'a> {
        java::imported("java.lang", "String")
    }

    fn rust_transferable_type(&self, direction: Direction) -> TokenStream {
        match direction.clone() {
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
