use proc_macro2::TokenStream;
use rstgen::{java, Java, Tokens};

use crate::ast::types::{AstBaseType, AstType};
use crate::base::{Convertible, Direction};
use crate::ident;
use crate::java::types::JavaType;

pub(crate) struct VecStruct {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<Java<'a>> for VecStruct {
    fn artifact_to_transfer(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        let json_cls = java::imported("com.google.gson", "Gson");
        let mut body = Tokens::new();
        push!(body, "new ", json_cls, "().toJson(", origin, ")");
        body
    }

    fn transfer_to_artifact(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        let mut body = Tokens::new();
        let json = java::imported("com.google.gson", "Gson");
        match self.ty.clone() {
            AstType::Vec(AstBaseType::Struct(ref base)) => {
                push!(
                    body,
                    "new ",
                    json,
                    "().fromJson(",
                    origin,
                    ", ",
                    base.to_string(),
                    "[].class)"
                );
            }
            _ => {}
        }

        body
    }

    fn rust_to_transfer(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        match self.ty.clone() {
            AstType::Vec(AstBaseType::Struct(ref base)) => {
                let proxy_struct = ident!(&format!("Proxy{}", base));
                match direction {
                    Direction::Invoke => {
                        quote! {
                            {
                                let proxies = #origin.into_iter().map(|each| #proxy_struct::from(each)).collect::<Vec<#proxy_struct>>();
                                let json = serde_json::to_string(&proxies);
                                env.new_string(json.unwrap()).expect("Couldn't create java string").into_inner()
                            }
                        }
                    }
                    Direction::Push => {
                        quote! {
                            {
                                let proxies = #origin.into_iter().map(|each| #proxy_struct::from(each)).collect::<Vec<#proxy_struct>>();
                                let json = serde_json::to_string(&proxies);
                                env.new_string(json.unwrap()).expect("Couldn't create java string").into()
                            }
                        }
                    }
                }
            }
            _ => {
                quote! {}
            }
        }
    }

    fn transfer_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        let value_get = match direction {
            Direction::Invoke => {
                quote! {}
            }
            Direction::Push => {
                quote! {
                    let #origin = match #origin {
                        Ok(JValue::Object(value)) => JString::from(value),
                        _ => panic!("Wrong vec struct type.")
                    };
                }
            }
        };

        match self.ty.clone() {
            AstType::Vec(AstBaseType::Struct(ref base)) => {
                let proxy_struct_name = ident!(&format!("Proxy{}", &base));
                let real_struct_name = ident!(&base);
                quote! {
                    {
                        #value_get
                        let json: String = env.get_string(#origin).expect("Couldn't get java string!").into();
                        let vec: Vec<#proxy_struct_name> = serde_json::from_str(&json).unwrap();
                        let result: Vec<#real_struct_name> = vec.into_iter().map(|each| #real_struct_name::from(each)).collect();
                        result
                    }
                }
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
