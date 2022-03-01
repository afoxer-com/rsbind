use super::callback::*;
use ast::contract::desc::*;
use ast::imp::desc::*;
use ast::types::*;
use bridge::file::*;
use errors::ErrorKind::*;
use errors::*;
use proc_macro2::{Ident, Punct, Spacing, Span, TokenStream};
use quote::TokenStreamExt;
use std::path::PathBuf;

///
/// create a new c bridges generator.
///
pub(crate) fn new_gen<'a>(
    out_dir: &'a PathBuf,
    trait_descs: &'a Vec<TraitDesc>,
    struct_descs: &'a Vec<StructDesc>,
    imp_desc: &'a Vec<ImpDesc>,
) -> BridgeFileGen<'a, CFileGenStrategy> {
    return BridgeFileGen {
        out_dir,
        trait_descs,
        struct_descs,
        imp_desc,
        strategy: CFileGenStrategy {
            callback_strategy: CCallbackStrategy {},
        },
    };
}

///
/// c bridge file generate strategy.
///
pub(crate) struct CFileGenStrategy {
    pub(crate) callback_strategy: CCallbackStrategy,
}

impl CFileGenStrategy {}

impl FileGenStrategy for CFileGenStrategy {
    fn gen_sdk_file(&self, _mod_names: &Vec<String>) -> Result<TokenStream> {
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

    fn quote_common_part(&self, _traits: &Vec<TraitDesc>) -> Result<TokenStream> {
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
        let orgin_arg_names = names.clone();
        let tys = struct_desc
            .fields
            .iter()
            .map(|field| Ident::new(&field.origin_ty, Span::call_site()))
            .collect::<Vec<Ident>>();
        Ok(quote! {
            #[repr(C)]
            #[derive(Serialize, Deserialize)]
            pub struct #struct_name {
                #(pub #names: #tys),*
            }

            impl From<#origin_struct_name> for #struct_name {
                fn from(origin: #origin_struct_name) -> Self {
                    #struct_name{#(#orgin_arg_names: origin.#arg_names),*}
                }
            }
        })
    }

    fn quote_method_sig(
        &self,
        trait_desc: &TraitDesc,
        _impl_desc: &ImpDesc,
        method: &MethodDesc,
        callbacks: &Vec<&TraitDesc>,
        _structs: &Vec<StructDesc>,
    ) -> Result<TokenStream> {
        let fun_name = Ident::new(
            &format!("{}_{}", &trait_desc.mod_name, &method.name),
            Span::call_site(),
        );

        let arg_names = method
            .args
            .iter()
            .filter(|arg| match arg.ty {
                AstType::Void => false,
                _ => true,
            })
            .map(|arg| Ident::new(&arg.name, Span::call_site()))
            .collect::<Vec<Ident>>();

        let arg_types = method
            .args
            .iter()
            .filter(|arg| match arg.ty {
                AstType::Void => false,
                _ => true,
            })
            .map(|arg| match arg.ty {
                AstType::Callback => {
                    let mut callback_trait = None;
                    for callback in callbacks.iter() {
                        if callback.name == arg.origin_ty {
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
                _ => self
                    .ty_to_tokens(&arg.ty, &arg.origin_ty, TypeDirection::Argument)
                    .unwrap(),
            })
            .collect::<Vec<TokenStream>>();

        let ret_ty_tokens = self.ty_to_tokens(
            &method.return_type,
            &method.origin_return_ty,
            TypeDirection::Return,
        )?;
        println!(
            "xxxxxx result ={:?} -> {:?}",
            &method.return_type, ret_ty_tokens
        );
        let sig_define = if arg_names.len() <= 0 {
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

        return Ok(sig_define);
    }

    fn quote_arg_convert(
        &self,
        trait_desc: &TraitDesc,
        arg: &ArgDesc,
        callbacks: &Vec<&TraitDesc>,
    ) -> Result<TokenStream> {
        let rust_arg_name = Ident::new(
            &format!("{}_{}", TMP_ARG_PREFIX, &arg.name),
            Span::call_site(),
        );
        let arg_name_ident = Ident::new(&arg.name, Span::call_site());

        Ok(match arg.ty {
            AstType::Byte | AstType::Int | AstType::Long | AstType::Float | AstType::Double => {
                let origin_type_ident = Ident::new(&arg.origin_ty, Span::call_site());
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
            AstType::Vec(base) => {
                if let AstBaseType::Byte = base {
                    if arg.origin_ty.clone().contains("i8") {
                        quote! {
                            let #rust_arg_name = unsafe { std::slice::from_raw_parts(#arg_name_ident.ptr as (*const i8), #arg_name_ident.len as usize).to_vec() };
                        }
                    } else {
                        quote! {
                            let #rust_arg_name = unsafe { std::slice::from_raw_parts(#arg_name_ident.ptr as (*const u8), #arg_name_ident.len as usize).to_vec() };
                        }
                    }
                } else {
                    let c_str_ident =
                        Ident::new(&format!("c_str_{}", &arg.name), Span::call_site());
                    let c_slice_ident =
                        Ident::new(&format!("c_str_{}", &arg.name), Span::call_site());
                    quote! {
                        let #c_str_ident: &CStr = unsafe{CStr::from_ptr(#arg_name_ident)};
                        let #c_slice_ident: &str = #c_str_ident.to_str().unwrap();
                        let #rust_arg_name = serde_json::from_str(&#c_slice_ident.to_owned()).unwrap();
                    }
                }
            }
            AstType::Callback => {
                println!("callback in argument found, {}", arg.origin_ty);
                self.callback_strategy
                    .arg_convert(arg, trait_desc, callbacks)
            }
            _ => {
                return Err(
                    GenerateError(format!("find unsupported type in arg, {:?}", &arg.ty)).into(),
                );
            }
        })
    }

    fn quote_return_convert(
        &self,
        ty: &AstType,
        ret_name: &str,
        origin_ty: &str,
    ) -> Result<TokenStream> {
        let ret_name_ident = Ident::new(ret_name, Span::call_site());

        Ok(match *ty {
            AstType::Void => quote!(),
            AstType::Boolean => quote! {
                if #ret_name_ident {1} else {0}
            },
            AstType::String => quote! {
                CString::new(#ret_name_ident).unwrap().into_raw()
            },
            AstType::Vec(ref base_ty) => match base_ty {
                AstBaseType::Struct => {
                    let struct_name = origin_ty.to_owned().replace("Vec<", "").replace(">", "");
                    let struct_ident =
                        Ident::new(&format!("Struct_{}", &struct_name), Span::call_site());
                    quote! {
                        let ret_value = ret_value.into_iter().map(|each| #struct_ident::from(each)).collect::<Vec<#struct_ident>>();
                        let json_ret = serde_json::to_string(&ret_value);
                        CString::new(json_ret.unwrap()).unwrap().into_raw()
                    }
                }
                _ => {
                    quote! {
                        let json_ret = serde_json::to_string(&ret_value);
                        CString::new(json_ret.unwrap()).unwrap().into_raw()
                    }
                }
            },
            AstType::Struct => {
                let struct_copy_name =
                    Ident::new(&format!("Struct_{}", origin_ty), Span::call_site());
                quote! {
                    let json_ret = serde_json::to_string(&#struct_copy_name::from(ret_value));
                    CString::new(json_ret.unwrap()).unwrap().into_raw()
                }
            }
            _ => {
                let ty_ident = self.ty_to_tokens(&ty, &origin_ty, TypeDirection::Return)?;
                quote! {
                    #ret_name_ident as #ty_ident
                }
            }
        })
    }

    fn ty_to_tokens(
        &self,
        ast_type: &AstType,
        origin_ty: &str,
        direction: TypeDirection,
    ) -> Result<TokenStream> {
        let mut tokens = TokenStream::new();
        match ast_type.clone() {
            AstType::Byte => tokens.append(Ident::new("i8", Span::call_site())),
            AstType::Int => tokens.append(Ident::new("i32", Span::call_site())),
            AstType::Long => tokens.append(Ident::new("i64", Span::call_site())),
            AstType::Float => tokens.append(Ident::new("f32", Span::call_site())),
            AstType::Double => tokens.append(Ident::new("f64", Span::call_site())),
            AstType::Boolean => tokens.append(Ident::new("i32", Span::call_site())),
            AstType::String => match direction {
                TypeDirection::Return => {
                    tokens.append(Punct::new('*', Spacing::Alone));
                    tokens.append(Ident::new("mut", Span::call_site()));
                    tokens.append(Ident::new("c_char", Span::call_site()));
                }
                TypeDirection::Argument => {
                    tokens.append(Punct::new('*', Spacing::Alone));
                    tokens.append(Ident::new("const", Span::call_site()));
                    tokens.append(Ident::new("c_char", Span::call_site()));
                }
            },
            AstType::Struct => {
                let struct_tokens = self.ty_to_tokens(&AstType::String, "String", direction)?;
                tokens = quote!(#struct_tokens)
            }
            AstType::Vec(base) => match base {
                AstBaseType::Byte => {
                    if let TypeDirection::Argument = direction {
                        tokens.append(Ident::new("CInt8Array", Span::call_site()));
                    } else {
                        let vec_tokens =
                            self.ty_to_tokens(&AstType::String, "String", direction)?;
                        tokens = quote!(#vec_tokens);
                    }
                }
                _ => {
                    let vec_tokens = self.ty_to_tokens(&AstType::String, "String", direction)?;
                    tokens = quote!(#vec_tokens);
                }
            },
            _ => (),
        };

        Ok(tokens)
    }
}
