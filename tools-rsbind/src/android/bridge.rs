use super::callback::*;
use ast::contract::desc::{ArgDesc, MethodDesc, StructDesc, TraitDesc};
use ast::imp::desc::*;
use ast::types::*;
use bridge::file::*;
use errors::ErrorKind::*;
use errors::*;
use proc_macro2::{Ident, Span, TokenStream};
use quote::TokenStreamExt;
use std::path::PathBuf;

///
/// create a new generator for java bridge files.
///
pub(crate) fn new_gen<'a>(
    out_dir: &'a PathBuf,
    trait_descs: &'a Vec<TraitDesc>,
    struct_descs: &'a Vec<StructDesc>,
    imp_desc: &'a Vec<ImpDesc>,
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
    fn gen_sdk_file(&self, mod_names: &Vec<String>) -> Result<TokenStream> {
        let mod_idents = mod_names
            .iter()
            .map(|name| Ident::new(&name, Span::call_site()))
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
            pub extern "C" fn JNI_OnLoad(jvm: JavaVM<'static>, _reserved: *mut c_void) -> jint {
                set_java_vm(jvm);
                JNI_VERSION_1_6
            }

            pub fn set_java_vm(jvm: JavaVM<'static>) {
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
            use android_logger::Filter;
            use std::sync::Arc;
        })
    }

    fn quote_common_part(&self, trait_desc: &Vec<TraitDesc>) -> Result<TokenStream> {
        let class_names = trait_desc
            .iter()
            .map(|desc| format!("{}.{}", self.java_namespace, &desc.name).replace(".", "/"))
            .collect::<Vec<String>>();

        Ok(quote! {
            lazy_static! {
                static ref JVM : Arc<RwLock<Option<JavaVM<'static>>>> = Arc::new(RwLock::new(None));
            }

            pub fn set_global_vm(jvm: JavaVM<'static>) {
                #(let _ = jvm.get_env().unwrap().find_class(#class_names);)*
                *(JVM.write().unwrap()) = Some(jvm);
            }
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
            .map(|field| Ident::new(&field.origin_ty, Span::call_site()))
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
        _callbacks: &Vec<&TraitDesc>,
        _structs: &Vec<StructDesc>,
    ) -> Result<TokenStream> {
        let namespace = self.java_namespace.replace(".", "_");
        let method_name = format!(
            "Java_{}_{}_native_1{}",
            &namespace,
            trait_desc.name,
            &method.name.replace("_", "_1")
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

        let method_sig = if arg_names.len() <= 0 {
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

        Ok(method_sig)
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
        let _class_name =
            format!("{}.{}", &self.java_namespace, &trait_desc.name).replace(".", "/");

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
                quote! {
                    let #rust_arg_name: String = env.get_string(#arg_name_ident).expect("Couldn't get java string!").into();
                }
            }
            AstType::Vec(base) => match base {
                AstBaseType::Byte => {
                    if arg.origin_ty.contains("i8") {
                        let tmp_arg_name =
                            Ident::new(&format!("tmp_{}", &arg.name), Span::call_site());
                        let tmp_arg_ptr =
                            Ident::new(&format!("tmp_{}_ptr", &arg.name), Span::call_site());
                        let tmp_arg_len =
                            Ident::new(&format!("tmp_{}_len", &arg.name), Span::call_site());
                        let tmp_arg_cap =
                            Ident::new(&format!("tmp_{}_cap", &arg.name), Span::call_site());
                        quote! {
                            let mut #tmp_arg_name = env.convert_byte_array(#arg_name_ident).unwrap();
                            let #tmp_arg_ptr = #tmp_arg_name.as_mut_ptr();
                            let #tmp_arg_len = #tmp_arg_name.len();
                            let #tmp_arg_cap = #tmp_arg_name.capacity();
                            let #rust_arg_name = unsafe {
                                std::mem::forget(#tmp_arg_name);
                                Vec::from_raw_parts(#tmp_arg_ptr as (* mut i8), #tmp_arg_len, #tmp_arg_cap)
                            };
                        }
                    } else {
                        quote! {
                            let #rust_arg_name = env.convert_byte_array(#arg_name_ident).unwrap();
                        }
                    }
                }
                _ => {
                    let json_arg_ident =
                        Ident::new(&format!("json_{}", &arg.name), Span::call_site());
                    quote! {
                        let #json_arg_ident: String = env.get_string(#arg_name_ident).expect("Couldn't get java string!").into();
                        let #rust_arg_name = serde_json::from_str(&#json_arg_ident).unwrap();
                    }
                }
            },
            AstType::Callback => self
                .java_callback_strategy
                .arg_convert(arg, trait_desc, callbacks),
            _ => {
                return Err(
                    GenerateError(format!("find unsupported type in arg, {:?}", &arg.ty)).into(),
                );
            }
        })
    }

    fn quote_return_convert(
        &self,
        return_ty: &AstType,
        ret_name: &str,
        origin_ty: &str,
    ) -> Result<TokenStream> {
        let ret_name_ident = Ident::new(ret_name, Span::call_site());

        Ok(match *return_ty {
            AstType::Void => quote!(),
            AstType::Boolean => quote! {
                if #ret_name_ident {1} else {0}
            },
            AstType::String => quote! {
                env.new_string(#ret_name_ident).expect("Couldn't create java string").into_inner()
            },
            AstType::Vec(ref base_ty) => match base_ty {
                AstBaseType::Struct => {
                    let struct_name = origin_ty.to_owned().replace("Vec<", "").replace(">", "");
                    let struct_ident =
                        Ident::new(&format!("Struct_{}", &struct_name), Span::call_site());
                    quote! {
                        let ret_value = ret_value.into_iter().map(|each| #struct_ident::from(each)).collect::<Vec<#struct_ident>>();
                        let json_ret = serde_json::to_string(&ret_value);
                        env.new_string(json_ret.unwrap()).expect("Couldn't create java string").into_inner()
                    }
                }
                AstBaseType::Byte => {
                    if origin_ty.contains("i8") {
                        quote! {
                            let ret_value_ptr = ret_value.as_mut_ptr();
                            let ret_value_len = ret_value.len();
                            let ret_value_cap = ret_value.capacity();
                            let tmp_ret_name = unsafe {
                                std::mem::forget(ret_value);
                                Vec::from_raw_parts(ret_value_ptr as (* mut u8), ret_value_len, ret_value_cap)
                            };
                            env.byte_array_from_slice(&tmp_ret_name).unwrap()
                        }
                    } else {
                        quote! {
                            env.byte_array_from_slice(&ret_value).unwrap()
                        }
                    }
                }
                _ => {
                    quote! {
                        let json_ret = serde_json::to_string(&ret_value);
                        env.new_string(json_ret.unwrap()).expect("Couldn't create java string").into_inner()
                    }
                }
            },
            AstType::Struct => {
                let struct_copy_name =
                    Ident::new(&format!("Struct_{}", origin_ty), Span::call_site());
                quote! {
                    let json_ret = serde_json::to_string(&#struct_copy_name::from(ret_value));
                    env.new_string(json_ret.unwrap()).expect("Couldn't create java string").into_inner()
                }
            }
            _ => {
                let ty_ident = self
                    .ty_to_tokens(&return_ty, TypeDirection::Return)
                    .unwrap();
                quote! {
                    #ret_name_ident as #ty_ident
                }
            }
        })
    }

    fn ty_to_tokens(&self, ast_type: &AstType, direction: TypeDirection) -> Result<TokenStream> {
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
            AstType::Vec(base) => match direction {
                TypeDirection::Argument => match base {
                    AstBaseType::Byte => tokens.append(Ident::new("jbyteArray", Span::call_site())),
                    _ => tokens.append(Ident::new("JString", Span::call_site())),
                },
                TypeDirection::Return => match base {
                    AstBaseType::Byte => tokens.append(Ident::new("jbyteArray", Span::call_site())),
                    _ => tokens.append(Ident::new("jstring", Span::call_site())),
                },
            },
            AstType::Struct => match direction {
                TypeDirection::Argument => tokens.append(Ident::new("JString", Span::call_site())),
                TypeDirection::Return => tokens.append(Ident::new("jstring", Span::call_site())),
            },
            AstType::Callback => tokens.append(Ident::new("i64", Span::call_site())),
            AstType::Void => (),
        };

        Ok(tokens)
    }
}
