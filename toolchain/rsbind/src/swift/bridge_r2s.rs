use proc_macro2::{Ident, Span, TokenStream};

use crate::ast::contract::desc::{ArgDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::errors::*;
use crate::swift::mapping::RustMapping;

///
/// Rust to C data convert.
///
pub(crate) fn arg_convert(arg: &ArgDesc, callbacks: &[&TraitDesc]) -> Result<TokenStream> {
    let cb_arg_name = Ident::new(&format!("c_{}", arg.name), Span::call_site());
    let cb_origin_arg_name = Ident::new(&arg.name, Span::call_site());
    Ok(match arg.ty.clone() {
        AstType::Boolean => {
            quote! {
                let #cb_arg_name = if #cb_origin_arg_name {1} else {0};
            }
        }
        AstType::String => {
            quote! {
                let #cb_arg_name = CString::new(#cb_origin_arg_name).unwrap().into_raw();
            }
        }
        AstType::Vec(AstBaseType::Byte(_)) => {
            quote! {
                let #cb_arg_name = unsafe {
                    CInt8Array {
                        ptr: #cb_origin_arg_name.as_ptr() as (*const i8),
                        len: #cb_origin_arg_name.len() as i32
                    }
                };
            }
        }
        AstType::Vec(AstBaseType::Short(_)) => {
            quote! {
                let #cb_arg_name = unsafe {
                    CInt16Array {
                        ptr: #cb_origin_arg_name.as_ptr() as (*const i16),
                        len: #cb_origin_arg_name.len() as i32
                    }
                };
            }
        }
        AstType::Vec(AstBaseType::Int(_)) => {
            quote! {
                let #cb_arg_name = unsafe {
                    CInt32Array {
                        ptr: #cb_origin_arg_name.as_ptr() as (*const i32),
                        len: #cb_origin_arg_name.len() as i32
                    }
                };
            }
        }
        AstType::Vec(AstBaseType::Long(_)) => {
            quote! {
                let #cb_arg_name = unsafe {
                    CInt64Array {
                        ptr: #cb_origin_arg_name.as_ptr() as (*const i64),
                        len: #cb_origin_arg_name.len() as i32
                    }
                };
            }
        }
        AstType::Vec(AstBaseType::Struct(struct_name)) => {
            let cb_tmp_arg_name = Ident::new(&format!("c_tmp_{}", arg.name), Span::call_site());
            let struct_ident = Ident::new(&format!("Struct_{}", &struct_name), Span::call_site());
            let cb_tmp_vec_arg_name =
                Ident::new(&format!("c_tmp_vec_{}", arg.name), Span::call_site());
            quote! {
                let #cb_tmp_vec_arg_name = #cb_origin_arg_name.into_iter().map(|each| #struct_ident::from(each)).collect::<Vec<#struct_ident>>();
                let #cb_tmp_arg_name = serde_json::to_string(&#cb_tmp_vec_arg_name);
                let #cb_arg_name = CString::new(#cb_tmp_arg_name.unwrap()).unwrap().into_raw();
            }
        }
        AstType::Vec(_) => {
            let cb_tmp_arg_name = Ident::new(&format!("c_tmp_{}", arg.name), Span::call_site());
            quote! {
                let #cb_tmp_arg_name = serde_json::to_string(&#cb_origin_arg_name);
                let #cb_arg_name = CString::new(#cb_tmp_arg_name.unwrap()).unwrap().into_raw();
            }
        }
        AstType::Struct(origin) => {
            let struct_copy_name = Ident::new(&format!("Struct_{}", &origin), Span::call_site());
            let cb_tmp_arg_name = Ident::new(&format!("c_tmp_{}", arg.name), Span::call_site());
            quote! {
                let #cb_tmp_arg_name = serde_json::to_string(&#struct_copy_name::from(#cb_origin_arg_name));
                let #cb_arg_name = CString::new(#cb_tmp_arg_name.unwrap()).unwrap().into_raw();
            }
        }
        AstType::Callback(ref origin) => {
            let return_cb_fn_name =
                Ident::new(&format!("box_to_model_{}", origin), Span::call_site());
            quote! {
                let #cb_arg_name = #return_cb_fn_name(callback_index);
            }
        }
        _ => {
            let arg_ty_ident = RustMapping::map_transfer_type(&arg.ty, callbacks);
            quote! {
                let #cb_arg_name = #cb_origin_arg_name as #arg_ty_ident;
            }
        }
    })
}

pub(crate) fn return_convert(return_type: &AstType) -> Result<TokenStream> {
    let ret_ty_tokens = match return_type {
        AstType::Void => quote!(()),
        AstType::Vec(ref base) => {
            let ident = Ident::new(&base.origin(), Span::call_site());
            quote!(Vec<#ident>)
        }
        _ => {
            let ident = Ident::new(&return_type.origin(), Span::call_site());
            quote!(#ident)
        }
    };

    Ok(match return_type {
        AstType::Void => quote!(),
        AstType::Boolean => quote! {
            let r_result = if result > 0 {true} else {false};
        },
        AstType::String => quote! {
            let s_result_c_str: &CStr = unsafe { CStr::from_ptr(result) };
            let s_result_str: &str = s_result_c_str.to_str().unwrap();
            let r_result: String = s_result_str.to_owned();
        },
        AstType::Vec(AstBaseType::Byte(ref origin))
        | AstType::Vec(AstBaseType::Short(ref origin))
        | AstType::Vec(AstBaseType::Int(ref origin))
        | AstType::Vec(AstBaseType::Long(ref origin)) => {
            let origin_ident = Ident::new(origin, Span::call_site());
            match return_type.clone() {
                AstType::Vec(AstBaseType::Byte(_)) => quote!(1),
                AstType::Vec(AstBaseType::Short(_)) => quote!(2),
                AstType::Vec(AstBaseType::Int(_)) => quote!(4),
                AstType::Vec(AstBaseType::Long(_)) => quote!(8),
                _ => quote!(1),
            };
            quote! {
                let r_result = unsafe { Vec::from_raw_parts(result.ptr as (* mut #origin_ident), result.len as usize, result.len as usize) };
            }
        }
        AstType::Callback(ref origin) => {
            let arg_cb_fn_name = Ident::new(&format!("model_to_box_{}", origin), Span::call_site());
            quote! {
                let r_result = #arg_cb_fn_name(result);
            }
        }
        _ => quote! {
            let r_result = result as #ret_ty_tokens;
        },
    })
}
