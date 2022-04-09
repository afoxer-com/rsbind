use crate::ast::contract::desc::*;
use crate::ast::imp::desc::*;
use crate::ast::types::*;
use crate::base::{Convertible, Direction};
use crate::bridge::file::*;
use crate::errors::*;
use crate::ident;
use crate::swift::converter::SwiftConvert;
use crate::swift::mapping::RustMapping;
use proc_macro2::{Ident, TokenStream};
use std::path::Path;

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
        strategy: CFileGenStrategy {},
    }
}

///
/// c bridge file generate strategy.
///
pub struct CFileGenStrategy {}

impl CFileGenStrategy {}

impl CFileGenStrategy {
    fn quote_free_rust_array(fn_name: String, ty: TokenStream) -> TokenStream {
        let fn_name_ident = ident!(&fn_name);
        quote! {
            #[no_mangle]
            pub extern "C" fn #fn_name_ident(ptr: *mut #ty, length: i32) {
                let catch_result = catch_unwind(AssertUnwindSafe(|| {
                    let len: usize = length as usize;
                    unsafe { Vec::from_raw_parts(ptr, len, len); }
                }));
                match catch_result {
                    Ok(_) => {}
                    Err(e) => { println!("catch_unwind of `rsbind free_rust` error: {:?}", e); }
                };
            }
        }
    }
}

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
            use std::sync::RwLock;
            use std::sync::Arc;
            use std::panic::*;
        })
    }

    fn quote_common_part(&self, _traits: &[TraitDesc]) -> Result<TokenStream> {
        let int8_free_fn =
            CFileGenStrategy::quote_free_rust_array("free_i8_array".to_string(), quote! {i8});
        let int16_free_fn =
            CFileGenStrategy::quote_free_rust_array("free_i16_array".to_string(), quote! {i16});
        let int32_free_fn =
            CFileGenStrategy::quote_free_rust_array("free_i32_array".to_string(), quote! {i32});
        let int64_free_fn =
            CFileGenStrategy::quote_free_rust_array("free_i64_array".to_string(), quote! {i64});

        Ok(quote! {
            lazy_static! {
                static ref CALLBACK_HASHMAP: Arc<RwLock<HashMap<i64, CallbackEnum>>> =  Arc::new(RwLock::new(HashMap::new()));
                static ref CALLBACK_INDEX : Arc<RwLock<i64>> = Arc::new(RwLock::new(0));
            }

            #[repr(C)]
            #[derive(Clone)]
            pub struct CInt8Array {
                pub ptr: * const i8,
                pub len: i32,
                pub free_ptr: extern "C" fn(*mut i8, i32),
            }

            #[repr(C)]
            #[derive(Clone)]
            pub struct CInt16Array {
                pub ptr: * const i16,
                pub len: i32,
                pub free_ptr: extern "C" fn(*mut i16, i32),
            }

            #[repr(C)]
            #[derive(Clone)]
            pub struct CInt32Array {
                pub ptr: * const i32,
                pub len: i32,
                pub free_ptr: extern "C" fn(*mut i32, i32),
            }

            #[repr(C)]
            #[derive(Clone)]
            pub struct CInt64Array {
                pub ptr: * const i64,
                pub len: i32,
                pub free_ptr: extern "C" fn(*mut i64, i32),
            }

            #int8_free_fn
            #int16_free_fn
            #int32_free_fn
            #int64_free_fn

            #[no_mangle]
            pub extern "C" fn free_str(ptr: *mut i8, length: i32) {
                let catch_result = catch_unwind(AssertUnwindSafe(|| unsafe {
                    let slice = std::slice::from_raw_parts_mut(ptr as (*mut u8), length as usize);
                    let cstr = CStr::from_bytes_with_nul_unchecked(slice);
                    CString::from(cstr);
                }));
                match catch_result {
                    Ok(_) => {}
                    Err(e) => {
                        println!("catch_unwind of `rsbind free_str` error: {:?}", e);
                    }
                };
            }
        })
    }

    fn quote_for_all_cb(&self, callbacks: &[&TraitDesc]) -> Result<TokenStream> {
        let enum_items = callbacks
            .iter()
            .map(|item| ident!(&item.name))
            .collect::<Vec<Ident>>();
        let enums = quote! {
            enum CallbackEnum {
                #(#enum_items(Box<dyn #enum_items>)),*
            }
        };

        let mut return_cb_fns = TokenStream::new();
        let mut arg_cb_fns = TokenStream::new();
        for callback in callbacks.iter() {
            let box_to_model_convert_tokens =
                box_to_model_convert(callback, callbacks, "r_result")?;
            let callback_model_str = &format!("{}_{}_Model", &callback.mod_name, callback.name);
            let callback_model_ident = ident!(callback_model_str);
            let callback_ident = ident!(&callback.name);
            let box_to_model_fn_name = ident!(&format!("box_to_model_{}", callback.name));
            return_cb_fns = quote! {
                #return_cb_fns

                fn #box_to_model_fn_name(callback_index: i64) ->  #callback_model_ident {
                    #box_to_model_convert_tokens
                }
            };

            let model_to_box_convert_tokens = model_to_box_convert(callback, callbacks)?;
            let model_to_box_fn_name = ident!(&format!("model_to_box_{}", callback.name));
            arg_cb_fns = quote! {
                #arg_cb_fns

                fn #model_to_box_fn_name(callback_model: #callback_model_ident) -> Box<dyn #callback_ident> {
                    #model_to_box_convert_tokens
                }
            };
        }

        Ok(quote! {
            #enums

            #return_cb_fns

            #arg_cb_fns
        })
    }

    fn quote_callback_structures(
        &self,
        callback: &TraitDesc,
        callbacks: &[&TraitDesc],
    ) -> Result<TokenStream> {
        let callback_str = &format!("{}_{}_Model", &callback.mod_name, &callback.name);
        let callback_struct =
            crate::swift::bridge_s2r::quote_callback_struct(callback, callbacks, callback_str)?;
        Ok(quote! {
            #[repr(C)]
            #callback_struct
        })
    }

    fn quote_for_structures(&self, struct_desc: &StructDesc) -> Result<TokenStream> {
        let proxy_struct_str = format!("Proxy{}", &struct_desc.name);
        let proxy_struct_name = ident!(&proxy_struct_str);
        let origin_struct_name = ident!(&struct_desc.name);
        let names = struct_desc
            .fields
            .iter()
            .map(|field| ident!(&field.name))
            .collect::<Vec<Ident>>();
        let tys = struct_desc
            .fields
            .iter()
            .map(|field| RustMapping::map_base_transfer_type(&field.ty))
            .collect::<Vec<TokenStream>>();

        let struct_array_str = format!("C{}Array", &struct_desc.name);
        let struct_array_name = ident!(&struct_array_str);

        fn origin_to_proxy_convert(field: &ArgDesc) -> TokenStream {
            let field_name = ident!(&field.name);
            let convert = SwiftConvert {
                ty: field.ty.clone(),
            }
            .rust_to_transfer(quote! {origin.#field_name}, Direction::Invoke);
            quote! {
                #field_name : #convert
            }
        }

        fn proxy_to_origin_convert(field: &ArgDesc) -> TokenStream {
            let field_name = ident!(&field.name);
            let convert = SwiftConvert {
                ty: field.ty.clone(),
            }
            .transfer_to_rust(quote! {proxy.#field_name}, Direction::Invoke);
            quote! {
                #field_name : #convert
            }
        }

        let origin_to_proxy_convert_tokens = struct_desc
            .fields
            .iter()
            .map(origin_to_proxy_convert)
            .collect::<Vec<TokenStream>>();

        let proxy_to_origin_convert_tokens = struct_desc
            .fields
            .iter()
            .map(proxy_to_origin_convert)
            .collect::<Vec<TokenStream>>();

        let free_proxy_struct_array_fn = ident!(&format!("free_{}", &struct_array_str));
        let free_proxy_struct_fn = ident!(&format!("free_{}", &proxy_struct_str));

        Ok(quote! {
            #[repr(C)]
            #[derive(Clone)]
            pub struct #proxy_struct_name {
                #(pub #names: #tys),*
            }

            impl From<#origin_struct_name> for #proxy_struct_name {
                fn from(origin: #origin_struct_name) -> Self {
                    #proxy_struct_name{
                        #(#origin_to_proxy_convert_tokens),*
                    }
                }
            }

            impl From<#proxy_struct_name> for #origin_struct_name {
                fn from(proxy: #proxy_struct_name) -> Self {
                    #origin_struct_name{
                        #(#proxy_to_origin_convert_tokens),*
                    }
                }
            }
            #[no_mangle]
            pub extern "C" fn #free_proxy_struct_fn(proxy: #proxy_struct_name) {
                let catch_result = catch_unwind(AssertUnwindSafe(|| {
                    #origin_struct_name::from(proxy);
                }));
                match catch_result {
                    Ok(_) => {}
                    Err(e) => {
                        println!("catch_unwind of `rsbind free proxy struct` error: {:?}", e);
                    }
                };
            }

            #[repr(C)]
            pub struct #struct_array_name {
                pub ptr: *const #proxy_struct_name,
                pub len: i32,
                pub free_ptr: extern "C" fn(*mut #proxy_struct_name, i32),
            }

            #[no_mangle]
            pub extern "C" fn #free_proxy_struct_array_fn(ptr: *mut #proxy_struct_name, len: i32) {
                let catch_result = catch_unwind(AssertUnwindSafe(|| {
                    unsafe {
                        // let proxy_vec =
                        Vec::from_raw_parts(
                            ptr as *mut #proxy_struct_name,
                            len as usize,
                            len as usize);
                        // proxy_vec.into_iter().for_each(|each| {#origin_struct_name::from(each);});
                    }
                }));
                match catch_result {
                    Ok(_) => {}
                    Err(e) => {
                        println!("catch_unwind of `rsbind free proxy struct` error: {:?}", e);
                    }
                };
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
        let fun_name = ident!(&format!(
            "{}_{}_{}",
            &trait_desc.mod_name, trait_desc.name, &method.name
        ));

        let arg_names = method
            .args
            .iter()
            .filter(|arg| !matches!(arg.ty, AstType::Void))
            .map(|arg| ident!(&arg.name))
            .collect::<Vec<Ident>>();

        let arg_types = method
            .args
            .iter()
            .filter(|arg| !matches!(arg.ty, AstType::Void))
            .map(|arg| RustMapping::map_transfer_type(&arg.ty, callbacks))
            .collect::<Vec<TokenStream>>();

        let ret_ty_tokens = RustMapping::map_transfer_type(&method.return_type, callbacks);
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
        _trait_desc: &TraitDesc,
        arg: &ArgDesc,
        callbacks: &[&TraitDesc],
    ) -> Result<TokenStream> {
        crate::swift::bridge_s2r::quote_arg_convert(arg, callbacks)
    }

    fn quote_return_convert(
        &self,
        _trait_desc: &TraitDesc,
        callbacks: &[&TraitDesc],
        return_ty: &AstType,
        ret_name: &str,
    ) -> Result<TokenStream> {
        let obtain_index = if let AstType::Callback(_) = return_ty.clone() {
            quote! {
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
            }
        } else {
            quote! {}
        };

        let return_convert =
            crate::swift::bridge_s2r::quote_return_convert(return_ty, callbacks, ret_name)?;

        let insert_callback = if let AstType::Callback(ref origin) = return_ty.clone() {
            let callback_ident = ident!(origin);
            quote! {
                (*CALLBACK_HASHMAP.write().unwrap()).insert(callback_index, CallbackEnum::#callback_ident(result));
            }
        } else {
            quote! {}
        };

        Ok(quote! {
            #obtain_index
            #return_convert
            #insert_callback
            r_result
        })
    }

    fn ty_to_tokens(&self, _ast_type: &AstType, _direction: TypeDirection) -> Result<TokenStream> {
        // We don't use it.
        Ok(quote!())
    }
}

fn model_to_box_convert(
    callback_desc: &TraitDesc,
    callbacks: &[&TraitDesc],
) -> Result<TokenStream> {
    let struct_name = &format!("{}_struct", &callback_desc.name);
    let struct_ident = ident!(struct_name);

    let mut method_names = Vec::new();
    let mut callback_methods = TokenStream::new();
    let mut callback_struct = TokenStream::new();
    for method in callback_desc.methods.iter() {
        println!(
            "quote method {} in callback {}",
            method.name, callback_desc.name
        );

        // arguments converting in callback
        let mut args_convert = TokenStream::new();
        for cb_arg in method.args.iter() {
            let origin_cb_arg_name = ident!(&cb_arg.name);
            let obtain_index = if let AstType::Callback(ref origin) = cb_arg.ty.clone() {
                let callback_ident = ident!(origin);
                quote! {
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

                    {
                        (*CALLBACK_HASHMAP.write().unwrap()).insert(callback_index, CallbackEnum::#callback_ident(#origin_cb_arg_name));
                    }
                }
            } else {
                quote! {}
            };

            let args_convert_each = crate::swift::bridge_r2s::arg_convert(cb_arg, callbacks)?;
            args_convert = quote! {
                #obtain_index
                #args_convert
                #args_convert_each
            }
        }

        let arg_names = &method
            .args
            .iter()
            .filter(|arg| !matches!(arg.ty, AstType::Void))
            .map(|arg| ident!(&arg.name))
            .collect::<Vec<Ident>>();

        let convert_arg_names = &method
            .args
            .iter()
            .filter(|arg| !matches!(arg.ty, AstType::Void))
            .map(|arg| ident!(&format!("c_{}", &arg.name)))
            .collect::<Vec<Ident>>();

        let mut has_callback_arg = false;
        let arg_types = &method
            .args
            .iter()
            .filter(|arg| !matches!(arg.ty, AstType::Void))
            .map(|arg| match arg.ty.clone() {
                AstType::Vec(vec_inner_name) => {
                    let vec_innder_ident = ident!(&vec_inner_name.origin());
                    quote!(Vec<#vec_innder_ident>)
                }
                AstType::Callback(origin) => {
                    has_callback_arg = true;
                    let origin_ident = ident!(&origin);
                    quote!(Box<dyn #origin_ident>)
                }
                _ => {
                    let ident = ident!(&arg.ty.origin());
                    quote!(#ident)
                }
            })
            .collect::<Vec<TokenStream>>();

        let ret_ty_tokens = match method.return_type {
            AstType::Void => quote!(()),
            AstType::Vec(ref base) => {
                let ident = ident!(&base.origin());
                quote!(Vec<#ident>)
            }
            AstType::Callback(ref origin) => {
                let origin_ident = ident!(origin);
                quote!(Box<dyn #origin_ident>)
            }
            _ => {
                let ident = ident!(&method.return_type.origin());
                quote!(#ident)
            }
        };

        let return_convert = crate::swift::bridge_r2s::return_convert(&method.return_type)
            .expect("Return convert error!");

        // return var ident name
        let return_var_name = match method.return_type {
            AstType::Void => quote!(),
            _ => quote!(r_result),
        };

        // methods calls on impl
        let method_name = ident!(&method.name);
        let fn_method_name = ident!(&format!("fn_{}", method.name));
        let each_method_tokens = quote! {
            fn #method_name(&self, #(#arg_names: #arg_types),*) -> #ret_ty_tokens {
                #args_convert
                let #fn_method_name = self.#method_name;
                let fn_free_ptr = self.free_ptr;
                let result = #fn_method_name(self.index, #(#convert_arg_names),*);
                #return_convert
                #return_var_name
            }
        };

        callback_methods = quote! {
            #callback_methods
            #each_method_tokens
        };

        method_names.push(method_name);
    }

    callback_struct =
        crate::swift::bridge_s2r::quote_callback_struct(callback_desc, callbacks, struct_name)
            .unwrap();

    let callback_ty = ident!(&callback_desc.name);
    let mut method_assign_tokens = TokenStream::new();
    for method_name in method_names.iter() {
        method_assign_tokens = quote! {
            #method_assign_tokens
            #method_name: callback_model.#method_name,
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

        Box::new(#struct_ident {
            #method_assign_tokens
            free_callback: callback_model.free_callback,
            free_ptr: callback_model.free_ptr,
            index: callback_model.index,
        })

    })
}

pub(crate) fn box_to_model_convert(
    callback: &TraitDesc,
    callbacks: &[&TraitDesc],
    ret_name: &str,
) -> Result<TokenStream> {
    let callback_model_str = &format!("{}_{}_Model", &callback.mod_name, &callback.name);
    let callback_model_ident = ident!(callback_model_str);
    let callback_ident = ident!(&callback.name);

    let mut method_names = vec![];
    let mut ret_method_names = vec![];

    let mut method_result = TokenStream::new();
    method_names = callback
        .methods
        .iter()
        .map(|method| ident!(&method.name))
        .collect::<Vec<Ident>>();
    ret_method_names = callback
        .methods
        .iter()
        .map(|method| ident!(&format!("ret_{}", &method.name)))
        .collect::<Vec<Ident>>();

    for method in callback.methods.iter() {
        let method_name = ident!(&method.name);
        let arg_names = &method
            .args
            .iter()
            .filter(|arg| !matches!(arg.ty, AstType::Void))
            .map(|arg| ident!(&arg.name))
            .collect::<Vec<Ident>>();
        let r_arg_names = &method
            .args
            .iter()
            .filter(|arg| !matches!(arg.ty, AstType::Void))
            .map(|arg| ident!(&format!("r_{}", &arg.name)))
            .collect::<Vec<Ident>>();
        let arg_types = &method
            .args
            .iter()
            .filter(|arg| !matches!(arg.ty, AstType::Void))
            .map(|arg| RustMapping::map_transfer_type(&arg.ty, callbacks))
            .collect::<Vec<TokenStream>>();
        let ret_ty_tokens = RustMapping::map_transfer_type(&method.return_type, callbacks);
        let mut args_convert = TokenStream::new();
        for arg in method.args.iter() {
            let each_convert = crate::swift::bridge_s2r::quote_arg_convert(arg, callbacks)?;
            args_convert = quote! {
                #args_convert
                #each_convert
            }
        }
        let return_convert = crate::swift::bridge_s2r::quote_return_convert(
            &method.return_type,
            callbacks,
            "result",
        )?;
        let ret_method_name = ident!(&format!("ret_{}", &method.name));

        if let AstType::Callback(ref origin) = method.return_type.clone() {
            let return_callback_ident = ident!(origin);
            method_result = quote! {
                #method_result

                pub extern "C" fn #ret_method_name(index: i64, #(#arg_names: #arg_types),*) -> #ret_ty_tokens {
                    #args_convert
                    let mut callback_index = 0;
                    let mut result: Option<Box<dyn #return_callback_ident >> = None;
                    let final_result = {
                        let callback_hashmap = &*CALLBACK_HASHMAP.read().unwrap();
                        let ret_callback = callback_hashmap.get(&index);
                        match ret_callback {
                            Some(ret_callback) => {
                                if let CallbackEnum::#callback_ident(ret_callback) = ret_callback {
                                    result = Some(ret_callback.#method_name(#(#r_arg_names),*));
                                    callback_index = {
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
                                    #return_convert
                                    r_result
                                } else {
                                    panic!("Callback doesn't match for index: {}", index);
                                }
                            }
                            None => {
                                panic!("No callback found for index: {}", index);
                            }
                        }
                    };

                    (*CALLBACK_HASHMAP.write().unwrap()).insert(callback_index, CallbackEnum::#return_callback_ident(result.unwrap()));
                    final_result
                }
            };
        } else {
            method_result = quote! {
                #method_result

                pub extern "C" fn #ret_method_name(index: i64, #(#arg_names: #arg_types),*) -> #ret_ty_tokens {
                    #args_convert
                    let callback_hashmap = &*CALLBACK_HASHMAP.read().unwrap();
                    let ret_callback = callback_hashmap.get(&index);
                    match ret_callback {
                        Some(ret_callback) => {
                            if let CallbackEnum::#callback_ident(ret_callback) = ret_callback {
                                let mut result = ret_callback.#method_name(#(#r_arg_names),*);
                                #return_convert
                                r_result
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
    }

    let free_fn_ident = ident!(&format!("{}_free_rust", &callback.crate_name));
    method_result = quote! {
        #method_result

        pub extern "C" fn ret_free_callback(index: i64) {
            (*CALLBACK_HASHMAP.write().unwrap()).remove(&index);
        }

        pub extern "C" fn ret_free_ptr(buffer: *mut i8, size: i32) {
            #free_fn_ident(buffer, size as u32)
        }
    };

    Ok(quote! {
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
