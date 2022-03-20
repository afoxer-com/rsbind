use std::path::Path;

use heck::ToUpperCamelCase;
use proc_macro2::{Ident, Span, TokenStream};

use crate::ast::contract::desc::{ArgDesc, MethodDesc, StructDesc, TraitDesc};
use crate::ast::imp::desc::*;
use crate::ast::types::*;
use crate::bridge::file::*;
use crate::errors::*;

use super::bridge_cb::*;

///
/// create a new generator for java bridge files.
///
pub(crate) fn new_gen<'a>(
    out_dir: &'a Path,
    trait_descs: &'a [TraitDesc],
    struct_descs: &'a [StructDesc],
    imp_desc: &'a [ImpDesc],
    java_namespace: &'a str,
) -> BridgeFileGen<'a, JniFileGenStrategy<'a>> {
    BridgeFileGen {
        out_dir,
        trait_descs,
        struct_descs,
        imp_desc,
        strategy: JniFileGenStrategy {
            java_namespace,
            java_callback_strategy: JavaCallbackStrategy {
                java_namespace: java_namespace.to_owned(),
            },
        },
    }
}

pub(crate) struct JniFileGenStrategy<'a> {
    java_namespace: &'a str,
    java_callback_strategy: JavaCallbackStrategy,
}

impl<'a> FileGenStrategy for JniFileGenStrategy<'a> {
    fn gen_sdk_file(&self, mod_names: &[String]) -> Result<TokenStream> {
        let mod_idents = mod_names
            .iter()
            .map(|name| Ident::new(name, Span::call_site()))
            .collect::<Vec<Ident>>();
        Ok(quote! {
            use jni::sys::JNI_VERSION_1_6;
            use jni::JNIEnv;
            use jni::JavaVM;
            use jni::sys::{jint, jlong, jstring, jbyteArray};
            use std::os::raw::c_void;
            use std::mem;

            #[cfg(feature = "rsbind")]
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn JNI_OnLoad(jvm: JavaVM, _reserved: *mut c_void) -> jint {
                set_java_vm(jvm);
                JNI_VERSION_1_6
            }

            pub fn set_java_vm(jvm: JavaVM) {
                #(::java::bridge::#mod_idents::set_global_vm(jvm);)*
            }

        })
    }

    fn quote_common_use_part(&self) -> Result<TokenStream> {
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

    fn quote_common_part(&self, trait_desc: &[TraitDesc]) -> Result<TokenStream> {
        let class_names = trait_desc
            .iter()
            .map(|desc| {
                format!("{}.Internal{}", self.java_namespace, &desc.name).replace('.', "/")
            })
            .collect::<Vec<String>>();

        Ok(quote! {
            lazy_static! {
                static ref JVM : Arc<RwLock<Option<JavaVM>>> = Arc::new(RwLock::new(None));
                static ref CALLBACK_HASHMAP: RwLock<HashMap<i64, CallbackEnum>> =  RwLock::new(HashMap::new());
                static ref CALLBACK_INDEX : RwLock<i64> = RwLock::new(0);
            }

            pub fn set_global_vm(jvm: JavaVM) {
                // #(let _ = jvm.get_env().unwrap().find_class(#class_names);)*
                *(JVM.write().unwrap()) = Some(jvm);
            }
        })
    }

    fn quote_for_all_cb(&self, callbacks: &[&TraitDesc]) -> Result<TokenStream> {
        let enum_items = callbacks
            .iter()
            .map(|item| Ident::new(&item.name, Span::call_site()))
            .collect::<Vec<Ident>>();

        let enum_tokens = quote! {
            enum CallbackEnum {
                #(#enum_items(Box<dyn #enum_items>)),*
            }
        };

        let mut methods_tokens = TokenStream::new();
        for callback in callbacks.iter() {
            let each_callback_tokens = self.quote_method_for_callback(callback, callbacks)?;
            methods_tokens = quote! {
                #methods_tokens

                #each_callback_tokens
            }
        }

        Ok(quote! {
            #enum_tokens

            #methods_tokens
        })
    }

    fn quote_callback_structures(&self, _trait_desc: &TraitDesc) -> Result<TokenStream> {
        Ok(quote! {})
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

    ///
    /// quote the method signature for java bridge file.
    ///
    fn quote_method_sig(
        &self,
        trait_desc: &TraitDesc,
        _impl_desc: &ImpDesc,
        method: &MethodDesc,
        _callbacks: &[&TraitDesc],
        _structs: &[StructDesc],
    ) -> Result<TokenStream> {
        println!(
            "[bridge][{}.{}]  🔆  begin quote jni bridge method signature.",
            &trait_desc.name, &method.name
        );
        let namespace = self.java_namespace.replace('.', "_");
        let method_name = format!(
            "Java_{}_Internal{}_native{}",
            &namespace,
            trait_desc.name.clone(),
            &method.name.to_upper_camel_case().replace('_', "_1")
        );
        let method_name_ident = Ident::new(&method_name, Span::call_site());
        let arg_names = method
            .args
            .iter()
            .map(|arg| Ident::new(&arg.name, Span::call_site()))
            .collect::<Vec<Ident>>();

        let arg_types = method
            .args
            .iter()
            .map(|arg| self.ty_to_tokens(&arg.ty, TypeDirection::Argument).unwrap())
            .collect::<Vec<TokenStream>>();

        let ret_ty_tokens = self.ty_to_tokens(&method.return_type, TypeDirection::Return)?;

        let method_sig = if arg_names.is_empty() {
            match method.return_type {
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
            match method.return_type {
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
            &trait_desc.name, &method.name
        );
        Ok(method_sig)
    }

    fn quote_arg_convert(
        &self,
        trait_desc: &TraitDesc,
        arg: &ArgDesc,
        callbacks: &[&TraitDesc],
    ) -> Result<TokenStream> {
        println!(
            "[bridge]  🔆  begin quote jni bridge method argument convert => {}:{}",
            &arg.name,
            &arg.ty.origin()
        );
        let result = match arg.clone().ty {
            AstType::Callback(_) => self
                .java_callback_strategy
                .arg_convert(arg, trait_desc, callbacks),
            _ => crate::java::bridge_j2r::quote_arg_convert(arg, self.java_namespace, trait_desc),
        };
        println!(
            "[bridge] ✅ end quote jni bridge method argument convert => {}:{}",
            &arg.name,
            &arg.ty.origin()
        );
        result
    }

    fn quote_return_convert(
        &self,
        trait_desc: &TraitDesc,
        callbacks: &[&TraitDesc],
        return_ty: &AstType,
        ret_name: &str,
    ) -> Result<TokenStream> {
        println!(
            "[bridge]  🔆  begin quote jni bridge method return convert => {}",
            return_ty.origin()
        );

        let result = match return_ty.clone() {
            AstType::Callback(_) => self
                .java_callback_strategy
                .return_convert(return_ty, trait_desc, callbacks),
            _ => crate::java::bridge_j2r::quote_return_convert(
                return_ty, trait_desc, callbacks, ret_name,
            ),
        };
        println!(
            "[bridge]  ✅  end quote jni bridge method return convert => {}",
            return_ty.origin()
        );

        result
    }

    fn ty_to_tokens(&self, ast_type: &AstType, direction: TypeDirection) -> Result<TokenStream> {
        crate::java::bridge_j2r::ty_to_tokens(ast_type, direction)
    }
}

impl<'a> JniFileGenStrategy<'a> {
    fn quote_method_for_callback(
        &self,
        callback: &TraitDesc,
        callbacks: &[&TraitDesc],
    ) -> Result<TokenStream> {
        let callback_ident = Ident::new(&callback.name, Span::call_site());

        let mut body = TokenStream::new();

        let namespace = self.java_namespace.replace('.', "_");
        for method in callback.methods.iter() {
            let method_name = format!(
                "Java_{}_Internal{}_j2r{}",
                &namespace,
                callback.name.clone(),
                &method.name.to_upper_camel_case().replace('_', "_1")
            );
            let origin_method_name = Ident::new(&method.name, Span::call_site());
            let method_name_ident = Ident::new(&method_name, Span::call_site());
            let arg_names = method
                .args
                .iter()
                .map(|arg| Ident::new(&arg.name, Span::call_site()))
                .collect::<Vec<Ident>>();

            let arg_types = method
                .args
                .iter()
                .map(|arg| {
                    crate::java::bridge_j2r::ty_to_tokens(&arg.ty, TypeDirection::Argument).unwrap()
                })
                .collect::<Vec<TokenStream>>();

            let ret_ty_tokens =
                crate::java::bridge_j2r::ty_to_tokens(&method.return_type, TypeDirection::Return)?;
            let method_sig = if arg_names.is_empty() {
                match method.return_type {
                    AstType::Void => quote! {
                        #[no_mangle]
                        #[allow(non_snake_case)]
                        pub extern "C" fn #method_name_ident(env: JNIEnv, class: JClass, index: i64)
                    },
                    _ => quote! {
                        #[no_mangle]
                        #[allow(non_snake_case)]
                        pub extern "C" fn #method_name_ident(env: JNIEnv, class: JClass, index: i64) -> #ret_ty_tokens
                    },
                }
            } else {
                match method.return_type {
                    AstType::Void => quote! {
                        #[no_mangle]
                        #[allow(non_snake_case)]
                        pub extern "C" fn #method_name_ident(env: JNIEnv, class: JClass, index: i64, #(#arg_names: #arg_types),*)
                    },

                    _ => quote! {
                        #[no_mangle]
                        #[allow(non_snake_case)]
                        pub extern "C" fn #method_name_ident(env: JNIEnv, class: JClass, index: i64, #(#arg_names: #arg_types),*) -> #ret_ty_tokens
                    },
                }
            };

            let mut args_convert = TokenStream::new();
            for arg in method.args.iter() {
                let each_convert =
                    crate::java::bridge_j2r::quote_arg_convert(arg, &namespace, callback)?;
                args_convert = quote! {
                    #args_convert
                    #each_convert
                }
            }

            let r_arg_names = &method
                .args
                .iter()
                .filter(|arg| !matches!(arg.ty, AstType::Void))
                .map(|arg| Ident::new(&format!("r_{}", &arg.name), Span::call_site()))
                .collect::<Vec<Ident>>();

            let return_convert = crate::java::bridge_j2r::quote_return_convert(
                &method.return_type,
                callback,
                callbacks,
                "r_result",
            )?;

            body = quote! {
                #body

                #method_sig {
                    #args_convert

                    let callback_hashmap = &*CALLBACK_HASHMAP.read().unwrap();
                        let ret_callback = callback_hashmap.get(&index);
                        match ret_callback {
                            Some(ret_callback) => {
                                if let CallbackEnum::#callback_ident(ret_callback) = ret_callback {
                                    let mut r_result = ret_callback.#origin_method_name(#(#r_arg_names),*);
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

        let free_method_name = Ident::new(
            &format!(
                "Java_{}_Internal{}_j2rFreeCallback",
                &namespace,
                callback.name.clone(),
            ),
            Span::call_site(),
        );

        body = quote! {
            #body

            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #free_method_name(env: JNIEnv, class: JClass, index: i64) {
                (*CALLBACK_HASHMAP.write().unwrap()).remove(&index);
            }
        };

        Ok(quote! {
            #body
        })
    }
}
