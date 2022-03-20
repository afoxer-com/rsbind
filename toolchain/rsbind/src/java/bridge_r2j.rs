use proc_macro2::{Ident, Span, TokenStream};

use crate::ast::contract::desc::{ArgDesc, MethodDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::bridge::file::TypeDirection;
use crate::errors::*;

///
/// Rust to Java data convert.
///
///
pub(crate) fn arg_convert(cb_arg: &ArgDesc) -> Result<TokenStream> {
    let cb_arg_name = Ident::new(&format!("j_{}", cb_arg.name), Span::call_site());
    let cb_origin_arg_name = Ident::new(&cb_arg.name, Span::call_site());

    Ok(match cb_arg.ty.clone() {
        AstType::Boolean => {
            quote! {
                let #cb_arg_name = if #cb_origin_arg_name {1} else {0};
            }
        }
        AstType::String => {
            quote! {
                let #cb_arg_name = env.new_string(#cb_origin_arg_name).unwrap().into();
            }
        }
        AstType::Vec(ref base_ty) => {
            let cb_tmp_arg_name = Ident::new(&format!("j_tmp_{}", cb_arg.name), Span::call_site());
            match base_ty {
                AstBaseType::Struct(struct_name) => {
                    let struct_ident =
                        Ident::new(&format!("Struct_{}", &struct_name), Span::call_site());
                    let cb_tmp_vec_arg_name =
                        Ident::new(&format!("j_tmp_vec_{}", cb_arg.name), Span::call_site());
                    quote! {
                        let #cb_tmp_vec_arg_name = #cb_origin_arg_name.into_iter().map(|each| #struct_ident::from(each)).collect::<Vec<#struct_ident>>();
                        let #cb_tmp_arg_name = serde_json::to_string(&#cb_tmp_vec_arg_name);
                        let #cb_arg_name = env.new_string(#cb_tmp_arg_name.unwrap()).unwrap().into();
                    }
                }
                AstBaseType::Byte(origin) => {
                    if origin.contains("i8") {
                        let tmp_arg_name =
                            Ident::new(&format!("tmp_{}", &cb_arg.name), Span::call_site());
                        let tmp_converted_arg_name = Ident::new(
                            &format!("tmp_converted_{}", &cb_arg.name),
                            Span::call_site(),
                        );
                        vec![1u8].as_slice();
                        quote! {
                            let #tmp_arg_name = #cb_origin_arg_name.as_slice();
                            let #tmp_converted_arg_name = unsafe { std::mem::transmute::<&[i8], &[u8]>(#tmp_arg_name) };
                            let #cb_arg_name = env.byte_array_from_slice(#tmp_converted_arg_name).unwrap();
                        }
                    } else {
                        quote! {
                            let #cb_arg_name = env.byte_array_from_slice(#cb_origin_arg_name.as_slice()).unwrap();
                        }
                    }
                }
                _ => {
                    quote! {
                        let #cb_tmp_arg_name = serde_json::to_string(&#cb_origin_arg_name);
                        let #cb_arg_name = env.new_string(#cb_tmp_arg_name.unwrap()).unwrap().into();
                    }
                }
            }
        }
        AstType::Struct(struct_name) => {
            let struct_copy_name =
                Ident::new(&format!("Struct_{}", &struct_name), Span::call_site());
            let cb_tmp_arg_name = Ident::new(&format!("r_tmp_{}", cb_arg.name), Span::call_site());
            quote! {
                let #cb_tmp_arg_name = serde_json::to_string(&#struct_copy_name::from(#cb_origin_arg_name));
                let #cb_arg_name = env.new_string(#cb_tmp_arg_name.unwrap()).unwrap().into();
            }
        }
        _ => {
            let arg_ty_ident = ty_to_tokens(&cb_arg.ty, TypeDirection::Argument).unwrap();
            quote! {
                let #cb_arg_name = #cb_origin_arg_name as #arg_ty_ident;
            }
        }
    })
}

pub(crate) fn return_convert(method: &MethodDesc) -> Result<TokenStream> {
    let ret_ty_tokens = match method.return_type {
        AstType::Void => quote!(()),
        AstType::Vec(ref base) => {
            let origin_ident = Ident::new(&base.origin(), Span::call_site());
            quote!(Vec<#origin_ident>)
        }
        _ => {
            let ident = Ident::new(&method.return_type.origin(), Span::call_site());
            quote!(#ident)
        }
    };

    Ok(match method.return_type.clone() {
        AstType::Void => quote!(),
        AstType::Boolean => quote! {
            let mut r_result = None;
            match result.unwrap() {
                JValue::Int(value) => r_result = Some(value),
                _ => assert!(false)
            }

            let s_result = if r_result.unwrap() > 0 {true} else {false};
        },

        AstType::Byte(origin) => {
            let origin_return_ty_ident = Ident::new(&origin, Span::call_site());
            quote! {
                let mut r_result = None;
                match result.unwrap() {
                    JValue::Byte(value) => r_result = Some(value),
                    _ => assert!(false)
                }

                let s_result = r_result.unwrap() as #origin_return_ty_ident;
            }
        }
        AstType::Short(origin) => {
            let origin_return_ty_ident = Ident::new(&origin, Span::call_site());
            quote! {
                let mut r_result = None;
                match result.unwrap() {
                    JValue::Short(value) => r_result = Some(value),
                    _ => assert!(false)
                }

                let s_result = r_result.unwrap() as #origin_return_ty_ident;
            }
        }
        AstType::Int(origin) => {
            let origin_return_ty_ident = Ident::new(&origin, Span::call_site());
            quote! {
                let mut r_result = None;
                match result.unwrap() {
                    JValue::Int(value) => r_result = Some(value),
                    _ => assert!(false)
                }

                let s_result = r_result.unwrap() as #origin_return_ty_ident;
            }
        }
        AstType::Long(origin) => {
            let origin_return_ty_ident = Ident::new(&origin, Span::call_site());
            quote! {
                let mut r_result = None;
                match result.unwrap() {
                    JValue::Long(value) => r_result = Some(value),
                    _ => assert!(false)
                }

                let s_result = r_result.unwrap() as #origin_return_ty_ident;
            }
        }
        AstType::Float(origin) => {
            let origin_return_ty_ident = Ident::new(&origin, Span::call_site());
            quote! {
                let mut r_result = None;
                match result.unwrap() {
                    JValue::Float(value) => r_result = Some(value),
                    _ => assert!(false)
                }

                let s_result = r_result.unwrap() as #origin_return_ty_ident;
            }
        }
        AstType::Double(origin) => {
            let origin_return_ty_ident = Ident::new(&origin, Span::call_site());
            quote! {
                let mut r_result = None;
                match result.unwrap() {
                    JValue::Double(value) => r_result = Some(value),
                    _ => assert!(false)
                }
                let s_result = r_result.unwrap() as #origin_return_ty_ident;
            }
        }
        AstType::String => {
            quote! {
                let mut r_result = None;
                match result.unwrap() {
                    JValue::Object(value) => r_result = Some(value),
                    _ => assert!(false)
                }

                let jstr = JString::from(r_result.unwrap());
                let s_result = env.get_string(jstr).unwrap().to_string_lossy().to_string();
            }
        }
        AstType::Vec(AstBaseType::Byte(ref origin)) => {
            let mut tokens = TokenStream::new();
            let buffer_get = quote! {
                let mut r_result = None;
                match result.unwrap() {
                    JValue::Object(value) => r_result = Some(value),
                    _ => assert!(false)
                }

                let jarray_result = r_result.unwrap().into_inner() as jbyteArray;
                let jarray_count = env.get_array_length(jarray_result).unwrap();
                let mut array_buffer: Vec<i8> = Vec::with_capacity(jarray_count as usize);
                env.get_byte_array_region(jarray_result, 0, array_buffer.as_mut_slice());
            };

            if origin.starts_with('u') {
                tokens = quote! {
                    #buffer_get

                    let mut array_buffer = std::mem::ManuallyDrop::new(array_buffer);
                    let array_buffer_p = array_buffer.as_mut_ptr();
                    let array_buffer_len = array_buffer.len();
                    let array_buffer_cap = array_buffer.capacity();
                    let s_result = unsafe { Vec::from_raw_parts(array_buffer_p as *mut u8, array_buffer_len, array_buffer_cap) };
                };
            } else {
                tokens = quote! {
                    #buffer_get
                    let s_result = array_buffer;
                };
            }
            tokens
        }
        AstType::Vec(_) => {
            quote! {
                let mut r_result = None;
                match result.unwrap() {
                    JValue::Object(value) => r_result = Some(value),
                    _ => assert!(false)
                }

                let jstr_result = JString::from(r_result.unwrap());
                let rstr_result = env.get_string(jstr_result).unwrap().to_string_lossy().to_string();
                let s_result = serde_json::from_str(&rstr_result).unwrap();
            }
        }
        _ => {
            quote! {
                let s_result = result as #ret_ty_tokens;
            }
        }
    })
}

pub(crate) fn ty_to_tokens(ast_type: &AstType, direction: TypeDirection) -> Result<TokenStream> {
    Ok(match ast_type.clone() {
        AstType::Byte(_) => quote!(i8),
        AstType::Short(_) => quote!(i16),
        AstType::Int(_) => quote!(i32),
        AstType::Long(_) => quote!(i64),
        AstType::Float(_) => quote!(f32),
        AstType::Double(_) => quote!(f64),
        AstType::Boolean => quote!(u8),
        AstType::String => match direction {
            TypeDirection::Argument => quote!(JString),
            TypeDirection::Return => quote!(jstring),
        },
        AstType::Vec(_base) => match direction {
            TypeDirection::Argument => quote!(JString),
            TypeDirection::Return => quote!(jstring),
        },
        AstType::Struct(_) => match direction {
            TypeDirection::Argument => quote!(JString),
            TypeDirection::Return => quote!(jstring),
        },
        AstType::Callback(_) => quote!(i64),
        _ => quote!(()),
    })
}
