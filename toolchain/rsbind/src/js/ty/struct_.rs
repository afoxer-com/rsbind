use crate::ast::contract::desc::StructDesc;
use crate::ast::types::{AstBaseType, AstType};
use crate::ast::AstResult;
use crate::base::lang::{Convertible, Direction};
use crate::js::ty::boolean::Boolean;
use crate::js::ty::converter::JsConverter;
use crate::js::ty::number::Number;
use crate::js::ty::string::Str;
use proc_macro2::{Ident, Literal, TokenStream};
use rstgen::{js, JavaScript, Tokens};
use std::collections::HashMap;

pub(crate) struct Struct {
    pub(crate) ty: AstType,
    pub(crate) ast: AstResult,
}

impl Struct {
    fn find_desc(&self) -> &StructDesc {
        match self.ty {
            AstType::Struct(ref custom_type) => {
                let list = self.ast.structs.get(&custom_type.mod_name).expect(&format!(
                    "Can't find struct desc for {}",
                    &custom_type.mod_name
                ));
                list.iter()
                    .find(|s| s.name == custom_type.origin)
                    .expect(&format!(
                        "Can't find struct desc for origin: {}",
                        &custom_type.origin
                    ))
            }
            _ => panic!("Wrong type in struct"),
        }
    }
}

impl<'a> Convertible<JavaScript<'static>> for Struct {
    fn native_to_transferable(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, JavaScript<'static>> {
        let mut tokens = Tokens::new();
        let desc = self.find_desc();
        nested!(tokens, "{");
        for (index, field) in desc.fields.iter().enumerate() {
            let native_to_transfer = JsConverter {
                ty: field.ty.clone(),
                ast: self.ast.clone(),
            }
            .native_to_transferable(format!("{}.{}", &origin, &field.name), Direction::Down);
            nested_f!(tokens, "{}: {},", field.name, native_to_transfer);
        }
        nested!(tokens, "}");

        tokens
    }

    fn transferable_to_native(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, JavaScript<'static>> {
        let mut tokens = Tokens::new();
        let desc = self.find_desc();
        nested!(tokens, "{");
        for (index, field) in desc.fields.iter().enumerate() {
            let transfer_to_native = JsConverter {
                ty: field.ty.clone(),
                ast: self.ast.clone(),
            }
            .transferable_to_native(format!("{}.{}", &origin, &field.name), Direction::Down);
            nested_f!(tokens, "{}: {},", field.name, transfer_to_native);
        }
        nested!(tokens, "}");

        tokens
    }

    fn rust_to_transferable(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        let proxy_ident = ident!(&format!("_{}", self.ty.origin()));
        let desc = self.find_desc();
        let keys_convert = desc
            .fields
            .iter()
            .map(|f| {
                let field_name = Literal::string(&f.name);
                Str { ty: f.ty.clone() }.rust_to_transferable(quote! {#field_name}, Direction::Down)
            })
            .collect::<Vec<TokenStream>>();

        let value_convert = desc
            .fields
            .iter()
            .map(|field| {
                let field_origin = ident!(&field.name);
                JsConverter {
                    ty: field.ty.clone(),
                    ast: self.ast.clone(),
                }
                .rust_to_transferable(quote! {#origin.#field_origin}, Direction::Down)
            })
            .collect::<Vec<TokenStream>>();

        quote! {{
            let mut j_result = ptr::null_mut();
            let ok = unsafe {napi_sys::napi_create_object(env, &mut j_result)};
            if ok == 0 {
                #(
                    let value = #value_convert;
                    let key = #keys_convert;
                    let ok = unsafe {napi_sys::napi_set_property(env, j_result, key, value)};
                    if ok != 0 {
                        panic!("failed on setting property for {:?}, code = {}", key, ok)
                    }
                )*
                j_result
            } else {
                panic!("Can't create object for struct. code = {}", ok);
            }
        }}
    }

    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        let obj_ident = ident!(&self.ty.origin());
        let desc = self.find_desc();
        let keys = desc
            .fields
            .iter()
            .map(|field| ident!(&field.name))
            .collect::<Vec<Ident>>();
        let keys_convert = desc
            .fields
            .iter()
            .map(|f| {
                let field_name = Literal::string(&f.name);
                Str { ty: f.ty.clone() }.rust_to_transferable(quote! {#field_name}, Direction::Down)
            })
            .collect::<Vec<TokenStream>>();

        let value_convert = desc
            .fields
            .iter()
            .map(|field| {
                let origin = ident!(&field.name);
                JsConverter {
                    ty: field.ty.clone(),
                    ast: self.ast.clone(),
                }
                .transferable_to_rust(quote! {value}, Direction::Down)
            })
            .collect::<Vec<TokenStream>>();

        let converted_ident = desc
            .fields
            .iter()
            .map(|field| ident!(&format!("{}_converted", &field.name)))
            .collect::<Vec<Ident>>();

        quote! {{
            #(
                let mut value = ptr::null_mut();
                let key = #keys_convert;
                let ok = unsafe {sys::napi_get_property(env, #origin, key, &mut value)};
                let #converted_ident = if ok != 0 {
                    panic!("failed on get property for {:?}, code = {}", key, ok)
                } else {
                    #value_convert
                };
            )*

            #obj_ident {
                #(
                  #keys: #converted_ident
                ),*
            }
        }}
    }

    fn native_type(&self) -> JavaScript<'static> {
        js::local(self.ty.origin())
    }

    fn native_transferable_type(&self, direction: Direction) -> JavaScript<'static> {
        js::local(format!("_{}", self.ty.origin()))
    }

    fn rust_transferable_type(&self, direction: Direction) -> TokenStream {
        quote! {napi_sys::napi_value}
    }

    fn quote_common_in_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_in_native(&self) -> Tokens<'static, JavaScript<'static>> {
        toks!()
    }

    fn quote_in_common_rs(&self) -> TokenStream {
        quote! {}
    }
}
