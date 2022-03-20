use heck::ToUpperCamelCase;
use proc_macro2::{Ident, Literal, Span, TokenStream};

use crate::ast::contract::desc::*;
use crate::ast::types::*;
use crate::bridge::file::*;
use crate::errors::*;

pub struct JavaCallbackStrategy {
    pub(crate) java_namespace: String,
}

impl CallbackGenStrategy for JavaCallbackStrategy {
    fn arg_convert(
        &self,
        arg: &ArgDesc,
        _trait_desc: &TraitDesc,
        callbacks: &[&TraitDesc],
    ) -> Result<TokenStream> {
        println!(
            "[bridge] 🔆  begin quote callback argument in method convert => {}.{}",
            &arg.name,
            &arg.ty.origin()
        );
        let rust_arg_name = Ident::new(
            &format!("{}_{}", TMP_ARG_PREFIX, &arg.name),
            Span::call_site(),
        );
        let struct_name = &format!("{}_struct", arg.name);
        let struct_ident = Ident::new(struct_name, Span::call_site());
        let arg_name_ident = Ident::new(&arg.name, Span::call_site());

        // find the callback type for this argument.
        let mut callback_desc = None;
        for desc in callbacks {
            if desc.name == arg.ty.origin() {
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
        let class_name =
            format!("{}.Internal{}", &self.java_namespace, &callback_desc.name).replace('.', "/");

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
                method_java_sig = format!("{}{}", &method_java_sig, cb_arg.ty.to_java_sig());

                let args_convert_each = crate::java::bridge_r2j::arg_convert(cb_arg)?;

                args_convert = quote! {
                    #args_convert
                    #args_convert_each
                };

                let cb_arg_array_each = match cb_arg.ty.clone() {
                    AstType::Byte(_) => quote! {
                        JValue::Byte(#cb_arg_name),
                    },

                    AstType::Short(_) => quote! {
                        JValue::Short(#cb_arg_name),
                    },

                    AstType::Boolean | AstType::Int(_) => quote! {
                        JValue::Int(#cb_arg_name),
                    },

                    AstType::Long(_) => quote! {
                        JValue::Long(#cb_arg_name),
                    },

                    AstType::String => quote! {
                        JValue::Object(#cb_arg_name),
                    },

                    AstType::Float(_) => quote! {
                        JValue::Float(#cb_arg_name),
                    },

                    AstType::Double(_) => quote! {
                        JValue::Double(#cb_arg_name),
                    },

                    AstType::Vec(AstBaseType::Byte(_)) => {
                        quote! {
                            JValue::Object(JObject::from(#cb_arg_name)),
                        }
                    }
                    AstType::Vec(_) => {
                        quote! {
                            JValue::Object(#cb_arg_name),
                        }
                    }
                    _ => quote! {
                        JValue::Object(#cb_arg_name),
                    },
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

            println!(
                "[bridge] 🔆  begin quote callback return type ident => {}.{}",
                &callback_desc.name, &method.name
            );
            let ret_ty_tokens = match method.return_type {
                AstType::Void => quote!(()),
                AstType::Vec(ref base) => {
                    let origin_ident = Ident::new(&base.origin(), Span::call_site());
                    quote!(Vec<#origin_ident>)
                }
                _ => {
                    let ident = Ident::new(&method.return_type.origin(), Span::call_site());
                    quote!(#ident)
                }
            };
            println!(
                "[bridge] ✅  end quote callback return type ident => {}.{}",
                &callback_desc.name, &method.name
            );

            let return_convert = crate::java::bridge_r2j::return_convert(method)?;

            let return_result_ident = match method.return_type {
                AstType::Void => quote!(),
                _ => quote!(s_result),
            };

            // methods calls on impl
            let method_name = Ident::new(&method.name, Span::call_site());
            let java_method_name = format!("r2j{}", &method.name.to_upper_camel_case());

            methods_result = quote! {
                #methods_result

                fn #method_name(&self, #(#arg_names: #arg_types),*) -> #ret_ty_tokens {
                    let ptr_jvm = JVM.read().unwrap();
                    let jvm = (*ptr_jvm).as_ref().unwrap();
                    let env = jvm.attach_current_thread_permanently().unwrap();

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
                    let ptr_jvm = JVM.read().unwrap();
                    let jvm = (*ptr_jvm).as_ref().unwrap();
                    let env = jvm.attach_current_thread_permanently().unwrap();

                    let _method_result = env.call_static_method(
                        #class_name,
                        "r2jFreeCallback",
                        "(J)V",
                        &[JValue::Long(self.index as jlong)],
                    );
                }
            }

            let #rust_arg_name: Box<#struct_ident> = Box::new(#struct_ident{index: #arg_name_ident});
        };

        println!(
            "[bridge] ✅  end quote callback argument in method convert => {}.{}",
            &arg.name,
            &arg.ty.origin()
        );
        Ok(result)
    }

    fn return_convert(
        &self,
        ret_ty: &AstType,
        _trait_desc: &TraitDesc,
        _callbacks: &[&TraitDesc],
    ) -> Result<TokenStream> {
        let callback_ident = Ident::new(&ret_ty.origin(), Span::call_site());
        Ok(quote! {
            let callback_index = { *CALLBACK_INDEX.read().unwrap() };
            if callback_index == i64::MAX {
                *CALLBACK_INDEX.write().unwrap() = 0;
            } else {
                *CALLBACK_INDEX.write().unwrap() = callback_index + 1;
            }
            (*CALLBACK_HASHMAP.write().unwrap()).insert(callback_index, CallbackEnum::#callback_ident(ret_value));

            callback_index
        })
    }
}
