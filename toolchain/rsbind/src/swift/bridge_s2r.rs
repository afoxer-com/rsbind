///!
///! Swift to Rust data convert.
///!
///
use proc_macro2::{Ident, Span, TokenStream};

use crate::ast::contract::desc::{ArgDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::errors::*;
use crate::ident;
use crate::swift::mapping::RustMapping;
use crate::ErrorKind::GenerateError;

pub(crate) fn quote_arg_convert(arg: &ArgDesc, callbacks: &[&TraitDesc]) -> Result<TokenStream> {
    let rust_arg_name = ident!(&format!("r_{}", &arg.name));
    let arg_name_ident = ident!(&arg.name);

    let result = match arg.ty.clone() {
        AstType::Byte(origin)
        | AstType::Short(origin)
        | AstType::Int(origin)
        | AstType::Long(origin)
        | AstType::Float(origin)
        | AstType::Double(origin) => {
            let origin_type_ident = ident!(&origin);
            quote! {
                let #rust_arg_name = #arg_name_ident as #origin_type_ident;
            }
        }
        AstType::Boolean => {
            quote! {
                let #rust_arg_name = if #arg_name_ident > 0 {true} else {false};
            }
        }
        AstType::String => {
            quote! {
                let #rust_arg_name = {
                    let slice = unsafe {std::slice::from_raw_parts(#arg_name_ident.ptr as (*const u8), #arg_name_ident.len as usize)};
                    let cstr = unsafe {CStr::from_bytes_with_nul_unchecked(slice)};
                    cstr.to_string_lossy().to_string()
                };
            }
        }
        AstType::Vec(AstBaseType::Byte(origin))
        | AstType::Vec(AstBaseType::Short(origin))
        | AstType::Vec(AstBaseType::Int(origin))
        | AstType::Vec(AstBaseType::Long(origin)) => {
            let base_ident = ident!(&origin);
            quote! {
                let #rust_arg_name = unsafe { std::slice::from_raw_parts(#arg_name_ident.ptr as (*const #base_ident), #arg_name_ident.len as usize).to_vec() };
            }
        }
        AstType::Vec(AstBaseType::Struct(origin)) => {
            let tmp_ident = ident!(&format!("vec_tmp_{}", &arg.name));
            let struct_name = ident!(&format!("Proxy{}", &origin));
            let origin_struct_name = ident!(&origin);
            quote! {
                let #tmp_ident: Vec<#struct_name> = unsafe {Vec::from_raw_parts(#arg_name_ident.ptr as *mut #struct_name, #arg_name_ident.len as usize, #arg_name_ident.len as usize)};
                let #rust_arg_name: Vec<#origin_struct_name> = #tmp_ident.into_iter().map(|each| each.into()).collect();
            }
        }
        AstType::Vec(_) => {
            let c_str_ident = ident!(&format!("c_str_{}", &arg.name));
            let c_slice_ident = ident!(&format!("c_slice_{}", &arg.name));
            quote! {
                let #c_str_ident: &CStr = unsafe{CStr::from_ptr(#arg_name_ident)};
                let #c_slice_ident: &str = #c_str_ident.to_str().unwrap();
                let #rust_arg_name = serde_json::from_str(&#c_slice_ident.to_owned()).unwrap();
            }
        }
        AstType::Callback(ref origin) => {
            let model_to_box_fn = ident!(&format!("model_to_box_{}", origin));
            quote! {
                let #rust_arg_name = #model_to_box_fn(#arg_name_ident);
            }
        }
        AstType::Struct(origin) => {
            quote! {
                let #rust_arg_name = #arg_name_ident.into();
            }
        }
        AstType::Void => {
            return Err(
                GenerateError(format!("find unsupported type in arg, {:?}", &arg.ty)).into(),
            );
        }
    };

    Ok(result)
}

pub(crate) fn quote_callback_struct(
    callback: &TraitDesc,
    callbacks: &[&TraitDesc],
    name: &str,
) -> Result<TokenStream> {
    let callback_ident = ident!(name);

    let callback_struct_sig = quote! {
        pub struct #callback_ident
    };

    let mut callback_methods = TokenStream::new();
    for method in callback.methods.iter() {
        let callback_method_ident = ident!(&method.name);
        let ret_ty_tokens = RustMapping::map_transfer_type(&method.return_type, callbacks);
        let arg_types = method
            .args
            .iter()
            .filter(|arg| !matches!(arg.ty, AstType::Void))
            .map(|arg| RustMapping::map_transfer_type(&arg.ty, callbacks))
            .collect::<Vec<TokenStream>>();

        callback_methods = quote! {
            #callback_methods
            pub #callback_method_ident: extern "C" fn(i64, #(#arg_types),*) -> #ret_ty_tokens,
        }
    }

    let callback_struct = quote! {
        #callback_struct_sig {
            #callback_methods
            pub free_callback: extern "C" fn(i64),
            pub free_ptr: extern "C" fn(* mut i8, i32),
            pub index: i64,

        }
    };

    Ok(callback_struct)
}

pub(crate) fn quote_return_convert(
    ty: &AstType,
    callbacks: &[&TraitDesc],
    ret_name: &str,
) -> Result<TokenStream> {
    let ret_name_ident = ident!(ret_name);

    let result = match ty.clone() {
        AstType::Void => quote! {
            let r_result = #ret_name_ident;
        },
        AstType::Boolean => quote! {
            let r_result = if #ret_name_ident {1} else {0};
        },
        AstType::String => quote! {
            let r_result = {
                let cstr = CString::new(#ret_name_ident).unwrap();
                let bytes = cstr.as_bytes_with_nul();
                let array = CInt8Array {
                    ptr: bytes.as_ptr() as (*const i8),
                    len: bytes.len() as i32
                };
                std::mem::forget(cstr);
                array
            };
        },
        AstType::Vec(AstBaseType::Struct(origin)) => {
            let struct_ident = ident!(&format!("Proxy{}", &origin));
            let origin_struct_name = ident!(&origin);
            let struct_array_name = ident!(&format!("C{}Array", &origin));
            quote! {
                let mut #ret_name_ident = #ret_name_ident.into_iter().map(|each| #struct_ident::from(each)).collect::<Vec<#struct_ident>>();
                #ret_name_ident.shrink_to_fit();
                let r_result = #struct_array_name {
                    ptr: result.as_ptr(),
                    len: result.len() as i32
                };
            }
        }
        AstType::Vec(AstBaseType::Byte(_))
        | AstType::Vec(AstBaseType::Short(_))
        | AstType::Vec(AstBaseType::Int(_))
        | AstType::Vec(AstBaseType::Long(_)) => {
            let array_ty = RustMapping::map_transfer_type(ty, callbacks);
            let inner_ty = match ty.clone() {
                AstType::Vec(base) => {
                    RustMapping::map_transfer_type(&AstType::from(base), callbacks)
                }
                _ => quote!(),
            };
            let ptr_name = ident!(&format!("ptr_{}", &ret_name));
            let len_name = ident!(&format!("len_{}", &ret_name));
            quote! {
                #ret_name_ident.shrink_to_fit();
                let #ptr_name = #ret_name_ident.as_ptr();
                let #len_name = #ret_name_ident.len();
                let r_result = unsafe {
                    std::mem::forget(#ret_name_ident);
                    #array_ty {
                        ptr: #ptr_name as (*const #inner_ty),
                        len: #len_name as i32,
                    }
                };
            }
        }
        AstType::Vec(_) => {
            quote! {
                let json_ret = serde_json::to_string(&#ret_name_ident);
                let r_result = CString::new(json_ret.unwrap()).unwrap().into_raw();
            }
        }
        AstType::Struct(origin) => {
            quote! {
                let r_result = #ret_name_ident.into();
            }
        }
        AstType::Callback(ref origin) => {
            let box_to_model_fn_name = ident!(&format!("box_to_model_{}", origin.to_string()));
            quote! {
                let r_result = #box_to_model_fn_name(callback_index);
            }
        }
        _ => {
            let ty_ident = RustMapping::map_transfer_type(ty, callbacks);
            quote! {
                let r_result = #ret_name_ident as #ty_ident;
            }
        }
    };

    Ok(result)
}
