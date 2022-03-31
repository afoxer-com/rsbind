use proc_macro2::{Ident, Span, TokenStream};

use crate::ast::contract::desc::{ArgDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::bridge::file::TMP_ARG_PREFIX;
use crate::errors::*;
use crate::ident;
use crate::swift::mapping::RustMapping;
use crate::ErrorKind::GenerateError;

///
/// C to Rust data convert.
///
pub(crate) fn quote_arg_convert(arg: &ArgDesc, callbacks: &[&TraitDesc]) -> Result<TokenStream> {
    let rust_arg_name = ident!(&format!("{}_{}", TMP_ARG_PREFIX, &arg.name));
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
            let c_str_ident = ident!(&format!("c_str_{}", &arg.name));
            let c_slice_ident = ident!(&format!("c_str_{}", &arg.name));
            quote! {
                let #c_str_ident: &CStr = unsafe{CStr::from_ptr(#arg_name_ident)};
                let #c_slice_ident: &str = #c_str_ident.to_str().unwrap();
                let #rust_arg_name: String = #c_slice_ident.to_owned();
            }
        }
        AstType::Vec(AstBaseType::Byte(origin)) => {
            if origin.contains("i8") {
                quote! {
                    let #rust_arg_name = unsafe { std::slice::from_raw_parts(#arg_name_ident.ptr as (*const i8), #arg_name_ident.len as usize).to_vec() };
                }
            } else {
                quote! {
                    let #rust_arg_name = unsafe { std::slice::from_raw_parts(#arg_name_ident.ptr as (*const u8), #arg_name_ident.len as usize).to_vec() };
                }
            }
        }
        AstType::Vec(AstBaseType::Short(origin)) => {
            if origin.contains("i16") {
                quote! {
                    let #rust_arg_name = unsafe { std::slice::from_raw_parts(#arg_name_ident.ptr as (*const i16), #arg_name_ident.len as usize).to_vec() };
                }
            } else {
                quote! {
                    let #rust_arg_name = unsafe { std::slice::from_raw_parts(#arg_name_ident.ptr as (*const u16), #arg_name_ident.len as usize).to_vec() };
                }
            }
        }
        AstType::Vec(AstBaseType::Int(origin)) => {
            if origin.starts_with('i') {
                quote! {
                    let #rust_arg_name = unsafe { std::slice::from_raw_parts(#arg_name_ident.ptr as (*const i32), #arg_name_ident.len as usize).to_vec() };
                }
            } else {
                quote! {
                    let #rust_arg_name = unsafe { std::slice::from_raw_parts(#arg_name_ident.ptr as (*const u32), #arg_name_ident.len as usize).to_vec() };
                }
            }
        }
        AstType::Vec(AstBaseType::Long(origin)) => {
            if origin.starts_with('i') {
                quote! {
                    let #rust_arg_name = unsafe { std::slice::from_raw_parts(#arg_name_ident.ptr as (*const i64), #arg_name_ident.len as usize).to_vec() };
                }
            } else {
                quote! {
                    let #rust_arg_name = unsafe { std::slice::from_raw_parts(#arg_name_ident.ptr as (*const u64), #arg_name_ident.len as usize).to_vec() };
                }
            }
        }
        AstType::Vec(AstBaseType::Struct(origin)) => {
            let c_str_ident = ident!(&format!("c_str_{}", &arg.name));
            let c_slice_ident = ident!(&format!("c_slice_{}", &arg.name));
            let tmp_ident = ident!(&format!("c_tmp_{}", &arg.name));
            let struct_name = ident!(&format!("Struct_{}", &origin));
            let origin_struct_name = ident!(&origin);
            quote! {
                let #c_str_ident: &CStr = unsafe{CStr::from_ptr(#arg_name_ident)};
                let #c_slice_ident: &str = #c_str_ident.to_str().unwrap();
                let #tmp_ident: Vec<#struct_name> = serde_json::from_str(&#c_slice_ident.to_owned()).unwrap();
                let #rust_arg_name = #tmp_ident.into_iter().map(|each| #origin_struct_name::from(each)).collect();
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
            let c_str_ident = ident!(&format!("c_str_{}", &arg.name));
            let c_slice_ident = ident!(&format!("c_slice_{}", &arg.name));
            let tmp_struct = ident!(&format!("c_tmp_{}", &arg.name));
            let struct_name = ident!(&format!("Struct_{}", &origin));
            quote! {
                let #c_str_ident: &CStr = unsafe{CStr::from_ptr(#arg_name_ident)};
                let #c_slice_ident: &str = #c_str_ident.to_str().unwrap();
                let #tmp_struct: #struct_name = serde_json::from_str(&#c_slice_ident.to_owned()).unwrap();
                let #rust_arg_name = #tmp_struct.into();
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
            let r_result = CString::new(#ret_name_ident).unwrap().into_raw();
        },
        AstType::Vec(AstBaseType::Struct(struct_name)) => {
            let struct_ident = ident!(&format!("Struct_{}", &struct_name));
            quote! {
                let #ret_name_ident = #ret_name_ident.into_iter().map(|each| #struct_ident::from(each)).collect::<Vec<#struct_ident>>();
                let json_ret = serde_json::to_string(&#ret_name_ident);
                let r_result = CString::new(json_ret.unwrap()).unwrap().into_raw();
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
            let struct_copy_name = ident!(&format!("Struct_{}", &origin));
            quote! {
                let json_ret = serde_json::to_string(&#struct_copy_name::from(#ret_name_ident));
                let r_result = CString::new(json_ret.unwrap()).unwrap().into_raw();
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
