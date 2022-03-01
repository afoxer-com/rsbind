use ast::contract::desc::*;
use ast::types::*;
use bridge::file::*;
use errors::*;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::TokenStreamExt;

pub struct JavaCallbackStrategy {
    pub(crate) java_namespace: String,
}

impl CallbackGenStrategy for JavaCallbackStrategy {
    fn arg_convert(
        &self,
        arg: &ArgDesc,
        trait_desc: &TraitDesc,
        callbacks: &Vec<&TraitDesc>,
    ) -> TokenStream {
        println!(
            "[bridge] 🔆  begin quote callback argument in method convert => {}.{}",
            &arg.name, &arg.origin_ty
        );
        let rust_arg_name = Ident::new(
            &format!("{}_{}", TMP_ARG_PREFIX, &arg.name),
            Span::call_site(),
        );
        let class_name = format!("{}.{}", &self.java_namespace, &trait_desc.name).replace(".", "/");
        let struct_name = &format!("{}_struct", arg.name);
        let struct_ident = Ident::new(struct_name, Span::call_site());
        let arg_name_ident = Ident::new(&arg.name, Span::call_site());

        // find the callback type for this argument.
        let mut callback_desc = None;
        for desc in callbacks {
            if desc.name == arg.origin_ty {
                callback_desc = Some(desc);
            }
        }

        let callback_struct = quote! {
            #[derive(Serialize, Deserialize)]
            struct #struct_ident {
                index: i64,
            }
        };

        let _callback_methods = TokenStream::new();
        let callback_desc = callback_desc.unwrap();
        let mut methods_result = TokenStream::new();
        for method in callback_desc.methods.iter() {
            println!(
                "[bridge] 🔆  begin quote callback method => {}.{}",
                &callback_desc.name, &method.name
            );
            // arguments converting in callback
            let mut args_convert = TokenStream::new();
            let mut method_java_sig = "(J".to_owned();
            let mut cb_arg_array = quote!(JValue::Long(self.index),);
            for cb_arg in method.args.iter() {
                let cb_arg_name = Ident::new(&format!("j_{}", cb_arg.name), Span::call_site());
                let cb_origin_arg_name = Ident::new(&cb_arg.name, Span::call_site());
                method_java_sig = format!("{}{}", &method_java_sig, cb_arg.ty.to_java_sig());

                let args_convert_each = match cb_arg.ty {
                    AstType::Boolean => {
                        quote! {
                            let #cb_arg_name = if #cb_origin_arg_name {1} else {0};
                        }
                    }
                    AstType::String => {
                        quote! {
                            let #cb_arg_name = env.new_string(#cb_origin_arg_name).unwrap().into();
                        }
                    }
                    AstType::Vec(ref base_ty) => {
                        let cb_tmp_arg_name =
                            Ident::new(&format!("j_tmp_{}", cb_arg.name), Span::call_site());
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
                                    &format!("j_tmp_vec_{}", cb_arg.name),
                                    Span::call_site(),
                                );
                                quote! {
                                    let #cb_tmp_vec_arg_name = #cb_origin_arg_name.into_iter().map(|each| #struct_ident::from(each)).collect::<Vec<#struct_ident>>();
                                    let #cb_tmp_arg_name = serde_json::to_string(&#cb_tmp_vec_arg_name);
                                    let #cb_arg_name = env.new_string(#cb_tmp_arg_name.unwrap()).unwrap().into();
                                }
                            }
                            AstBaseType::Byte => {
                                if cb_arg.origin_ty.clone().contains("i8") {
                                    let tmp_arg_name = Ident::new(
                                        &format!("tmp_{}", &cb_arg.name),
                                        Span::call_site(),
                                    );
                                    let tmp_converted_arg_name = Ident::new(
                                        &format!("tmp_converted_{}", &cb_arg.name),
                                        Span::call_site(),
                                    );
                                    vec![1u8].as_slice();
                                    quote! {
                                        let #tmp_arg_name = #cb_origin_arg_name.as_slice();
                                        let #tmp_converted_arg_name = unsafe { std::mem::transmute::<&[i8], &[u8]>(#tmp_arg_name) };
                                        let #cb_arg_name = env.byte_array_from_slice(#tmp_converted_arg_name).unwrap();
                                    }
                                } else {
                                    quote! {
                                        let #cb_arg_name = env.byte_array_from_slice(#cb_origin_arg_name.as_slice()).unwrap();
                                    }
                                }
                            }
                            _ => {
                                quote! {
                                    let #cb_tmp_arg_name = serde_json::to_string(&#cb_origin_arg_name);
                                    let #cb_arg_name = env.new_string(#cb_tmp_arg_name.unwrap()).unwrap().into();
                                }
                            }
                        }
                    }
                    AstType::Struct => {
                        let struct_copy_name =
                            Ident::new(&format!("Struct_{}", &cb_arg.origin_ty), Span::call_site());
                        let cb_tmp_arg_name =
                            Ident::new(&format!("r_tmp_{}", cb_arg.name), Span::call_site());
                        quote! {
                            let #cb_tmp_arg_name = serde_json::to_string(&#struct_copy_name::from(#cb_origin_arg_name));
                            let #cb_arg_name = env.new_string(#cb_tmp_arg_name.unwrap()).unwrap().into();
                        }
                    }
                    _ => {
                        let arg_ty_ident = self
                            .ty_to_tokens(&cb_arg.ty, &cb_arg.origin_ty, TypeDirection::Argument)
                            .unwrap();
                        quote! {
                            let #cb_arg_name = #cb_origin_arg_name as #arg_ty_ident;
                        }
                    }
                };

                let cb_arg_array_each = match cb_arg.ty {
                    AstType::Byte => quote! {
                        JValue::Byte(#cb_arg_name),
                    },

                    AstType::Boolean | AstType::Int => quote! {
                        JValue::Int(#cb_arg_name),
                    },

                    AstType::Long => quote! {
                        JValue::Long(#cb_arg_name),
                    },

                    AstType::String => quote! {
                        JValue::Object(#cb_arg_name),
                    },

                    AstType::Float => quote! {
                        JValue::Float(#cb_arg_name),
                    },

                    AstType::Double => quote! {
                        JValue::Double(#cb_arg_name),
                    },

                    AstType::Vec(base) => {
                        if base == AstBaseType::Byte {
                            quote! {
                                JValue::Object(JObject::from(#cb_arg_name)),
                            }
                        } else {
                            quote! {
                                JValue::Object(#cb_arg_name),
                            }
                        }
                    }

                    _ => quote! {
                        JValue::Object(#cb_arg_name),
                    },
                };

                args_convert = quote! {
                    #args_convert
                    #args_convert_each
                };

                cb_arg_array = quote! {#cb_arg_array #cb_arg_array_each};
            }
            method_java_sig = format!("{}){}", &method_java_sig, method.return_type.to_java_sig());
            let method_java_sig_literal = Literal::string(&method_java_sig);

            let arg_names = &method
                .args
                .iter()
                .map(|arg| Ident::new(&arg.name, Span::call_site()))
                .collect::<Vec<Ident>>();
            let _convert_arg_names = &method
                .args
                .iter()
                .map(|arg| Ident::new(&format!("c_{}", &arg.name), Span::call_site()))
                .collect::<Vec<Ident>>();
            let arg_types = &method
                .args
                .iter()
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

            println!(
                "[bridge] 🔆  begin quote callback return type ident => {}.{}",
                &callback_desc.name, &method.name
            );
            let ret_ty_tokens = match method.return_type {
                AstType::Void => quote!(()),
                _ => {
                    let ident = Ident::new(&method.origin_return_ty, Span::call_site());
                    quote!(#ident)
                }
            };
            println!(
                "[bridge] ✅  end quote callback return type ident => {}.{}",
                &callback_desc.name, &method.name
            );

            let return_convert = match method.return_type {
                AstType::Void => quote!(),
                AstType::Boolean => quote! {
                    let mut r_result = None;
                    match result.unwrap() {
                        JValue::Int(value) => r_result = Some(value),
                        _ => assert!(false)
                    }

                    let s_result = if r_result.unwrap() > 0 {true} else {false};
                },

                AstType::Byte => {
                    let origin_return_ty_ident =
                        Ident::new(&method.origin_return_ty, Span::call_site());
                    quote! {
                        let mut r_result = None;
                        match result.unwrap() {
                            JValue::Byte(value) => r_result = Some(value),
                            _ => assert!(false)
                        }

                        let s_result = r_result.unwrap() as #origin_return_ty_ident;
                    }
                }

                AstType::Int => {
                    let origin_return_ty_ident =
                        Ident::new(&method.origin_return_ty, Span::call_site());
                    quote! {
                        let mut r_result = None;
                        match result.unwrap() {
                            JValue::Int(value) => r_result = Some(value),
                            _ => assert!(false)
                        }

                        let s_result = r_result.unwrap() as #origin_return_ty_ident;
                    }
                }
                AstType::Long => {
                    let origin_return_ty_ident =
                        Ident::new(&method.origin_return_ty, Span::call_site());
                    quote! {
                        let mut r_result = None;
                        match result.unwrap() {
                            JValue::Long(value) => r_result = Some(value),
                            _ => assert!(false)
                        }

                        let s_result = r_result.unwrap() as #origin_return_ty_ident;
                    }
                }
                AstType::Float => {
                    let origin_return_ty_ident =
                        Ident::new(&method.origin_return_ty, Span::call_site());
                    quote! {
                        let mut r_result = None;
                        match result.unwrap() {
                            JValue::Float(value) => r_result = Some(value),
                            _ => assert!(false)
                        }

                        let s_result = r_result.unwrap() as #origin_return_ty_ident;
                    }
                }
                AstType::Double => {
                    let _origin_return_ty_ident =
                        Ident::new(&method.origin_return_ty, Span::call_site());
                    quote! {
                        let mut r_result = None;
                        match result.unwrap() {
                            JValue::Double(value) => r_result = Some(value),
                            _ => assert!(false)
                        }
                        let s_result = env.get_string(r_result.unwrap()).expect("Couldn't get java string!").into();
                    }
                }
                AstType::String => {
                    let origin_return_ty_ident =
                        Ident::new(&method.origin_return_ty, Span::call_site());
                    quote! {
                        let mut r_result = None;
                        match result.unwrap() {
                            JValue::Object(value) => r_result = Some(value),
                            _ => assert!(false)
                        }

                        let s_result = r_result.unwrap() as #origin_return_ty_ident;
                    }
                }
                _ => {
                    quote! {
                        let s_result = result as #ret_ty_tokens;
                    }
                }
            };

            let return_result_ident = match method.return_type {
                AstType::Void => quote!(),
                _ => quote!(s_result),
            };

            // methods calls on impl
            let method_name = Ident::new(&method.name, Span::call_site());
            let java_method_name = format!("invoke_{}_{}", &callback_desc.name, &method.name);

            methods_result = quote! {
                #methods_result

                fn #method_name(&self, #(#arg_names: #arg_types),*) -> #ret_ty_tokens {
                    let env = (*JVM.read().unwrap()).unwrap().attach_current_thread().unwrap();

                    #args_convert

                    let result = env.call_static_method(
                        #class_name,
                        #java_method_name,
                        #method_java_sig_literal,
                         &[
                            #cb_arg_array
                        ],
                    );

                    #return_convert
                    #return_result_ident
                }
            };

            println!(
                "[bridge] ✅ end quote callback method => {}.{}",
                &callback_desc.name, &method.name
            );
        }

        let callback_ident = Ident::new(&callback_desc.name, Span::call_site());
        let _callback_str_ident = Ident::new(&format!("r_{}_str", &arg.name), Span::call_site());
        let result = quote! {
            #callback_struct

            impl #callback_ident for #struct_ident {
                #methods_result
            }

            impl Drop for #struct_ident {
                fn drop(&mut self) {
                    let env = (*JVM.read().unwrap()).unwrap().attach_current_thread().unwrap();
                    let _method_result = env.call_static_method(
                        #class_name,
                        "free_callback",
                        "(J)V",
                        &[JValue::Long(self.index as jlong)],
                    );
                }
            }

            let #rust_arg_name: Box<#struct_ident> = Box::new(#struct_ident{index: #arg_name_ident});
        };

        println!(
            "[bridge] ✅  end quote callback argument in method convert => {}.{}",
            &arg.name, &arg.origin_ty
        );
        result
    }
}

impl JavaCallbackStrategy {
    fn ty_to_tokens(
        &self,
        ast_type: &AstType,
        origin_ty: &str,
        direction: TypeDirection,
    ) -> Result<TokenStream> {
        let mut tokens = TokenStream::new();
        match *ast_type {
            AstType::Byte => tokens.append(Ident::new("i8", Span::call_site())),
            AstType::Int => tokens.append(Ident::new("i32", Span::call_site())),
            AstType::Long => tokens.append(Ident::new("i64", Span::call_site())),
            AstType::Float => tokens.append(Ident::new("f32", Span::call_site())),
            AstType::Double => tokens.append(Ident::new("f64", Span::call_site())),
            AstType::Boolean => tokens.append(Ident::new("u8", Span::call_site())),
            AstType::String => match direction {
                TypeDirection::Argument => tokens.append(Ident::new("JString", Span::call_site())),
                TypeDirection::Return => tokens.append(Ident::new("jstring", Span::call_site())),
            },
            AstType::Vec(_base) => match direction {
                TypeDirection::Argument => tokens.append(Ident::new("JString", Span::call_site())),
                TypeDirection::Return => tokens.append(Ident::new("jstring", Span::call_site())),
            },
            AstType::Struct => match direction {
                TypeDirection::Argument => tokens.append(Ident::new("JString", Span::call_site())),
                TypeDirection::Return => tokens.append(Ident::new("jstring", Span::call_site())),
            },
            AstType::Callback => tokens.append(Ident::new("i64", Span::call_site())),
            _ => (),
        };

        Ok(tokens)
    }
}
