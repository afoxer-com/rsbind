use std::fmt::format;
use std::path::Path;

use crate::ast::contract::desc::*;
use crate::ast::imp::desc::*;
use crate::ast::types::*;
use crate::bridge::file::*;
use crate::errors::ErrorKind::*;
use crate::errors::*;
use crate::swift::mapping::RustMapping;
use proc_macro2::{Ident, Span, TokenStream};

use super::bridge_cb::*;

///
/// create a new c bridges generator.
///
pub(crate) fn new_gen<'a>(
    out_dir: &'a Path,
    trait_descs: &'a [TraitDesc],
    struct_descs: &'a [StructDesc],
    imp_desc: &'a [ImpDesc],
) -> BridgeFileGen<'a, CFileGenStrategy> {
    BridgeFileGen {
        out_dir,
        trait_descs,
        struct_descs,
        imp_desc,
        strategy: CFileGenStrategy {
            callback_strategy: CCallbackStrategy {},
        },
    }
}

///
/// c bridge file generate strategy.
///
pub struct CFileGenStrategy {
    pub callback_strategy: CCallbackStrategy,
}

impl CFileGenStrategy {}

impl FileGenStrategy for CFileGenStrategy {
    fn gen_sdk_file(&self, _mod_names: &[String]) -> Result<TokenStream> {
        Ok(quote!())
    }

    fn quote_common_use_part(&self) -> Result<TokenStream> {
        Ok(quote! {
            use std::ffi::CStr;
            use std::os::raw::c_char;
            use std::ffi::CString;
            use c::bridge::common::*;
        })
    }

    fn quote_common_part(&self, _traits: &[TraitDesc]) -> Result<TokenStream> {
        Ok(quote! {})
    }

    fn quote_callback_structures(&self, trait_desc: &TraitDesc) -> Result<TokenStream> {
        let callback_str = &format!("{}_{}_Model", &trait_desc.mod_name, &trait_desc.name);
        let callback_struct = self
            .callback_strategy
            .quote_callback_struct(trait_desc, callback_str)?;
        Ok(quote! {
            #[repr(C)]
            #callback_struct
        })
    }

    fn quote_for_structures(&self, struct_desc: &StructDesc) -> Result<TokenStream> {
        let struct_name = Ident::new(&format!("Struct_{}", &struct_desc.name), Span::call_site());
        let origin_struct_name = Ident::new(&struct_desc.name, Span::call_site());
        let names = struct_desc
            .fields
            .iter()
            .map(|field| Ident::new(&field.name, Span::call_site()))
            .collect::<Vec<Ident>>();
        let arg_names = names.clone();
        let origin_arg_names = names.clone();
        let tys = struct_desc
            .fields
            .iter()
            .map(|field| Ident::new(&field.ty.origin(), Span::call_site()))
            .collect::<Vec<Ident>>();
        Ok(quote! {
            #[repr(C)]
            #[derive(Serialize, Deserialize)]
            pub struct #struct_name {
                #(pub #names: #tys),*
            }

            impl From<#origin_struct_name> for #struct_name {
                fn from(origin: #origin_struct_name) -> Self {
                    #struct_name{#(#origin_arg_names: origin.#arg_names),*}
                }
            }

            impl From<#struct_name> for #origin_struct_name {
                fn from(origin: #struct_name) -> Self {
                    #origin_struct_name{#(#origin_arg_names: origin.#arg_names),*}
                }
            }
        })
    }

    fn quote_method_sig(
        &self,
        trait_desc: &TraitDesc,
        _impl_desc: &ImpDesc,
        method: &MethodDesc,
        callbacks: &[&TraitDesc],
        _structs: &[StructDesc],
    ) -> Result<TokenStream> {
        let fun_name = Ident::new(
            &format!("{}_{}", &trait_desc.mod_name, &method.name),
            Span::call_site(),
        );

        let arg_names = method
            .args
            .iter()
            .filter(|arg| !matches!(arg.ty, AstType::Void))
            .map(|arg| Ident::new(&arg.name, Span::call_site()))
            .collect::<Vec<Ident>>();

        let arg_types = method
            .args
            .iter()
            .filter(|arg| !matches!(arg.ty, AstType::Void))
            .map(|arg| match arg.ty.clone() {
                AstType::Callback(origin) => {
                    let mut callback_trait = None;
                    for callback in callbacks.iter() {
                        if callback.name == origin.clone() {
                            callback_trait = Some(callback);
                            break;
                        }
                    }
                    let callback_str = &format!(
                        "{}_{}_Model",
                        &callback_trait.unwrap().mod_name,
                        &callback_trait.unwrap().name
                    );
                    let callback_ident = Ident::new(callback_str, Span::call_site());
                    quote!(#callback_ident)
                }
                _ => RustMapping::map_arg_transfer_type(&arg.ty),
            })
            .collect::<Vec<TokenStream>>();

        let ret_ty_tokens = RustMapping::map_return_transfer_type(&method.return_type);
        let sig_define = if arg_names.is_empty() {
            match method.return_type {
                AstType::Void => quote! {
                    #[no_mangle]
                    pub extern "C" fn #fun_name()
                },
                _ => quote! {
                    #[no_mangle]
                    pub extern "C" fn #fun_name() -> #ret_ty_tokens
                },
            }
        } else {
            match method.return_type {
                AstType::Void => quote! {
                    #[no_mangle]
                    pub extern "C" fn #fun_name(#(#arg_names: #arg_types),*)
                },
                _ => quote! {
                    #[no_mangle]
                    pub extern "C" fn #fun_name(#(#arg_names: #arg_types),*) -> #ret_ty_tokens
                },
            }
        };

        Ok(sig_define)
    }

    fn quote_arg_convert(
        &self,
        trait_desc: &TraitDesc,
        arg: &ArgDesc,
        callbacks: &[&TraitDesc],
    ) -> Result<TokenStream> {
        let rust_arg_name = Ident::new(
            &format!("{}_{}", TMP_ARG_PREFIX, &arg.name),
            Span::call_site(),
        );
        let arg_name_ident = Ident::new(&arg.name, Span::call_site());

        let result = match arg.clone().ty {
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
                if origin.starts_with("i") {
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
                if origin.starts_with("i") {
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
                let c_slice_ident =
                    Ident::new(&format!("c_slice_{}", &arg.name), Span::call_site());
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
                let c_slice_ident =
                    Ident::new(&format!("c_slice_{}", &arg.name), Span::call_site());
                quote! {
                    let #c_str_ident: &CStr = unsafe{CStr::from_ptr(#arg_name_ident)};
                    let #c_slice_ident: &str = #c_str_ident.to_str().unwrap();
                    let #rust_arg_name = serde_json::from_str(&#c_slice_ident.to_owned()).unwrap();
                }
            }
            AstType::Callback(origin) => {
                println!("callback in argument found, {}", &origin);
                self.callback_strategy
                    .arg_convert(arg, trait_desc, callbacks)
            }
            AstType::Struct(origin) => {
                let c_str_ident = Ident::new(&format!("c_str_{}", &arg.name), Span::call_site());
                let c_slice_ident =
                    Ident::new(&format!("c_slice_{}", &arg.name), Span::call_site());
                let tmp_struct = Ident::new(&format!("c_tmp_{}", &arg.name), Span::call_site());
                let struct_name = Ident::new(&format!("Struct_{}", &origin), Span::call_site());
                quote! {
                    let #c_str_ident: &CStr = unsafe{CStr::from_ptr(#arg_name_ident)};
                    let #c_slice_ident: &str = #c_str_ident.to_str().unwrap();
                    let #tmp_struct: #struct_name = serde_json::from_str(&#c_slice_ident.to_owned()).unwrap();
                    let #rust_arg_name = #tmp_struct.into();
                }
            }
            _ => {
                return Err(
                    GenerateError(format!("find unsupported type in arg, {:?}", &arg.ty)).into(),
                );
            }
        };

        Ok(result)
    }

    fn quote_return_convert(&self, ty: &AstType, ret_name: &str) -> Result<TokenStream> {
        let ret_name_ident = Ident::new(ret_name, Span::call_site());

        Ok(match ty.clone() {
            AstType::Void => quote!(),
            AstType::Boolean => quote! {
                if #ret_name_ident {1} else {0}
            },
            AstType::String => quote! {
                CString::new(#ret_name_ident).unwrap().into_raw()
            },
            AstType::Vec(AstBaseType::Struct(struct_name)) => {
                let struct_ident =
                    Ident::new(&format!("Struct_{}", &struct_name), Span::call_site());
                quote! {
                    let #ret_name_ident = #ret_name_ident.into_iter().map(|each| #struct_ident::from(each)).collect::<Vec<#struct_ident>>();
                    let json_ret = serde_json::to_string(&#ret_name_ident);
                    CString::new(json_ret.unwrap()).unwrap().into_raw()
                }
            }
            AstType::Vec(AstBaseType::Byte(ref origin))
            | AstType::Vec(AstBaseType::Short(ref origin))
            | AstType::Vec(AstBaseType::Int(ref origin))
            | AstType::Vec(AstBaseType::Long(ref origin)) => {
                let array_ty = RustMapping::map_return_transfer_type(&ty);
                let inner_ty = match ty.clone() {
                    AstType::Vec(base) => {
                        RustMapping::map_return_transfer_type(&AstType::from(base))
                    }
                    _ => quote!()
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
                let struct_copy_name =
                    Ident::new(&format!("Struct_{}", &origin), Span::call_site());
                quote! {
                    let json_ret = serde_json::to_string(&#struct_copy_name::from(#ret_name_ident));
                    CString::new(json_ret.unwrap()).unwrap().into_raw()
                }
            }
            _ => {
                let ty_ident = RustMapping::map_return_transfer_type(ty);
                quote! {
                    #ret_name_ident as #ty_ident
                }
            }
        })
    }

    fn ty_to_tokens(&self, _ast_type: &AstType, _direction: TypeDirection) -> Result<TokenStream> {
        // We don't use it.
        Ok(quote!())
    }
}
