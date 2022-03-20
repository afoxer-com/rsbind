use std::path::Path;
use proc_macro2::{Ident, Span, TokenStream};
use crate::ast::contract::desc::*;
use crate::ast::imp::desc::*;
use crate::ast::types::*;
use crate::bridge::file::*;
use crate::errors::*;
use crate::swift::mapping::RustMapping;

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
            use std::collections::HashMap;
        })
    }

    fn quote_common_part(&self, _traits: &[TraitDesc]) -> Result<TokenStream> {
        Ok(quote! {
            lazy_static! {
                static ref CALLBACK_HASHMAP: std::sync::RwLock<HashMap<i64, CallbackEnum>> =  std::sync::RwLock::new(HashMap::new());
                static ref CALLBACK_INDEX : std::sync::RwLock<i64> = std::sync::RwLock::new(0);
            }
        })
    }

    fn quote_for_all_cb(&self, callbacks: &[&TraitDesc]) -> Result<TokenStream> {
        let enum_items = callbacks
            .iter()
            .map(|item| Ident::new(&item.name, Span::call_site()))
            .collect::<Vec<Ident>>();
        Ok(quote! {
            enum CallbackEnum {
                #(#enum_items(Box<dyn #enum_items>)),*
            }
        })
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
                _ => RustMapping::map_c2r_transfer_type(&arg.ty),
            })
            .collect::<Vec<TokenStream>>();

        let ret_ty_tokens = match method.return_type.clone() {
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

            _ => RustMapping::map_r2c_transfer_type(&method.return_type),
        };

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
        return match arg.ty.clone() {
            AstType::Callback(origin) => {
                println!("callback in argument found, {}", &origin);
                self.callback_strategy
                    .arg_convert(arg, trait_desc, callbacks)
            }
            _ => crate::swift::bridge_c2r::quote_arg_convert(arg),
        };
    }

    fn quote_return_convert(
        &self,
        trait_desc: &TraitDesc,
        callbacks: &[&TraitDesc],
        return_ty: &AstType,
        ret_name: &str,
    ) -> Result<TokenStream> {
        return match return_ty.clone() {
            AstType::Callback(origin) => {
                println!("callback in argument found, {}", &origin);
                self.callback_strategy
                    .return_convert(return_ty, trait_desc, callbacks)
            }
            _ => crate::swift::bridge_c2r::quote_return_convert(return_ty, ret_name),
        };
    }

    fn ty_to_tokens(&self, _ast_type: &AstType, _direction: TypeDirection) -> Result<TokenStream> {
        // We don't use it.
        Ok(quote!())
    }
}
