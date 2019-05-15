use bridge::file::*;
use ast::contract::desc::*;
use ast::types::*;
use errors::*;
use proc_macro2::{Ident, Punct, Spacing, Span, TokenStream};
use quote::TokenStreamExt;

pub struct CCallbackStrategy {}

impl CallbackGenStrategy for CCallbackStrategy {
    fn arg_convert(
        &self,
        arg: &ArgDesc,
        _trait_desc: &TraitDesc,
        callbacks: &Vec<&TraitDesc>,
    ) -> TokenStream {
        let arg_name_ident = Ident::new(&arg.name, Span::call_site());

        let struct_name = &format!("{}_struct", arg.name);
        let struct_ident = Ident::new(struct_name, Span::call_site());

        // find the callback type for this argument.
        let mut callback_desc = None;
        for desc in callbacks {
            if desc.name == arg.origin_ty {
                callback_desc = Some(desc);
            }
        }

        let mut method_names = Vec::new();
        let mut callback_methods = TokenStream::new();
        let mut callback_struct = TokenStream::new();
        if callback_desc.is_some() {
            let callback_desc = callback_desc.unwrap();
            for method in callback_desc.methods.iter() {
                println!(
                    "quote method {} in callback {}",
                    method.name, callback_desc.name
                );

                let mut strs_to_release: Vec<Ident> = vec![];
                // arguments converting in callback
                let mut args_convert = TokenStream::new();
                for cb_arg in method.args.iter() {
                    let cb_arg_name = Ident::new(&format!("c_{}", cb_arg.name), Span::call_site());
                    let cb_origin_arg_name = Ident::new(&cb_arg.name, Span::call_site());
                    let args_convert_each = match cb_arg.ty {
                        AstType::Boolean => {
                            quote! {
                                let #cb_arg_name = if #cb_origin_arg_name {1} else {0};
                            }
                        }
                        AstType::String => {
                            strs_to_release.push(cb_arg_name.clone());
                            quote! {
                                let #cb_arg_name = CString::new(#cb_origin_arg_name).unwrap().into_raw();
                            }
                        }
                        AstType::Vec(ref base_ty) => {
                            strs_to_release.push(cb_arg_name.clone());
                            let cb_tmp_arg_name =
                                Ident::new(&format!("c_tmp_{}", cb_arg.name), Span::call_site());
                            match base_ty {
                                AstBaseType::Struct => {
                                    let struct_name = cb_arg
                                        .origin_ty
                                        .to_owned()
                                        .replace("Vec<", "")
                                        .replace(">", "");
                                    let struct_ident = Ident::new(
                                        &format!("Struct_{}", &struct_name),
                                        Span::call_site(),
                                    );
                                    let cb_tmp_vec_arg_name = Ident::new(
                                        &format!("c_tmp_vec_{}", cb_arg.name),
                                        Span::call_site(),
                                    );
                                    quote! {
                                        let #cb_tmp_vec_arg_name = #cb_origin_arg_name.into_iter().map(|each| #struct_ident::from(each)).collect::<Vec<#struct_ident>>();
                                        let #cb_tmp_arg_name = serde_json::to_string(&#cb_tmp_vec_arg_name);
                                        let #cb_arg_name = CString::new(#cb_tmp_arg_name.unwrap()).unwrap().into_raw();
                                    }
                                }
                                _ => {
                                    quote! {
                                        let #cb_tmp_arg_name = serde_json::to_string(&#cb_origin_arg_name);
                                        let #cb_arg_name = CString::new(#cb_tmp_arg_name.unwrap()).unwrap().into_raw();
                                    }
                                }
                            }
                        }
                        AstType::Struct => {
                            strs_to_release.push(cb_arg_name.clone());
                            let struct_copy_name = Ident::new(
                                &format!("Struct_{}", &cb_arg.origin_ty),
                                Span::call_site(),
                            );
                            let cb_tmp_arg_name =
                                Ident::new(&format!("c_tmp_{}", cb_arg.name), Span::call_site());
                            quote! {
                                let #cb_tmp_arg_name = serde_json::to_string(&#struct_copy_name::from(#cb_origin_arg_name));
                                let #cb_arg_name = CString::new(#cb_tmp_arg_name.unwrap()).unwrap().into_raw();
                            }
                        }
                        _ => {
                            let arg_ty_ident = self.ty_to_tokens(&cb_arg.ty).unwrap();
                            quote! {
                                let #cb_arg_name = #cb_origin_arg_name as #arg_ty_ident;
                            }
                        }
                    };
                    args_convert = quote! {
                        #args_convert
                        #args_convert_each
                    }
                }

                let arg_names = &method
                    .args
                    .iter()
                    .filter(|arg| match arg.ty {
                        AstType::Void => false,
                        _ => true,
                    })
                    .map(|arg| Ident::new(&arg.name, Span::call_site()))
                    .collect::<Vec<Ident>>();

                let convert_arg_names = &method
                    .args
                    .iter()
                    .filter(|arg| match arg.ty {
                        AstType::Void => false,
                        _ => true,
                    })
                    .map(|arg| Ident::new(&format!("c_{}", &arg.name), Span::call_site()))
                    .collect::<Vec<Ident>>();

                let arg_types = &method
                    .args
                    .iter()
                    .filter(|arg| match arg.ty {
                        AstType::Void => false,
                        _ => true,
                    })
                    .map(|arg| match arg.ty {
                        AstType::Vec(_base_ty) => {
                            let vec_inner_name =
                                arg.origin_ty.clone().replace("Vec<", "").replace(">", "");
                            let vec_innder_ident = Ident::new(&vec_inner_name, Span::call_site());
                            quote!(Vec<#vec_innder_ident>)
                        }
                        _ => {
                            let ident = Ident::new(&arg.origin_ty, Span::call_site());
                            quote!(#ident)
                        }
                    })
                    .collect::<Vec<TokenStream>>();

                let ret_ty_tokens = match method.return_type {
                    AstType::Void => quote!(()),
                    _ => {
                        let ident = Ident::new(&method.origin_return_ty, Span::call_site());
                        quote!(#ident)
                    }
                };

                let return_convert = match method.return_type {
                    AstType::Void => quote!(),
                    AstType::Boolean => quote! {
                        let s_result = if result > 0 {true} else {false};
                    },
                    AstType::String => quote! {
                        let s_result_c_str: &CStr = unsafe { CStr::from_ptr(result) };
                        let s_result_str: &str = s_result_str.to_str().unwrap();
                        let s_result: String = s_result_str.to_owned();
                    },
                    _ => quote! {
                        let s_result = result as #ret_ty_tokens;
                    },
                };

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
        let callback_ty = Ident::new(&arg.origin_ty, Span::call_site());
        let callback_name = Ident::new(&arg.name, Span::call_site());
        let mut method_assign_tokens = TokenStream::new();
        for method_name in method_names.iter() {
            method_assign_tokens = quote! {
                #method_assign_tokens
                #method_name: #callback_name.#method_name,
            }
        }

        // total converting codes.
        quote! {
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
                index: #arg_name_ident.index,
            });

        }
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
            let ret_ty_tokens = match method.return_type {
                AstType::Void => quote!(()),
                _ => self.ty_to_tokens(&method.return_type).unwrap(),
            };

            let arg_types = method
                .args
                .iter()
                .filter(|arg| match arg.ty {
                    AstType::Void => false,
                    _ => true,
                })
                .map(|arg| self.ty_to_tokens(&arg.ty).unwrap())
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
                pub index: i64,

            }
        };

        return Ok(callback_struct);
    }

    fn ty_to_tokens(&self, ast_type: &AstType) -> Result<TokenStream> {
        let mut tokens = TokenStream::new();
        match *ast_type {
            AstType::Byte => tokens.append(Ident::new("i8", Span::call_site())),
            AstType::Int => tokens.append(Ident::new("i32", Span::call_site())),
            AstType::Long => tokens.append(Ident::new("i64", Span::call_site())),
            AstType::Float => tokens.append(Ident::new("f32", Span::call_site())),
            AstType::Double => tokens.append(Ident::new("f64", Span::call_site())),
            AstType::Boolean => tokens.append(Ident::new("i32", Span::call_site())),
            AstType::String => {
                tokens.append(Punct::new('*', Spacing::Alone));
                tokens.append(Ident::new("const", Span::call_site()));
                tokens.append(Ident::new("c_char", Span::call_site()));
            }
            AstType::Struct => {
                let struct_tokens = self.ty_to_tokens(&AstType::String).unwrap();
                tokens = quote!(#struct_tokens)
            }
            AstType::Vec(_) => {
                let vec_tokens = self.ty_to_tokens(&AstType::String).unwrap();
                tokens = quote!(#vec_tokens)
            }
            _ => (),
        };

        Ok(tokens)
    }
}
