use proc_macro2::TokenStream;
use rstgen::{java, Java, Tokens};

use crate::ast::types::{AstBaseType, AstType};
use crate::base::lang::{Convertible, Direction};
use crate::java::types::JavaType;

pub(crate) struct VecDefault {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<Java<'a>> for VecDefault {
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
        if let AstType::Vec(ref base) = self.ty.clone() {
            let base = match base {
                AstBaseType::Boolean => java::BOOLEAN,
                AstBaseType::Byte(_) => java::BYTE,
                AstBaseType::Short(_) => java::SHORT,
                AstBaseType::Int(_) => java::INTEGER,
                AstBaseType::Long(_) => java::LONG,
                AstBaseType::Float(_) => java::FLOAT,
                AstBaseType::Double(_) => java::DOUBLE,
                AstBaseType::String => java::imported("java.lang", "String"),
                AstBaseType::Void => java::VOID,
                AstBaseType::Callback(ref origin) | AstBaseType::Struct(ref origin) => {
                    java::local(origin.origin.clone())
                }
            };
            let json = java::imported("com.google.gson", "Gson");
            return toks!(
                "new ",
                json,
                "().fromJson(",
                origin,
                ", ",
                base.as_boxed(),
                "[].class)"
            );
        }

        toks!("")
    }

    fn rust_to_transferable(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        match direction {
            Direction::Down => {
                quote! {{
                    let json = serde_json::to_string(&#origin);
                    env.new_string(json.unwrap()).expect("Couldn't create java string").into_inner()
                }}
            }
            Direction::Up => {
                quote! {{
                    let json = serde_json::to_string(&#origin);
                    env.new_string(json.unwrap()).expect("Couldn't create java string").into()
                }}
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
                        _ => panic!("Wrong vec default type.")
                    };
                }
            }
        };
        quote! {{
            #value_get
            let json: String = env.get_string(#origin).expect("Couldn't get java string!").into();
            serde_json::from_str(&json).unwrap()
        }}
    }

    fn native_type(&self) -> Java<'a> {
        match self.ty.clone() {
            AstType::Vec(base) => JavaType::new(AstType::from(base.clone())).to_boxed_array(),
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
