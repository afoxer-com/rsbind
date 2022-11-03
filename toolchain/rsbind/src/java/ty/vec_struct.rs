use proc_macro2::TokenStream;
use rstgen::{java, Java, Tokens};

use crate::ast::types::{AstBaseType, AstType};
use crate::base::lang::{Convertible, Direction};
use crate::ident;
use crate::java::types::JavaType;

pub(crate) struct VecStruct {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<Java<'a>> for VecStruct {
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
        let json = java::imported("com.google.gson", "Gson");
        if let AstType::Vec(AstBaseType::Struct(ref base)) = self.ty.clone() {
            return toks!(
                "new ",
                json,
                "().fromJson(",
                origin,
                ", ",
                base.origin.clone(),
                "[].class)"
            );
        }

        toks!("")
    }

    fn rust_to_transferable(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        match self.ty.clone() {
            AstType::Vec(AstBaseType::Struct(ref base)) => {
                let proxy_struct = ident!(&format!("Proxy{}", &base.origin));
                match direction {
                    Direction::Down => {
                        quote! {{
                            let proxies = #origin.into_iter().map(|each| #proxy_struct::from(each)).collect::<Vec<#proxy_struct>>();
                            let json = serde_json::to_string(&proxies);
                            env.new_string(json.unwrap()).expect("Couldn't create java string").into_inner()
                        }}
                    }
                    Direction::Up => {
                        quote! {{
                            let proxies = #origin.into_iter().map(|each| #proxy_struct::from(each)).collect::<Vec<#proxy_struct>>();
                            let json = serde_json::to_string(&proxies);
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
                        _ => panic!("Wrong vec struct type.")
                    };
                }
            }
        };

        match self.ty.clone() {
            AstType::Vec(AstBaseType::Struct(ref base)) => {
                let proxy_struct_name = ident!(&format!("Proxy{}", &base.origin));
                let real_struct_name = ident!(&base.origin);
                quote! {{
                    #value_get
                    let json: String = env.get_string(#origin).expect("Couldn't get java string!").into();
                    let vec: Vec<#proxy_struct_name> = serde_json::from_str(&json).unwrap();
                    let result: Vec<#real_struct_name> = vec.into_iter().map(|each| #real_struct_name::from(each)).collect();
                    result
                }}
            }
            _ => {
                quote! {}
            }
        }
    }

    fn native_type(&self) -> Java<'a> {
        match self.ty.clone() {
            AstType::Vec(base) => JavaType::new(AstType::from(base)).to_array(),
            _ => java::local(""),
        }
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
