use heck::ToUpperCamelCase;
use proc_macro2::{Ident, Literal, TokenStream};
use rstgen::Java;

use crate::ast::contract::desc::TraitDesc;

use crate::ast::types::*;
use crate::base::lang::{
    BridgeContext, CallbackContext, Convertible, Direction, LangImp, MethodContext, ModContext,
    StructContext,
};
use crate::errors::*;
use crate::java::converter::JavaConvert;
use crate::java::JavaExtra;

use crate::ident;

pub(crate) fn index_to_callback(
    callback_desc: &TraitDesc,
    java_namespace: &str,
) -> Result<TokenStream> {
    let index_struct = quote! {
        #[derive(Serialize, Deserialize)]
        struct IndexStruct {
            index: i64,
        }
    };

    let class_name =
        format!("{}.Internal{}", java_namespace, &callback_desc.name).replace('.', "/");

    let mut all_method_tokens = TokenStream::new();
    for method in callback_desc.methods.iter() {
        println!("[bridge] 🔆  begin quote callback method");
        // arguments converting in callback
        let mut args_convert = TokenStream::new();
        let mut method_java_sig = "(J".to_owned();
        let mut cb_arg_array = quote!(JValue::Long(self.index),);
        for cb_arg in method.args.iter() {
            method_java_sig = format!("{}{}", &method_java_sig, cb_arg.ty.to_java_sig());

            let cb_arg_name = ident!(&format!("j_{}", cb_arg.name));
            let cb_origin_arg_name = ident!(&cb_arg.name);

            let convert = JavaConvert {
                ty: cb_arg.ty.clone(),
            }
            .rust_to_transferable(quote! {#cb_origin_arg_name}, Direction::Up);
            let args_convert_each = quote! {
                let #cb_arg_name = #convert;
            };

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
                AstType::Callback(_) => {
                    quote! {
                        JValue::Long(#cb_arg_name),
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
            .map(|arg| ident!(&arg.name))
            .collect::<Vec<Ident>>();
        let arg_types = &method
            .args
            .iter()
            .map(|arg| match arg.ty.clone() {
                AstType::Vec(vec_inner_name) => {
                    let vec_innder_ident = ident!(&vec_inner_name.origin());
                    quote!(Vec<#vec_innder_ident>)
                }
                AstType::Callback(ref origin) => {
                    let callback_ident = ident!(&origin.origin);
                    quote!(Box<dyn #callback_ident>)
                }
                _ => {
                    let ident = ident!(&arg.ty.origin());
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
                let origin_ident = ident!(&base.origin());
                quote!(Vec<#origin_ident>)
            }
            AstType::Callback(ref origin) => {
                let callback_ident = ident!(&origin.origin);
                quote!(Box<dyn #callback_ident>)
            }
            _ => {
                let ident = ident!(&method.return_type.origin());
                quote!(#ident)
            }
        };
        println!(
            "[bridge] ✅  end quote callback return type ident => {}.{}",
            &callback_desc.name, &method.name
        );

        let return_convert = if let AstType::Void = method.return_type.clone() {
            quote! {}
        } else {
            let convert = JavaConvert {
                ty: method.return_type.clone(),
            }
            .transferable_to_rust(quote! {result}, Direction::Up);
            quote! {
                let r_result = #convert;
            }
        };

        let return_result_ident = match method.return_type {
            AstType::Void => quote!(),
            _ => quote!(r_result),
        };

        // methods calls on impl
        let method_name = ident!(&method.name);
        let java_method_name = format!("r2j{}", &method.name.to_upper_camel_case());

        all_method_tokens = quote! {
            #all_method_tokens

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

    let callback_ident = ident!(&callback_desc.name);
    let result = quote! {
        #index_struct

        impl #callback_ident for IndexStruct {
            #all_method_tokens
        }

        impl Drop for IndexStruct {
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

        Box::new(IndexStruct{index: index})
    };

    println!("[bridge] ✅  end quote callback argument in method convert",);
    Ok(result)
}

pub struct JavaImp {}

impl LangImp<Java<'static>, JavaExtra> for JavaImp {
    fn quote_lib_file(
        &self,
        context: &BridgeContext<Java<'static>, JavaExtra>,
    ) -> Result<TokenStream> {
        let mut bridges: Vec<String> = vec![];
        for (mod_name, _traits) in &context.ast.traits {
            let out_mod_name = format!("java_{}", mod_name);
            bridges.push(out_mod_name);
        }

        let mod_idents = bridges
            .iter()
            .map(|name| ident!(name))
            .collect::<Vec<Ident>>();

        let host_crate_underscore = ident!(&context.crate_name.replace("-", "_"));
        Ok(quote! {
            #![allow(warnings)]
            extern crate #host_crate_underscore;
            extern crate jni;
            #[macro_use]
            extern crate serde_derive;
            extern crate serde;
            #[macro_use]
            extern crate lazy_static;
            #[macro_use]
            extern crate log;
            use log::Level;

            use jni::sys::JNI_VERSION_1_6;
            use jni::JNIEnv;
            use jni::JavaVM;
            use jni::sys::{jint, jlong, jstring, jbyteArray};
            use std::os::raw::c_void;
            use std::mem;

            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn JNI_OnLoad(jvm: JavaVM, _reserved: *mut c_void) -> jint {
                set_java_vm(jvm);
                JNI_VERSION_1_6
            }

            pub fn set_java_vm(jvm: JavaVM) {
                #(crate::#mod_idents::set_global_vm(jvm);)*
            }
        })
    }

    fn quote_common_file(
        &self,
        _context: &BridgeContext<Java<'static>, JavaExtra>,
    ) -> Result<TokenStream> {
        Ok(quote! {})
    }

    fn quote_use_part(
        &self,
        _context: &ModContext<Java<'static>, JavaExtra>,
    ) -> Result<TokenStream> {
        Ok(quote! {
            use jni::JNIEnv;
            use jni::JavaVM;
            use jni::objects::{JClass, JObject, JString, JValue};
            use jni::sys::{jint, jlong, jstring, jbyteArray};
            use std::os::raw::c_void;
            use jni::sys::JNI_VERSION_1_6;
            use std::sync::RwLock;
            use log::Level;
            use std::sync::Arc;
            use std::collections::HashMap;
        })
    }

    fn quote_common_part(
        &self,
        context: &ModContext<Java<'static>, JavaExtra>,
    ) -> Result<TokenStream> {
        let class_names = context
            .traits
            .iter()
            .map(|desc| {
                format!(
                    "{}.Internal{}",
                    context.bridge_ctx.extra.namespace.to_owned(),
                    &desc.name
                )
                .replace('.', "/")
            })
            .collect::<Vec<String>>();

        Ok(quote! {
            lazy_static! {
                static ref JVM : Arc<RwLock<Option<JavaVM>>> = Arc::new(RwLock::new(None));
                static ref CALLBACK_HASHMAP: Arc<RwLock<HashMap<i64, CallbackEnum>>> =  Arc::new(RwLock::new(HashMap::new()));
                static ref CALLBACK_INDEX : Arc<RwLock<i64>> = Arc::new(RwLock::new(0));
            }

            pub fn set_global_vm(jvm: JavaVM) {
                #(let _ = jvm.get_env().unwrap().find_class(#class_names);)*
                *(JVM.write().unwrap()) = Some(jvm);
            }
        })
    }

    fn quote_method_sig(
        &self,
        context: &MethodContext<Java<'static>, JavaExtra>,
    ) -> Result<TokenStream> {
        println!(
            "[bridge][{}.{}]  🔆  begin quote jni bridge method signature.",
            &context.service_ctx.trait_.name, &context.method.name
        );
        let namespace = context
            .service_ctx
            .mod_ctx
            .bridge_ctx
            .extra
            .namespace
            .replace('.', "_");

        let method_name = format!(
            "Java_{}_Internal{}_native{}",
            &namespace,
            context.service_ctx.trait_.name.clone(),
            &context.method.name.to_upper_camel_case().replace('_', "_1")
        );
        let method_name_ident = ident!(&method_name);
        let arg_names = context
            .method
            .args
            .iter()
            .map(|arg| ident!(&arg.name))
            .collect::<Vec<Ident>>();

        let arg_types = context
            .method
            .args
            .iter()
            .map(|arg| JavaConvert { ty: arg.ty.clone() }.rust_transferable_type(Direction::Down))
            .collect::<Vec<TokenStream>>();

        let ret_ty_tokens = JavaConvert {
            ty: context.method.return_type.clone(),
        }
        .rust_transferable_type(Direction::Up);

        let method_sig = if arg_names.is_empty() {
            match context.method.return_type {
                AstType::Void => quote! {
                    #[no_mangle]
                    #[allow(non_snake_case)]
                    pub extern "C" fn #method_name_ident(env: JNIEnv, class: JClass)
                },
                _ => quote! {
                    #[no_mangle]
                    #[allow(non_snake_case)]
                    pub extern "C" fn #method_name_ident(env: JNIEnv, class: JClass) -> #ret_ty_tokens
                },
            }
        } else {
            match context.method.return_type {
                AstType::Void => quote! {
                    #[no_mangle]
                    #[allow(non_snake_case)]
                    pub extern "C" fn #method_name_ident(env: JNIEnv, class: JClass, #(#arg_names: #arg_types),*)
                },

                _ => quote! {
                    #[no_mangle]
                    #[allow(non_snake_case)]
                    pub extern "C" fn #method_name_ident(env: JNIEnv, class: JClass, #(#arg_names: #arg_types),*) -> #ret_ty_tokens
                },
            }
        };
        println!(
            "[bridge][{}.{}]  ✅  end quote jni bridge method signature.",
            &context.service_ctx.trait_.name, &context.method.name
        );
        Ok(method_sig)
    }

    fn quote_for_one_struct(
        &self,
        context: &StructContext<Java<'static>, JavaExtra>,
    ) -> Result<TokenStream> {
        let struct_name = ident!(&format!("Proxy{}", &context.struct_.name));
        let origin_struct_name = ident!(&context.struct_.name);
        let names = context
            .struct_
            .fields
            .iter()
            .map(|field| ident!(&field.name))
            .collect::<Vec<Ident>>();
        let arg_names = names.clone();
        let origin_arg_names = names.clone();
        let tys = context
            .struct_
            .fields
            .iter()
            .map(|field| ident!(&field.ty.origin()))
            .collect::<Vec<Ident>>();
        Ok(quote! {
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

    fn quote_for_one_callback(
        &self,
        context: &CallbackContext<Java<'static>, JavaExtra>,
    ) -> Result<TokenStream> {
        let callback = context.callback;
        let callback_ident = ident!(&callback.name);

        let mut body = TokenStream::new();

        let namespace = context.mod_ctx.bridge_ctx.extra.namespace.replace('.', "_");
        for method in callback.methods.iter() {
            let method_name = format!(
                "Java_{}_Internal{}_j2r{}",
                &namespace,
                callback.name.clone(),
                &method.name.to_upper_camel_case().replace('_', "_1")
            );
            let origin_method_name = ident!(&method.name);
            let method_name_ident = ident!(&method_name);
            let arg_names = method
                .args
                .iter()
                .map(|arg| ident!(&arg.name))
                .collect::<Vec<Ident>>();

            let arg_types = method
                .args
                .iter()
                .map(|arg| {
                    JavaConvert { ty: arg.ty.clone() }.rust_transferable_type(Direction::Down)
                })
                .collect::<Vec<TokenStream>>();

            let ret_ty_tokens = JavaConvert {
                ty: method.return_type.clone(),
            }
            .rust_transferable_type(Direction::Up);

            let mut args_convert = TokenStream::new();
            for arg in method.args.iter() {
                let rust_arg_str = format!("r_{}", &arg.name);
                let rust_arg_name = ident!(&rust_arg_str);
                let arg_name_ident = ident!(&arg.name);
                let convert = JavaConvert { ty: arg.ty.clone() }
                    .transferable_to_rust(quote! {#arg_name_ident}, Direction::Down);
                let each_convert = quote! {
                    let #rust_arg_name = #convert;
                };

                args_convert = quote! {
                    #args_convert
                    #each_convert
                }
            }

            let r_arg_names = &method
                .args
                .iter()
                .filter(|arg| !matches!(arg.ty, AstType::Void))
                .map(|arg| ident!(&format!("r_{}", &arg.name)))
                .collect::<Vec<Ident>>();

            let mut return_convert = if let AstType::Void = method.return_type.clone() {
                quote! {}
            } else {
                JavaConvert {
                    ty: method.return_type.clone(),
                }
                .rust_to_transferable(quote! {result}, Direction::Down)
            };

            if let AstType::Callback(ref origin) = method.return_type.clone() {
                let return_callback_ident = ident!(&origin.origin);

                body = quote! {
                    #body

                    #[no_mangle]
                    #[allow(non_snake_case)]
                    pub extern "C" fn #method_name_ident(env: JNIEnv, class: JClass, index: i64, #(#arg_names: #arg_types),*) -> #ret_ty_tokens {
                        #args_convert
                        let mut result: Option<Box<dyn #return_callback_ident >> = None;
                        let final_result = {
                            let callback_hashmap = &*CALLBACK_HASHMAP.read().unwrap();
                            let ret_callback = callback_hashmap.get(&index);
                            match ret_callback {
                                Some(ret_callback) => {
                                    if let CallbackEnum::#callback_ident(ret_callback) = ret_callback {
                                        result = Some(ret_callback.#origin_method_name(#(#r_arg_names),*));
                                        {
                                            let mut global_index = CALLBACK_INDEX.write().unwrap();
                                            let mut index = *global_index;
                                            if index == i64::MAX {
                                                *global_index = 0;
                                                index = 0;
                                            } else {
                                                *global_index = index + 1;
                                                index = index + 1;
                                            }
                                            index
                                        }
                                    } else {
                                        panic!("Callback doesn't match for index: {}", index);
                                    }
                                }
                                None => {
                                    panic!("No callback found for index: {}", index);
                                }
                            }
                        };

                        (*CALLBACK_HASHMAP.write().unwrap()).insert(final_result, CallbackEnum::#return_callback_ident(result.unwrap()));
                        final_result
                    }

                }
            } else {
                body = quote! {
                    #body

                    #[no_mangle]
                    #[allow(non_snake_case)]
                    pub extern "C" fn #method_name_ident(env: JNIEnv, class: JClass, index: i64, #(#arg_names: #arg_types),*) -> #ret_ty_tokens {
                        #args_convert
                        let callback_hashmap = &*CALLBACK_HASHMAP.read().unwrap();
                        let ret_callback = callback_hashmap.get(&index);
                        match ret_callback {
                            Some(ret_callback) => {
                                if let CallbackEnum::#callback_ident(ret_callback) = ret_callback {
                                    let mut result = ret_callback.#origin_method_name(#(#r_arg_names),*);
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
                }
            }
        }

        let free_method_name = ident!(&format!(
            "Java_{}_Internal{}_j2rFreeCallback",
            &namespace,
            callback.name.clone(),
        ));

        body = quote! {
            #body

            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #free_method_name(env: JNIEnv, class: JClass, index: i64) {
                (*CALLBACK_HASHMAP.write().unwrap()).remove(&index);
            }
        };

        let index_to_cb_fn_name = ident!(&format!("index_to_callback_{}", &callback.name));

        let callback_ident = ident!(&callback.name);
        let index_to_cb_fn_body =
            index_to_callback(callback, &context.mod_ctx.bridge_ctx.extra.namespace)?;
        let index_to_cb_fn = quote! {
            fn #index_to_cb_fn_name(index: i64) -> Box<dyn #callback_ident> {
                #index_to_cb_fn_body
            }
        };

        let cb_to_index_fn_name = ident!(&format!("callback_to_index_{}", &callback.name));
        let cb_to_index_fn = quote! {
            fn #cb_to_index_fn_name(callback: Box<dyn #callback_ident>) -> i64 {
                let callback_index = {
                    let mut global_index = CALLBACK_INDEX.write().unwrap();
                    let mut index = *global_index;
                    if index == i64::MAX {
                        *global_index = 0;
                        index = 0;
                    } else {
                        *global_index = index + 1;
                        index = index + 1;
                    }
                    index
                };
                (*CALLBACK_HASHMAP.write().unwrap()).insert(callback_index, CallbackEnum::#callback_ident(callback));

                callback_index
            }

        };

        Ok(quote! {
            #body
            #index_to_cb_fn
            #cb_to_index_fn
        })
    }

    fn provide_converter(&self, ty: &AstType) -> Box<dyn Convertible<Java<'static>>> {
        Box::new(JavaConvert { ty: ty.clone() })
    }
}
