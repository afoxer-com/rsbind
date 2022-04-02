///!
///! Rust to Swift data convert.
///!
use proc_macro2::{Ident, Span, TokenStream};

use crate::ast::contract::desc::{ArgDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::errors::*;
use crate::ident;
use crate::swift::mapping::RustMapping;

pub(crate) fn arg_convert(arg: &ArgDesc, callbacks: &[&TraitDesc]) -> Result<TokenStream> {
    let cb_arg_name = ident!(&format!("c_{}", arg.name));
    let cb_origin_arg_name = ident!(&arg.name);
    Ok(match arg.ty.clone() {
        AstType::Boolean => {
            quote! {
                let #cb_arg_name = if #cb_origin_arg_name {1} else {0};
            }
        }
        AstType::String => {
            let ptr_arg = ident!(&format!("ptr_{}", &arg.name));
            quote! {
                let #cb_arg_name = {
                    let cstr = CString::new(#cb_origin_arg_name).unwrap();
                    let bytes = cstr.as_bytes_with_nul();
                    let array = CInt8Array {
                        ptr: bytes.as_ptr() as (*const i8),
                        len: bytes.len() as i32
                    };
                    std::mem::forget(cstr);
                    array
                };
                let #ptr_arg = #cb_arg_name.ptr;
            }
        }
        AstType::Vec(AstBaseType::Byte(_))
        | AstType::Vec(AstBaseType::Short(_))
        | AstType::Vec(AstBaseType::Int(_))
        | AstType::Vec(AstBaseType::Long(_)) => {
            let base_ty = match arg.ty.clone() {
                AstType::Vec(base) => {
                    RustMapping::map_base_transfer_type(&AstType::from(base.clone()))
                }
                _ => quote!(),
            };
            let array_ty = RustMapping::map_base_transfer_type(&arg.ty);
            quote! {
                let #cb_arg_name = unsafe {
                    #array_ty {
                        ptr: #cb_origin_arg_name.as_ptr() as (*const #base_ty),
                        len: #cb_origin_arg_name.len() as i32
                    }
                };
            }
        }
        AstType::Vec(AstBaseType::Struct(origin)) => {
            let struct_ident = ident!(&format!("Proxy{}", &origin));
            let cb_tmp_vec_arg_name = ident!(&format!("c_tmp_vec_{}", arg.name));
            let struct_array_name = ident!(&format!("C{}Array", origin));
            quote! {
                let mut #cb_tmp_vec_arg_name = #cb_origin_arg_name.into_iter().map(|each| #struct_ident::from(each)).collect::<Vec<#struct_ident>>();
                #cb_tmp_vec_arg_name.shrink_to_fit();
                let #cb_arg_name = #struct_array_name {
                    ptr: #cb_tmp_vec_arg_name.as_ptr(),
                    len: #cb_tmp_vec_arg_name.len() as i32
                };
            }
        }
        AstType::Vec(_) => {
            let cb_tmp_arg_name = ident!(&format!("c_tmp_{}", arg.name));
            quote! {
                let #cb_tmp_arg_name = serde_json::to_string(&#cb_origin_arg_name);
                let #cb_arg_name = CString::new(#cb_tmp_arg_name.unwrap()).unwrap().into_raw();
            }
        }
        AstType::Struct(origin) => {
            quote! {
                let #cb_arg_name = #cb_origin_arg_name.into();
            }
        }
        AstType::Callback(ref origin) => {
            let return_cb_fn_name = ident!(&format!("box_to_model_{}", origin));
            quote! {
                let #cb_arg_name = #return_cb_fn_name(callback_index);
            }
        }
        AstType::Void
        | AstType::Byte(_)
        | AstType::Int(_)
        | AstType::Short(_)
        | AstType::Long(_)
        | AstType::Float(_)
        | AstType::Double(_) => {
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
            let ident = ident!(&base.origin());
            quote!(Vec<#ident>)
        }
        _ => {
            let ident = ident!(&return_type.origin());
            quote!(#ident)
        }
    };

    Ok(match return_type {
        AstType::Void => quote!(),
        AstType::Boolean => quote! {
            let r_result = if result > 0 {true} else {false};
        },
        AstType::String => quote! {
            let r_result = {
                let slice = unsafe {std::slice::from_raw_parts(result.ptr as (*const u8), result.len as usize)};
                let cstr = unsafe {CStr::from_bytes_with_nul_unchecked(slice)};
                cstr.to_string_lossy().to_string()
            };
        },
        AstType::Vec(AstBaseType::Byte(ref origin))
        | AstType::Vec(AstBaseType::Short(ref origin))
        | AstType::Vec(AstBaseType::Int(ref origin))
        | AstType::Vec(AstBaseType::Long(ref origin)) => {
            let origin_ident = ident!(origin);
            quote! {
                let r_result = unsafe { Vec::from_raw_parts(result.ptr as (* mut #origin_ident), result.len as usize, result.len as usize) };
            }
        }
        AstType::Vec(AstBaseType::Struct(ref origin)) => {
            let struct_ident = ident!(&format!("Proxy{}", origin));
            quote! {
                let tmp_vec_result: Vec<#struct_ident> = unsafe {Vec::from_raw_parts(result.ptr as *mut #struct_ident, result.len as usize, result.len as usize)};
                let r_result = tmp_vec_result.into_iter().map(|each| each.into()).collect();
            }
        }
        AstType::Vec(_) => {
            quote! {
                let c_str_result: &CStr = unsafe { CStr::from_ptr(result) };
                let c_slice_result: &str = c_str_result.to_str().unwrap();
                let r_result = serde_json::from_str(&c_slice_result.to_owned()).unwrap();
            }
        }
        AstType::Callback(ref origin) => {
            let arg_cb_fn_name = ident!(&format!("model_to_box_{}", origin));
            quote! {
                let r_result = #arg_cb_fn_name(result);
            }
        }
        AstType::Struct(ref origin) => {
            quote! {
                let r_result = result.into();
            }
        }
        AstType::Byte(_)
        | AstType::Int(_)
        | AstType::Short(_)
        | AstType::Long(_)
        | AstType::Float(_)
        | AstType::Double(_) => {
            quote! {
                let r_result = result as #ret_ty_tokens;
            }
        }
    })
}
