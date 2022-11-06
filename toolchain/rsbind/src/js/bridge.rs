use crate::ast::types::AstType;
use crate::base::lang::{
    BridgeContext, CallbackContext, Convertible, Direction, LangImp, MethodContext, ModContext,
    StructContext,
};
use crate::js::ty::converter::JsConverter;
use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Literal, TokenStream};
use rstgen::JavaScript;
use syn::Lit;

pub struct JsImp {}

impl LangImp<JavaScript<'static>, ()> for JsImp {
    fn quote_lib_file(
        &self,
        context: &BridgeContext<JavaScript<'static>, ()>,
    ) -> crate::Result<TokenStream> {
        let host_crate_underscore = ident!(&context.crate_name.replace("-", "_"));
        Ok(quote! {
            extern crate #host_crate_underscore;
            extern crate napi_sys;
        })
    }

    fn quote_common_file(
        &self,
        context: &BridgeContext<JavaScript<'static>, ()>,
    ) -> crate::Result<TokenStream> {
        Ok(quote! {})
    }

    fn quote_use_part(
        &self,
        context: &ModContext<JavaScript<'static>, ()>,
    ) -> crate::Result<TokenStream> {
        Ok(quote! {
            use napi_sys as sys;
            use std::ptr;
        })
    }

    fn quote_common_part(
        &self,
        context: &ModContext<JavaScript<'static>, ()>,
    ) -> crate::Result<TokenStream> {
        let mut rust_methods: Vec<Ident> = Vec::new();
        let mut js_methods: Vec<Literal> = Vec::new();

        for each_trait in context.traits.iter() {
            for method in each_trait.methods.iter() {
                let method_name = format!(
                    "{}_{}",
                    each_trait.name.to_snake_case(),
                    method.name.to_snake_case()
                );

                rust_methods.push(ident!(&method_name));
                js_methods.push(Literal::string(&method_name))
            }
        }

        Ok(quote! {
            #[no_mangle]
            unsafe extern "C" fn napi_register_module_v1(env: sys::napi_env, exports: sys::napi_value) -> sys::napi_value {
                println!("napi_register_module_v1 running.");
                #(let mut function = ptr::null_mut();
                let cstring = std::ffi::CString::new(#js_methods).expect("CString::new failed");
                let js_name_bytes = cstring.to_bytes_with_nul();
                let js_name = std::ffi::CStr::from_bytes_with_nul_unchecked(js_name_bytes);
                sys::napi_create_function(
                        env,
                        js_name.as_ptr(),
                        js_name_bytes.len(),
                        Some(#rust_methods),
                        ptr::null_mut(),
                        &mut function,
                );
                sys::napi_set_named_property(env, exports, js_name.as_ptr(), function);)*
                exports
            }
        })
    }

    fn quote_method_sig(
        &self,
        context: &MethodContext<JavaScript<'static>, ()>,
    ) -> crate::Result<TokenStream> {
        let method_name = format!(
            "{}_{}",
            context.service_ctx.trait_.name.to_snake_case(),
            &context.method.name.to_snake_case()
        );

        let method_name_ident = ident!(&method_name);
        Ok(quote! {
            pub extern "C" fn #method_name_ident(env: sys::napi_env, cb_info: sys::napi_callback_info) -> sys::napi_value
        })
    }

    fn quote_for_one_struct(
        &self,
        context: &StructContext<JavaScript<'static>, ()>,
    ) -> crate::Result<TokenStream> {
        Ok(quote! {})
    }

    fn quote_for_one_callback(
        &self,
        context: &CallbackContext<JavaScript<'static>, ()>,
    ) -> crate::Result<TokenStream> {
        Ok(quote! {})
    }

    fn provide_converter(
        &self,
        ty: &AstType,
        context: &BridgeContext<JavaScript<'static>, ()>,
    ) -> Box<dyn Convertible<JavaScript<'static>>> {
        Box::new(JsConverter {
            ty: ty.clone(),
            ast: context.ast.clone(),
        })
    }
}
