use crate::ast::contract::desc::*;
use crate::ast::imp::desc::*;
use crate::ast::types::*;
use crate::base::lang::{Convertible, Direction};
use crate::errors::*;
use crate::swift::converter::SwiftConvert;
use crate::swift::mapping::RustMapping;
use crate::ErrorKind::GenerateError;
use crate::{ident, AstResult};
use proc_macro2::{Ident, TokenStream};
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use std::path::Path;

///
/// Executor for generating core files of bridge mod.
///
pub(crate) struct BridgeFileGen<'a> {
    pub traits: &'a [TraitDesc],
    pub structs: &'a [StructDesc],
    pub imps: &'a [ImpDesc],
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
            use std::ffi::CStr;
            use std::os::raw::c_char;
            use std::ffi::CString;
            use c::common::*;
            use std::collections::HashMap;
            use std::sync::RwLock;
            use std::sync::Arc;
            use std::panic::*;
        })
    }

    fn quote_common_part(&self, _traits: &[TraitDesc]) -> Result<TokenStream> {
        Ok(quote! {
            lazy_static! {
                static ref CALLBACK_HASHMAP: Arc<RwLock<HashMap<i64, CallbackEnum>>> =  Arc::new(RwLock::new(HashMap::new()));
                static ref CALLBACK_INDEX : Arc<RwLock<i64>> = Arc::new(RwLock::new(0));
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
        let callback_struct = quote_callback_struct(callback, callbacks, callback_str)?;
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
            .map(|field| {
                SwiftConvert {
                    ty: field.ty.clone(),
                }
                .rust_transferable_type(Direction::Down)
            })
            .collect::<Vec<TokenStream>>();

        let struct_array_str = format!("C{}Array", &struct_desc.name);
        let struct_array_name = ident!(&struct_array_str);

        fn origin_to_proxy_convert(field: &ArgDesc) -> TokenStream {
            let field_name = ident!(&field.name);
            let convert = SwiftConvert {
                ty: field.ty.clone(),
            }
            .rust_to_transferable(quote! {origin.#field_name}, Direction::Down);
            quote! {
                #field_name : #convert
            }
        }

        fn proxy_to_origin_convert(field: &ArgDesc) -> TokenStream {
            let field_name = ident!(&field.name);
            let convert = SwiftConvert {
                ty: field.ty.clone(),
            }
            .transferable_to_rust(quote! {proxy.#field_name}, Direction::Down);
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
                pub cap: i32,
                pub free_ptr: extern "C" fn(*mut #proxy_struct_name, i32, i32),
            }

            #[no_mangle]
            pub extern "C" fn #free_proxy_struct_array_fn(ptr: *mut #proxy_struct_name, len: i32, cap: i32) {
                let catch_result = catch_unwind(AssertUnwindSafe(|| {
                    unsafe {
                        // let proxy_vec =
                        Vec::from_raw_parts(
                            ptr as *mut #proxy_struct_name,
                            len as usize,
                            cap as usize);
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
            .map(|arg| SwiftConvert { ty: arg.ty.clone() }.rust_transferable_type(Direction::Down))
            .collect::<Vec<TokenStream>>();

        let ret_ty_tokens = SwiftConvert {
            ty: method.return_type.clone(),
        }
        .rust_transferable_type(Direction::Up);
        let sig_define = quote! {
            #[no_mangle]
            pub extern "C" fn #fun_name(#(#arg_names: #arg_types),*) -> #ret_ty_tokens
        };

        Ok(sig_define)
    }

    fn quote_arg_convert(
        &self,
        _trait_desc: &TraitDesc,
        arg: &ArgDesc,
        callbacks: &[&TraitDesc],
    ) -> Result<TokenStream> {
        let rust_arg_name = ident!(&format!("r_{}", &arg.name));
        let arg_name_ident = ident!(&arg.name);

        let convert = SwiftConvert { ty: arg.ty.clone() }
            .transferable_to_rust(quote! {#arg_name_ident}, Direction::Down);
        let convert = quote! {
            let #rust_arg_name = #convert;
        };

        Ok(convert)
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

        let ret_name_ident = ident!(ret_name);
        let convert = SwiftConvert {
            ty: return_ty.clone(),
        }
        .rust_to_transferable(quote! {#ret_name_ident}, Direction::Down);
        let return_convert = quote! {
            let r_result = #convert;
        };

        let insert_callback = if let AstType::Callback(ref origin) = return_ty.clone() {
            let callback_ident = ident!(&origin.origin);
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
}

fn model_to_box_convert(
    callback_desc: &TraitDesc,
    callbacks: &[&TraitDesc],
) -> Result<TokenStream> {
    let struct_name = &format!("{}_struct", &callback_desc.name);
    let struct_ident = ident!(struct_name);

    let mut method_names = Vec::new();
    let mut callback_methods = TokenStream::new();
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
                let callback_ident = ident!(&origin.origin);
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

            let cb_arg_name = ident!(&format!("c_{}", cb_arg.name));
            let cb_origin_arg_name = ident!(&cb_arg.name);

            let convert = SwiftConvert {
                ty: cb_arg.ty.clone(),
            }
            .rust_to_transferable(quote! {#cb_origin_arg_name}, Direction::Up);
            let convert = quote! {
                let #cb_arg_name = #convert;
            };

            let args_convert_each = match cb_arg.ty.clone() {
                AstType::String | AstType::Vec(_) => {
                    let ptr_arg = ident!(&format!("ptr_{}", &cb_arg.name));
                    quote! {
                        #convert
                        let #ptr_arg = #cb_arg_name.ptr;
                    }
                }
                _ => convert,
            };

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
                    let origin_ident = ident!(&origin.origin);
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
                let origin_ident = ident!(&origin.origin);
                quote!(Box<dyn #origin_ident>)
            }
            _ => {
                let ident = ident!(&method.return_type.origin());
                quote!(#ident)
            }
        };

        let convert = SwiftConvert {
            ty: method.return_type.clone(),
        }
        .transferable_to_rust(quote! {result}, Direction::Up);
        let return_convert = quote! {
            let r_result = #convert;
        };

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

    let callback_struct = quote_callback_struct(callback_desc, callbacks, struct_name).unwrap();

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
    _ret_name: &str,
) -> Result<TokenStream> {
    let callback_model_str = &format!("{}_{}_Model", &callback.mod_name, &callback.name);
    let callback_model_ident = ident!(callback_model_str);
    let callback_ident = ident!(&callback.name);

    let mut method_result = TokenStream::new();
    let method_names = callback
        .methods
        .iter()
        .map(|method| ident!(&method.name))
        .collect::<Vec<Ident>>();
    let ret_method_names = callback
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
            .map(|arg| SwiftConvert { ty: arg.ty.clone() }.rust_transferable_type(Direction::Down))
            .collect::<Vec<TokenStream>>();
        let ret_ty_tokens = SwiftConvert {
            ty: method.return_type.clone(),
        }
        .rust_transferable_type(Direction::Up);
        let mut args_convert = TokenStream::new();
        for arg in method.args.iter() {
            let rust_arg_name = ident!(&format!("r_{}", &arg.name));
            let arg_name_ident = ident!(&arg.name);

            let convert = SwiftConvert { ty: arg.ty.clone() }
                .transferable_to_rust(quote! {#arg_name_ident}, Direction::Down);
            let each_convert = quote! {
                let #rust_arg_name = #convert;
            };

            args_convert = quote! {
                #args_convert
                #each_convert
            }
        }

        let convert = SwiftConvert { ty: method.return_type.clone() }
            .rust_to_transferable(quote! {result}, Direction::Down);
        let return_convert = quote! {
            let r_result = #convert;
        };

        let ret_method_name = ident!(&format!("ret_{}", &method.name));

        if let AstType::Callback(ref origin) = method.return_type.clone() {
            let return_callback_ident = ident!(&origin.origin);
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

    let _free_fn_ident = ident!(&format!("{}_free_rust", &callback.crate_name));
    method_result = quote! {
        #method_result

        pub extern "C" fn ret_free_callback(index: i64) {
            (*CALLBACK_HASHMAP.write().unwrap()).remove(&index);
        }

        pub extern "C" fn ret_free_ptr(buffer: *mut i8, len: i32, cap: i32) {
            free_i8_array(buffer, len, cap)
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

pub(crate) fn quote_callback_struct(
    callback: &TraitDesc,
    callbacks: &[&TraitDesc],
    name: &str,
) -> Result<TokenStream> {
    let callback_ident = ident!(name);

    let callback_struct_sig = quote! {
        pub struct #callback_ident
    };

    let mut callback_methods = TokenStream::new();
    for method in callback.methods.iter() {
        let callback_method_ident = ident!(&method.name);
        let ret_ty_tokens = SwiftConvert {
            ty: method.return_type.clone(),
        }
        .rust_transferable_type(Direction::Up);
        let arg_types = method
            .args
            .iter()
            .filter(|arg| !matches!(arg.ty, AstType::Void))
            .map(|arg| SwiftConvert { ty: arg.ty.clone() }.rust_transferable_type(Direction::Down))
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
            pub free_ptr: extern "C" fn(* mut i8, i32, i32),
            pub index: i64,

        }
    };

    Ok(callback_struct)
}

///
/// The executor for generating a bridge mod
///
pub(crate) struct BridgeCodeGen<'a> {
    pub ast: &'a AstResult,
    pub bridge_dir: &'a Path,
    pub crate_name: String,
}

impl<'a> BridgeCodeGen<'a> {
    fn common_file_gen(&self) -> Result<TokenStream> {
        let int8_free_fn = self.quote_free_rust_array("free_i8_array".to_string(), quote! {i8});
        let int16_free_fn = self.quote_free_rust_array("free_i16_array".to_string(), quote! {i16});
        let int32_free_fn = self.quote_free_rust_array("free_i32_array".to_string(), quote! {i32});
        let int64_free_fn = self.quote_free_rust_array("free_i64_array".to_string(), quote! {i64});

        let tokens = quote! {
            use std::panic::*;
            use std::ffi::CString;
            use std::os::raw::c_char;
            use std::ffi::CStr;

            #[repr(C)]
            #[derive(Clone)]
            pub struct CInt8Array {
                pub ptr: * const i8,
                pub len: i32,
                pub cap: i32,
                pub free_ptr: extern "C" fn(*mut i8, i32, i32),
            }

            #[repr(C)]
            #[derive(Clone)]
            pub struct CInt16Array {
                pub ptr: * const i16,
                pub len: i32,
                pub cap: i32,
                pub free_ptr: extern "C" fn(*mut i16, i32, i32),
            }

            #[repr(C)]
            #[derive(Clone)]
            pub struct CInt32Array {
                pub ptr: * const i32,
                pub len: i32,
                pub cap: i32,
                pub free_ptr: extern "C" fn(*mut i32, i32, i32),
            }

            #[repr(C)]
            #[derive(Clone)]
            pub struct CInt64Array {
                pub ptr: * const i64,
                pub len: i32,
                pub cap: i32,
                pub free_ptr: extern "C" fn(*mut i64, i32, i32),
            }

            #int8_free_fn
            #int16_free_fn
            #int32_free_fn
            #int64_free_fn

            #[no_mangle]
            pub extern "C" fn free_str(ptr: *mut i8, length: i32, cap: i32) {
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

        };

        Ok(tokens)
    }

    fn quote_free_rust_array(&self, fn_name: String, ty: TokenStream) -> TokenStream {
        let fn_name_ident = ident!(&fn_name);
        quote! {
            #[no_mangle]
            pub extern "C" fn #fn_name_ident(ptr: *mut #ty, length: i32, cap: i32) {
                let catch_result = catch_unwind(AssertUnwindSafe(|| {
                    let len: usize = length as usize;
                    unsafe { Vec::from_raw_parts(ptr, len as usize, cap as usize); }
                }));
                match catch_result {
                    Ok(_) => {}
                    Err(e) => { println!("catch_unwind of `rsbind free_rust` error: {:?}", e); }
                };
            }
        }
    }
}

impl<'a> BridgeCodeGen<'a> {
    ///
    /// generate the bridge files
    ///
    pub(crate) fn gen_files(&self) -> Result<()> {
        let empty_vec = vec![];

        let traits = &self.ast.traits;
        let structs = &self.ast.structs;
        let imps = &self.ast.imps;

        let mut bridges: Vec<String> = vec![];
        for (mod_name, trait_vec) in traits {
            let struct_vec = structs.get(mod_name).unwrap_or(&empty_vec);

            // generate bridge files.
            let out_mod_name = format!("c_{}", mod_name);
            let out_file_name = format!("{}.rs", &out_mod_name);

            let tokens = BridgeFileGen {
                traits: trait_vec,
                structs: struct_vec,
                imps,
            }
            .gen_one_bridge_file()?;
            self.write(&out_file_name, &tokens)?;
            bridges.push(out_mod_name)
        }

        // generate sdk.rs
        let tokens = quote! {};
        self.write("sdk.rs", &tokens)?;
        bridges.push("sdk".to_owned());

        // generate common.rs
        let tokens = self.common_file_gen()?;
        self.write("common.rs", &tokens)?;

        // generate mod.rs
        let bridge_ident = bridges
            .iter()
            .map(|bridge| ident!(bridge))
            .collect::<Vec<Ident>>();

        let bridge_mod_tokens = quote! {
            # (pub mod #bridge_ident;)*
            pub mod common;
        };
        self.write("mod.rs", &bridge_mod_tokens)?;

        Ok(())
    }

    fn write(&self, file: &str, tokens: &TokenStream) -> Result<()> {
        let file_path = self.bridge_dir.join(file);
        let mut file = File::create(&file_path)?;
        file.write_all(&tokens.to_string().into_bytes())?;
        Ok(())
    }
}
