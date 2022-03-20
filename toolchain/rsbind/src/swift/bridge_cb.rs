use proc_macro2::{Ident, Span, TokenStream};

use crate::ast::contract::desc::*;
use crate::ast::types::*;
use crate::bridge::file::*;
use crate::errors::*;
use crate::swift::mapping::RustMapping;

pub struct CCallbackStrategy {}

impl CallbackGenStrategy for CCallbackStrategy {
    fn arg_convert(
        &self,
        arg: &ArgDesc,
        _trait_desc: &TraitDesc,
        callbacks: &[&TraitDesc],
    ) -> Result<TokenStream> {
        let arg_name_ident = Ident::new(&arg.name, Span::call_site());

        let struct_name = &format!("{}_struct", arg.name);
        let struct_ident = Ident::new(struct_name, Span::call_site());

        // find the callback type for this argument.
        let mut callback_desc = None;
        for desc in callbacks {
            if desc.name == arg.ty.origin() {
                callback_desc = Some(desc);
            }
        }

        let mut method_names = Vec::new();
        let mut callback_methods = TokenStream::new();
        let mut callback_struct = TokenStream::new();
        if let Some(callback_desc) = callback_desc {
            for method in callback_desc.methods.iter() {
                println!(
                    "quote method {} in callback {}",
                    method.name, callback_desc.name
                );

                let mut strs_to_release: Vec<Ident> = vec![];
                // arguments converting in callback
                let mut args_convert = TokenStream::new();
                for cb_arg in method.args.iter() {
                    let args_convert_each = crate::swift::bridge_r2c::arg_convert(cb_arg)
                        .expect("Argument Convert failed.");

                    match cb_arg.ty.clone() {
                        AstType::String
                        | AstType::Vec(AstBaseType::Float(_))
                        | AstType::Vec(AstBaseType::Double(_))
                        | AstType::Vec(AstBaseType::Boolean)
                        | AstType::Vec(AstBaseType::String)
                        | AstType::Vec(AstBaseType::Struct(_))
                        | AstType::Struct(_) => {
                            let cb_arg_name =
                                Ident::new(&format!("c_{}", cb_arg.name), Span::call_site());
                            strs_to_release.push(cb_arg_name.clone());
                        }
                        _ => {}
                    }

                    args_convert = quote! {
                        #args_convert
                        #args_convert_each
                    }
                }

                let arg_names = &method
                    .args
                    .iter()
                    .filter(|arg| !matches!(arg.ty, AstType::Void))
                    .map(|arg| Ident::new(&arg.name, Span::call_site()))
                    .collect::<Vec<Ident>>();

                let convert_arg_names = &method
                    .args
                    .iter()
                    .filter(|arg| !matches!(arg.ty, AstType::Void))
                    .map(|arg| Ident::new(&format!("c_{}", &arg.name), Span::call_site()))
                    .collect::<Vec<Ident>>();

                let arg_types = &method
                    .args
                    .iter()
                    .filter(|arg| !matches!(arg.ty, AstType::Void))
                    .map(|arg| match arg.ty.clone() {
                        AstType::Vec(vec_inner_name) => {
                            let vec_innder_ident =
                                Ident::new(&vec_inner_name.origin(), Span::call_site());
                            quote!(Vec<#vec_innder_ident>)
                        }
                        _ => {
                            let ident = Ident::new(&arg.ty.origin(), Span::call_site());
                            quote!(#ident)
                        }
                    })
                    .collect::<Vec<TokenStream>>();

                let ret_ty_tokens = match method.return_type {
                    AstType::Void => quote!(()),
                    AstType::Vec(ref base) => {
                        let ident = Ident::new(&base.origin(), Span::call_site());
                        quote!(Vec<#ident>)
                    }
                    _ => {
                        let ident = Ident::new(&method.return_type.origin(), Span::call_site());
                        quote!(#ident)
                    }
                };

                let return_convert = crate::swift::bridge_r2c::return_convert(&method.return_type)
                    .expect("Return convert error!");

                // return var ident name
                let return_var_name = match method.return_type {
                    AstType::Void => quote!(),
                    _ => quote!(s_result),
                };

                // methods calls on impl
                let method_name = Ident::new(&method.name, Span::call_site());
                let fn_method_name = Ident::new(&format!("fn_{}", method.name), Span::call_site());
                let method_result = quote! {
                    fn #method_name(&self, #(#arg_names: #arg_types),*) -> #ret_ty_tokens {
                        #args_convert
                        let #fn_method_name = self.#method_name;
                        let result = #fn_method_name(self.index, #(#convert_arg_names),*);
                        #(unsafe {CString::from_raw(#strs_to_release)};)*
                        #return_convert
                        #return_var_name
                    }
                };

                callback_methods = quote! {
                    #callback_methods
                    #method_result
                };

                method_names.push(method_name);
            }

            callback_struct = self
                .quote_callback_struct(callback_desc, struct_name)
                .unwrap();
        }

        // xxxx : arg.xxxx
        // assign values from arg to struct
        let converted_callback_name = Ident::new(&format!("r_{}", &arg.name), Span::call_site());
        let callback_ty = Ident::new(&arg.ty.origin(), Span::call_site());
        let callback_name = Ident::new(&arg.name, Span::call_site());
        let mut method_assign_tokens = TokenStream::new();
        for method_name in method_names.iter() {
            method_assign_tokens = quote! {
                #method_assign_tokens
                #method_name: #callback_name.#method_name,
            }
        }

        // total converting codes.
        Ok(quote! {
            #callback_struct

            impl #callback_ty for #struct_ident {
                #callback_methods
            }

            impl Drop for  #struct_ident {
                fn drop(&mut self) {
                    let free_callback = self.free_callback;
                    free_callback(self.index)
                }
            }

            let #converted_callback_name = Box::new(#struct_ident {
                #method_assign_tokens
                free_callback: #arg_name_ident.free_callback,
                free_ptr: #arg_name_ident.free_ptr,
                index: #arg_name_ident.index,
            });

        })
    }

    fn return_convert(
        &self,
        ret_ty: &AstType,
        trait_desc: &TraitDesc,
        callbacks: &[&TraitDesc],
    ) -> Result<TokenStream> {
        let callback_model_str = &format!("{}_{}_Model", &trait_desc.mod_name, &ret_ty.origin());
        let callback_model_ident = Ident::new(callback_model_str, Span::call_site());
        let callback_ident = Ident::new(&ret_ty.origin(), Span::call_site());

        // find the callback type for this argument.
        let mut callback_desc = None;
        for desc in callbacks {
            if desc.name == ret_ty.origin() {
                callback_desc = Some(desc);
            }
        }

        let mut method_names = vec![];
        let mut ret_method_names = vec![];

        let mut method_result = TokenStream::new();
        if let Some(callback_desc) = callback_desc {
            method_names = callback_desc
                .methods
                .iter()
                .map(|method| Ident::new(&method.name, Span::call_site()))
                .collect::<Vec<Ident>>();
            ret_method_names = callback_desc
                .methods
                .iter()
                .map(|method| Ident::new(&format!("ret_{}", &method.name), Span::call_site()))
                .collect::<Vec<Ident>>();

            for method in callback_desc.methods.iter() {
                let method_name = Ident::new(&method.name, Span::call_site());
                let arg_names = &method
                    .args
                    .iter()
                    .filter(|arg| !matches!(arg.ty, AstType::Void))
                    .map(|arg| Ident::new(&arg.name, Span::call_site()))
                    .collect::<Vec<Ident>>();
                let r_arg_names = &method
                    .args
                    .iter()
                    .filter(|arg| !matches!(arg.ty, AstType::Void))
                    .map(|arg| Ident::new(&format!("r_{}", &arg.name), Span::call_site()))
                    .collect::<Vec<Ident>>();
                let arg_types = &method
                    .args
                    .iter()
                    .filter(|arg| !matches!(arg.ty, AstType::Void))
                    .map(|arg| RustMapping::map_c2r_transfer_type(&arg.ty))
                    .collect::<Vec<TokenStream>>();
                let ret_ty_tokens = RustMapping::map_c2r_transfer_type(&method.return_type);
                let mut args_convert = TokenStream::new();
                for arg in method.args.iter() {
                    let each_convert = crate::swift::bridge_c2r::quote_arg_convert(arg)?;
                    args_convert = quote! {
                        #args_convert
                        #each_convert
                    }
                }

                let return_convert = crate::swift::bridge_c2r::quote_return_convert(
                    &method.return_type,
                    "r_result",
                )?;

                let ret_method_name =
                    Ident::new(&format!("ret_{}", &method.name), Span::call_site());
                method_result = quote! {
                    #method_result

                    pub extern "C" fn #ret_method_name(index: i64, #(#arg_names: #arg_types),*) -> #ret_ty_tokens {
                        #args_convert

                        let callback_hashmap = &*CALLBACK_HASHMAP.read().unwrap();
                        let ret_callback = callback_hashmap.get(&index);
                        match ret_callback {
                            Some(ret_callback) => {
                                if let CallbackEnum::#callback_ident(ret_callback) = ret_callback {
                                    let mut r_result = ret_callback.#method_name(#(#r_arg_names),*);
                                    #return_convert
                                } else {
                                    panic!("Callback doesn't match for index: {}", index);
                                }
                            }
                            None => {
                                panic!("No callback found for index: {}", index);
                            }
                        }
                    }
                };
            }

            let free_fn_ident = Ident::new(
                &format!("{}_free_rust", &trait_desc.crate_name),
                Span::call_site(),
            );
            method_result = quote! {
                #method_result

                pub extern "C" fn ret_free_callback(index: i64) {
                    (*CALLBACK_HASHMAP.write().unwrap()).remove(&index);
                }

                pub extern "C" fn ret_free_ptr(buffer: *mut i8, size: i32) {
                    #free_fn_ident(buffer, size as u32)
                }
            }
        }

        Ok(quote! {
            let callback_index = { *CALLBACK_INDEX.read().unwrap() };
            if callback_index == i64::MAX {
                *CALLBACK_INDEX.write().unwrap() = 0;
            } else {
                *CALLBACK_INDEX.write().unwrap() = callback_index + 1;
            }
            (*CALLBACK_HASHMAP.write().unwrap()).insert(callback_index, CallbackEnum::#callback_ident(ret_value));

            impl #callback_model_ident {
                #method_result
            }

            #callback_model_ident {
                #(#method_names: #callback_model_ident::#ret_method_names),*,
                free_callback: #callback_model_ident::ret_free_callback,
                free_ptr: #callback_model_ident::ret_free_ptr,
                index: callback_index
            }
        })
    }
}

impl CCallbackStrategy {
    pub(crate) fn quote_callback_struct(
        &self,
        trait_desc: &TraitDesc,
        name: &str,
    ) -> Result<TokenStream> {
        let callback_ident = Ident::new(name, Span::call_site());

        let callback_struct_sig = quote! {
            pub struct #callback_ident
        };

        let mut callback_methods = TokenStream::new();
        for method in trait_desc.methods.iter() {
            let callback_method_ident = Ident::new(&method.name, Span::call_site());
            let ret_ty_tokens = RustMapping::map_c2r_transfer_type(&method.return_type);
            let arg_types = method
                .args
                .iter()
                .filter(|arg| !matches!(arg.ty, AstType::Void))
                .map(|arg| RustMapping::map_c2r_transfer_type(&arg.ty))
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
}
