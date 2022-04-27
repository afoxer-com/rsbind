use std::cmp::Ordering;
use std::path::Path;

use heck::ToUpperCamelCase;
use proc_macro2::{Ident, Literal, TokenStream};

use crate::ast::contract::desc::{ArgDesc, MethodDesc, StructDesc, TraitDesc};
use crate::ast::imp::desc::*;
use crate::ast::types::*;
use crate::bridges::ModGenStrategy;
use crate::errors::*;
use crate::ident;
use crate::ErrorKind::GenerateError;

pub(crate) struct JavaGenStrategyImp {
    pub(crate) namespace: String,
}

impl ModGenStrategy for JavaGenStrategyImp {
    fn mod_name(&self, mod_name: &str) -> String {
        format!("java_{}", mod_name)
    }

    fn sdk_file_gen(&self, mod_names: &[String]) -> Result<TokenStream> {
        let mod_idents = mod_names
            .iter()
            .map(|name| ident!(name))
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

    fn common_file_gen(&self) -> Result<TokenStream> {
        Ok(quote! {})
    }

    fn bridge_file_gen(
        &self,
        traits: &[TraitDesc],
        structs: &[StructDesc],
        imps: &[ImpDesc],
    ) -> Result<TokenStream> {
        BridgeFileGen {
            traits,
            structs,
            imps,
            namespace: self.namespace.clone(),
        }
        .gen_one_bridge_file()
    }
}

pub(crate) enum TypeDirection {
    Argument,
    Return,
}

///
/// Executor for generating core files of bridge mod.
///
pub(crate) struct BridgeFileGen<'a> {
    pub traits: &'a [TraitDesc],
    pub structs: &'a [StructDesc],
    pub imps: &'a [ImpDesc],
    pub namespace: String,
}

impl<'a> BridgeFileGen<'a> {
    ///
    /// generate one bridge file for one contract mod.
    ///
    pub(crate) fn gen_one_bridge_file(&self) -> Result<TokenStream> {
        println!("[bridge] 🔆  begin generate bridge file.");
        let use_part = self.quote_use_part().unwrap();
        let common_part = self.quote_common_part(self.traits).unwrap();
        let bridge_codes = self.gen_for_one_mod().unwrap();

        let mut merge_tokens = quote! {
            #use_part
            #common_part
        };

        for bridge_code in bridge_codes {
            if let Ok(code) = bridge_code {
                merge_tokens = quote! {
                    #merge_tokens
                    #code
                };
            }
        }

        println!("[bridge] ✅  end generate bridge file.");
        Ok(merge_tokens)
    }

    ///
    /// generate bridge file from a file of trait.
    ///
    fn gen_for_one_mod(&self) -> Result<Vec<Result<TokenStream>>> {
        let mut results: Vec<Result<TokenStream>> = vec![];

        let callbacks = self
            .traits
            .iter()
            .filter(|desc| desc.is_callback)
            .collect::<Vec<&TraitDesc>>();

        println!("callbacks is {:?}", &callbacks);

        for desc in self.traits.iter() {
            if desc.is_callback {
                results.push(self.quote_callback_structures(desc, &callbacks));
                continue;
            }

            let imps = self
                .imps
                .iter()
                .filter(|info| info.contract == desc.name)
                .collect::<Vec<&ImpDesc>>();

            println!("desc => {:?}", desc);
            println!("imps => {:?}", imps);
            println!("all imps => {:?}", &self.imps);

            match imps.len().cmp(&1) {
                Ordering::Less => {}
                Ordering::Equal => {
                    results.push(self.generate_for_one_trait(
                        desc,
                        imps[0],
                        &callbacks,
                        self.structs,
                    ));
                }
                Ordering::Greater => {
                    println!("You have more than one impl for trait {}", desc.name);
                    return Err(GenerateError(format!(
                        "You have more than one impl for trait {}",
                        desc.name
                    ))
                    .into());
                }
            }
        }

        let tokens = self.quote_for_all_cb(&callbacks);
        results.push(tokens);

        for struct_desc in self.structs.iter() {
            let tokens = self.quote_for_structures(struct_desc);
            results.push(tokens);
        }

        Ok(results)
    }

    fn generate_for_one_trait(
        &self,
        trait_desc: &TraitDesc,
        imp: &ImpDesc,
        callbacks: &[&TraitDesc],
        structs: &[StructDesc],
    ) -> Result<TokenStream> {
        println!(
            "[bridge][{}]  🔆  begin generate bridge on trait.",
            &trait_desc.name
        );
        let mut merge: TokenStream = TokenStream::new();

        for method in trait_desc.methods.iter() {
            println!(
                "[bridge][{}.{}]  🔆  begin generate bridge method.",
                &trait_desc.name, &method.name
            );
            let one_method = self
                .quote_one_method(trait_desc, imp, method, callbacks, structs)
                .unwrap();

            println!(
                "[bridge][{}.{}]  ✅  end generate bridge method.",
                &trait_desc.name, &method.name
            );

            merge = quote! {
                #merge
                #one_method
            };
        }
        println!(
            "[bridge][{}]  ✅  end generate bridge on trait.",
            &trait_desc.name
        );
        Ok(merge)
    }

    ///
    /// quote use part
    ///
    fn quote_use_part(&self) -> Result<TokenStream> {
        println!("[bridge]  🔆  begin quote use part.");
        let mut merge = self.quote_common_use_part().unwrap();

        for trait_desc in self.traits.iter() {
            if trait_desc.is_callback {
                println!("Skip callback trait {}", &trait_desc.name);
                continue;
            }

            let imps = self
                .imps
                .iter()
                .filter(|info| info.contract == trait_desc.name)
                .collect::<Vec<&ImpDesc>>();

            match imps.len().cmp(&1) {
                Ordering::Less => {}
                Ordering::Equal => {
                    let use_part = self
                        .quote_one_use_part(&trait_desc.mod_path, &imps[0].mod_path)
                        .unwrap();
                    merge = quote! {
                       #use_part
                       #merge
                    };
                }
                Ordering::Greater => {
                    println!("You have more than one impl for trait {}", trait_desc.name);
                    return Err(GenerateError(format!(
                        "You have more than one impl for trait {}",
                        trait_desc.name
                    ))
                    .into());
                }
            }
        }
        println!("[bridge]  ✅  end quote use part.");
        Ok(merge)
    }

    fn quote_one_use_part(&self, trait_mod_path: &str, imp_mod_path: &str) -> Result<TokenStream> {
        let trait_mod_splits: Vec<Ident> = trait_mod_path
            .split("::")
            .collect::<Vec<&str>>()
            .iter()
            .map(|str| ident!(str))
            .collect();
        let imp_mod_splits: Vec<Ident> = imp_mod_path
            .split("::")
            .collect::<Vec<&str>>()
            .iter()
            .map(|str| ident!(str))
            .collect();

        Ok(quote! {
            use #(#trait_mod_splits::)**;
            use #(#imp_mod_splits::)**;
        })
    }

    ///
    /// quote one method
    ///
    fn quote_one_method(
        &self,
        trait_desc: &TraitDesc,
        imp: &ImpDesc,
        method: &MethodDesc,
        callbacks: &[&TraitDesc],
        structs: &[StructDesc],
    ) -> Result<TokenStream> {
        println!(
            "[bridge][{}.{}]  🔆 ️begin quote method.",
            &trait_desc.name, &method.name
        );
        let sig_define = self
            .quote_method_sig(trait_desc, imp, method, callbacks, structs)
            .unwrap();

        let mut arg_convert = TokenStream::new();
        for arg in method.args.iter() {
            let arg_tokens = self.quote_arg_convert(trait_desc, arg, callbacks).unwrap();
            arg_convert = quote! {
                #arg_convert
                #arg_tokens
            }
        }

        let call_imp = self.quote_imp_call(&imp.name, method)?;

        let return_handle =
            self.quote_return_convert(trait_desc, callbacks, &method.return_type, "result")?;

        // combine all the parts
        let result = quote! {
            #sig_define {
                #arg_convert
                #call_imp
                #return_handle
            }
        };

        println!(
            "[bridge][{}.{}] ✅ end quote method.",
            &trait_desc.name, &method.name
        );
        Ok(result)
    }

    fn quote_imp_call(&self, impl_name: &str, method: &MethodDesc) -> Result<TokenStream> {
        println!(
            "[bridge][{}.{}]  🔆 ️begin quote imp call.",
            impl_name, &method.name
        );

        let ret_name_str = "result";
        let imp_fun_name = ident!(&method.name);
        let ret_name_ident = ident!(ret_name_str);

        let tmp_arg_names = method
            .args
            .iter()
            .map(|e| &e.name)
            .map(|arg_name| ident!(&format!("r_{}", arg_name)))
            .collect::<Vec<Ident>>();

        let rust_args_repeat = quote! {
            #(#tmp_arg_names),*
        };

        let imp_ident = ident!(impl_name);
        let imp_call = match method.return_type.clone() {
            AstType::Void => quote! {
                let #ret_name_ident = #imp_ident::#imp_fun_name(#rust_args_repeat);
            },
            AstType::Vec(AstBaseType::Byte(_))
            | AstType::Vec(AstBaseType::Short(_))
            | AstType::Vec(AstBaseType::Int(_))
            | AstType::Vec(AstBaseType::Long(_)) => {
                quote! {
                    let mut #ret_name_ident = #imp_ident::#imp_fun_name(#rust_args_repeat);
                }
            }
            AstType::Vec(_)
            | AstType::Struct(_)
            | AstType::Callback(_)
            | AstType::String
            | AstType::Byte(_)
            | AstType::Short(_)
            | AstType::Int(_)
            | AstType::Long(_)
            | AstType::Float(_)
            | AstType::Double(_)
            | AstType::Boolean => {
                quote! {
                    let #ret_name_ident = #imp_ident::#imp_fun_name(#rust_args_repeat);
                }
            }
        };

        println!(
            "[bridge][{}.{}]  ✅ end quote imp call.",
            impl_name, &method.name
        );

        Ok(imp_call)
    }
}

impl<'a> BridgeFileGen<'a> {
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
            .map(|desc| format!("{}.Internal{}", self.namespace, &desc.name).replace('.', "/"))
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

    fn quote_for_all_cb(&self, callbacks: &[&TraitDesc]) -> Result<TokenStream> {
        let enum_items = callbacks
            .iter()
            .map(|item| ident!(&item.name))
            .collect::<Vec<Ident>>();

        let enum_tokens = quote! {
            enum CallbackEnum {
                #(#enum_items(Box<dyn #enum_items>)),*
            }
        };

        let mut methods_tokens = TokenStream::new();
        for callback in callbacks.iter() {
            let each_callback_tokens = self.quote_j2r_method_for_callback(callback, callbacks)?;
            methods_tokens = quote! {
                #methods_tokens

                #each_callback_tokens
            }
        }

        let mut index_to_cb_fns = quote!();
        let mut cb_to_index_fns = quote!();

        for callback in callbacks.iter() {
            let index_to_cb_fn_name = ident!(&format!("index_to_callback_{}", &callback.name));

            let callback_ident = ident!(&callback.name);
            let index_to_cb_fn_body = index_to_callback(callback, &self.namespace)?;
            let index_to_cb_fn = quote! {
                fn #index_to_cb_fn_name(index: i64) -> Box<dyn #callback_ident> {
                    #index_to_cb_fn_body
                }
            };

            index_to_cb_fns = quote! {
                #index_to_cb_fns

                #index_to_cb_fn
            };

            let cb_to_index_fn_name = ident!(&format!("callback_to_index_{}", &callback.name));
            cb_to_index_fns = quote! {
                #cb_to_index_fns
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
        }

        Ok(quote! {
            #enum_tokens

            #methods_tokens

            #index_to_cb_fns

            #cb_to_index_fns
        })
    }

    fn quote_callback_structures(
        &self,
        _trait_desc: &TraitDesc,
        _callbacks: &[&TraitDesc],
    ) -> Result<TokenStream> {
        Ok(quote! {})
    }

    fn quote_for_structures(&self, struct_desc: &StructDesc) -> Result<TokenStream> {
        let struct_name = ident!(&format!("Proxy{}", &struct_desc.name));
        let origin_struct_name = ident!(&struct_desc.name);
        let names = struct_desc
            .fields
            .iter()
            .map(|field| ident!(&field.name))
            .collect::<Vec<Ident>>();
        let arg_names = names.clone();
        let origin_arg_names = names.clone();
        let tys = struct_desc
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
        let namespace = self.namespace.replace('.', "_");
        let method_name = format!(
            "Java_{}_Internal{}_native{}",
            &namespace,
            trait_desc.name.clone(),
            &method.name.to_upper_camel_case().replace('_', "_1")
        );
        let method_name_ident = ident!(&method_name);
        let arg_names = method
            .args
            .iter()
            .map(|arg| ident!(&arg.name))
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
        _callbacks: &[&TraitDesc],
    ) -> Result<TokenStream> {
        println!(
            "[bridge]  🔆  begin quote jni bridge method argument convert => {}:{}",
            &arg.name,
            &arg.ty.origin()
        );
        let result = crate::java::bridge_j2r::quote_arg_convert(arg, &self.namespace, trait_desc);
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

        let result = crate::java::bridge_j2r::quote_return_convert(
            return_ty, trait_desc, callbacks, ret_name,
        );
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

impl<'a> BridgeFileGen<'a> {
    fn quote_j2r_method_for_callback(
        &self,
        callback: &TraitDesc,
        callbacks: &[&TraitDesc],
    ) -> Result<TokenStream> {
        let callback_ident = ident!(&callback.name);

        let mut body = TokenStream::new();

        let namespace = self.namespace.replace('.', "_");
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
                    crate::java::bridge_j2r::ty_to_tokens(&arg.ty, TypeDirection::Argument).unwrap()
                })
                .collect::<Vec<TokenStream>>();

            let ret_ty_tokens =
                crate::java::bridge_j2r::ty_to_tokens(&method.return_type, TypeDirection::Return)?;

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
                .map(|arg| ident!(&format!("r_{}", &arg.name)))
                .collect::<Vec<Ident>>();

            let return_convert = crate::java::bridge_j2r::quote_return_convert(
                &method.return_type,
                callback,
                callbacks,
                "result",
            )?;

            if let AstType::Callback(ref origin) = method.return_type.clone() {
                let return_callback_ident = ident!(origin);

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

        Ok(quote! {
            #body
        })
    }
}

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
            let cb_arg_name = ident!(&format!("j_{}", cb_arg.name));
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
                    let callback_ident = ident!(origin);
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
                let callback_ident = ident!(origin);
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

        let return_convert = crate::java::bridge_r2j::return_convert(method)?;

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
