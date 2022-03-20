use proc_macro2::{Ident, Span, TokenStream};

use crate::ast::contract::desc::{ArgDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::bridge::file::TMP_ARG_PREFIX;
use crate::ErrorKind::GenerateError;
use crate::errors::*;
use crate::swift::mapping::RustMapping;

///
/// C to Rust data convert.
///
pub(crate) fn quote_arg_convert(arg: &ArgDesc) -> Result<TokenStream> {
    let rust_arg_name = Ident::new(
        &format!("{}_{}", TMP_ARG_PREFIX, &arg.name),
        Span::call_site(),
    );
    let arg_name_ident = Ident::new(&arg.name, Span::call_site());

    let result = match arg.ty.clone() {
        AstType::Byte(origin)
        | AstType::Short(origin)
        | AstType::Int(origin)
        | AstType::Long(origin)
        | AstType::Float(origin)
        | AstType::Double(origin) => {
            let origin_type_ident = Ident::new(&origin, Span::call_site());
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
            let c_str_ident = Ident::new(&format!("c_str_{}", &arg.name), Span::call_site());
            let c_slice_ident = Ident::new(&format!("c_str_{}", &arg.name), Span::call_site());
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
            let c_str_ident = Ident::new(&format!("c_str_{}", &arg.name), Span::call_site());
            let c_slice_ident = Ident::new(&format!("c_slice_{}", &arg.name), Span::call_site());
            let tmp_ident = Ident::new(&format!("c_tmp_{}", &arg.name), Span::call_site());
            let struct_name = Ident::new(&format!("Struct_{}", &origin), Span::call_site());
            let origin_struct_name = Ident::new(&origin, Span::call_site());
            quote! {
                let #c_str_ident: &CStr = unsafe{CStr::from_ptr(#arg_name_ident)};
                let #c_slice_ident: &str = #c_str_ident.to_str().unwrap();
                let #tmp_ident: Vec<#struct_name> = serde_json::from_str(&#c_slice_ident.to_owned()).unwrap();
                let #rust_arg_name = #tmp_ident.into_iter().map(|each| #origin_struct_name::from(each)).collect();
            }
        }
        AstType::Vec(_) => {
            let c_str_ident = Ident::new(&format!("c_str_{}", &arg.name), Span::call_site());
            let c_slice_ident = Ident::new(&format!("c_slice_{}", &arg.name), Span::call_site());
            quote! {
                let #c_str_ident: &CStr = unsafe{CStr::from_ptr(#arg_name_ident)};
                let #c_slice_ident: &str = #c_str_ident.to_str().unwrap();
                let #rust_arg_name = serde_json::from_str(&#c_slice_ident.to_owned()).unwrap();
            }
        }
        AstType::Callback(_) => {
            // Should not run here will handle somewhere else.
            quote! {}
        }
        AstType::Struct(origin) => {
            let c_str_ident = Ident::new(&format!("c_str_{}", &arg.name), Span::call_site());
            let c_slice_ident = Ident::new(&format!("c_slice_{}", &arg.name), Span::call_site());
            let tmp_struct = Ident::new(&format!("c_tmp_{}", &arg.name), Span::call_site());
            let struct_name = Ident::new(&format!("Struct_{}", &origin), Span::call_site());
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

pub(crate) fn quote_return_convert(ty: &AstType, ret_name: &str) -> Result<TokenStream> {
    let ret_name_ident = Ident::new(ret_name, Span::call_site());

    let result = match ty.clone() {
        AstType::Void => quote!(),
        AstType::Boolean => quote! {
            if #ret_name_ident {1} else {0}
        },
        AstType::String => quote! {
            CString::new(#ret_name_ident).unwrap().into_raw()
        },
        AstType::Vec(AstBaseType::Struct(struct_name)) => {
            let struct_ident = Ident::new(&format!("Struct_{}", &struct_name), Span::call_site());
            quote! {
                let #ret_name_ident = #ret_name_ident.into_iter().map(|each| #struct_ident::from(each)).collect::<Vec<#struct_ident>>();
                let json_ret = serde_json::to_string(&#ret_name_ident);
                CString::new(json_ret.unwrap()).unwrap().into_raw()
            }
        }
        AstType::Vec(AstBaseType::Byte(_))
        | AstType::Vec(AstBaseType::Short(_))
        | AstType::Vec(AstBaseType::Int(_))
        | AstType::Vec(AstBaseType::Long(_)) => {
            let array_ty = RustMapping::map_r2c_transfer_type(ty);
            let inner_ty = match ty.clone() {
                AstType::Vec(base) => RustMapping::map_r2c_transfer_type(&AstType::from(base)),
                _ => quote!(),
            };
            let ptr_name = Ident::new(&format!("ptr_{}", &ret_name), Span::call_site());
            let len_name = Ident::new(&format!("len_{}", &ret_name), Span::call_site());
            quote! {
                #ret_name_ident.shrink_to_fit();
                let #ptr_name = #ret_name_ident.as_ptr();
                let #len_name = #ret_name_ident.len();
                unsafe {
                    std::mem::forget(#ret_name_ident);
                    #array_ty {
                        ptr: #ptr_name as (*const #inner_ty),
                        len: #len_name as i32,
                    }
                }
            }
        }
        AstType::Vec(_) => {
            quote! {
                let json_ret = serde_json::to_string(&#ret_name_ident);
                CString::new(json_ret.unwrap()).unwrap().into_raw()
            }
        }
        AstType::Struct(origin) => {
            let struct_copy_name = Ident::new(&format!("Struct_{}", &origin), Span::call_site());
            quote! {
                let json_ret = serde_json::to_string(&#struct_copy_name::from(#ret_name_ident));
                CString::new(json_ret.unwrap()).unwrap().into_raw()
            }
        }
        AstType::Callback(_) => {
            // Should not run here. handle it somewhere else.
            quote! {}
        }
        _ => {
            let ty_ident = RustMapping::map_r2c_transfer_type(ty);
            quote! {
                #ret_name_ident as #ty_ident
            }
        }
    };

    Ok(result)
}
