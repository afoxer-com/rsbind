use proc_macro2::TokenStream;
use rstgen::{Java, Tokens};

use crate::ast::types::{AstBaseType, AstType};
use crate::base::{Convertible, Direction};
use crate::java::types::JavaType;

pub(crate) struct VecByte {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<Java<'a>> for VecByte {
    fn native_to_transferable(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        let mut body = Tokens::new();
        body.append(origin);
        body
    }

    fn transferable_to_native(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        let mut body = Tokens::new();
        body.append(origin);
        body
    }

    fn rust_to_transferable(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        match self.ty.clone() {
            AstType::Vec(AstBaseType::Byte(ref base)) => {
                if base.contains("i8") {
                    quote! {{
                        let slice = #origin.as_slice();
                        let converted = unsafe {
                            std::mem::transmute::<&[i8], &[u8]>(slice)
                        };
                        env.byte_array_from_slice(converted).unwrap()
                    }}
                } else {
                    quote! {
                        env.byte_array_from_slice(&#origin).unwrap()
                    }
                }
            }
            _ => {
                quote! {}
            }
        }
    }

    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        let buffer_get = match direction {
            Direction::Down => {
                quote! {}
            }
            Direction::Up => {
                quote! {
                    let #origin = match #origin {
                        Ok(JValue::Object(value)) => {
                            value.into_inner() as jbyteArray
                        }
                        _ => panic!("Wrong vec byte type.")
                    };
                }
            }
        };

        match self.ty.clone() {
            AstType::Vec(AstBaseType::Byte(ref base)) => {
                if base.contains("i8") {
                    quote! {{
                        #buffer_get
                        let mut byte_array = env.convert_byte_array(#origin).unwrap();
                        let mut_ptr = byte_array.as_mut_ptr();
                        let len = byte_array.len();
                        let cap = byte_array.capacity();
                        unsafe {
                            std::mem::forget(byte_array);
                            Vec::from_raw_parts(mut_ptr as (* mut i8), len, cap)
                        }
                    }}
                } else {
                    quote! {{
                        #buffer_get
                        env.convert_byte_array(#origin).unwrap()
                    }}
                }
            }
            _ => {
                quote! {}
            }
        }
    }

    fn native_type(&self) -> Java<'a> {
        JavaType::new(AstType::Byte("i8".to_string())).to_array()
    }

    fn quote_common_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_artifact(&self) -> Tokens<'static, Java<'a>> {
        Tokens::new()
    }
}
