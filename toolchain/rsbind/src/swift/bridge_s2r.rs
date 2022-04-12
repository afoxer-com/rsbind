///!
///! Swift to Rust data convert.
///!
///
use proc_macro2::TokenStream;

use crate::ast::contract::desc::{ArgDesc, TraitDesc};
use crate::ast::types::AstType;
use crate::base::{Convertible, Direction};
use crate::errors::*;
use crate::ident;
use crate::swift::converter::SwiftConvert;
use crate::swift::mapping::RustMapping;

pub(crate) fn quote_arg_convert(arg: &ArgDesc, _callbacks: &[&TraitDesc]) -> Result<TokenStream> {
    let rust_arg_name = ident!(&format!("r_{}", &arg.name));
    let arg_name_ident = ident!(&arg.name);

    let convert = SwiftConvert { ty: arg.ty.clone() }
        .transferable_to_rust(quote! {#arg_name_ident}, Direction::Down);
    let convert = quote! {
        let #rust_arg_name = #convert;
    };

    Ok(convert)
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
        let ret_ty_tokens = RustMapping::map_transfer_type(&method.return_type, callbacks);
        let arg_types = method
            .args
            .iter()
            .filter(|arg| !matches!(arg.ty, AstType::Void))
            .map(|arg| RustMapping::map_transfer_type(&arg.ty, callbacks))
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
            pub free_ptr: extern "C" fn(* mut i8, i32),
            pub index: i64,

        }
    };

    Ok(callback_struct)
}

pub(crate) fn quote_return_convert(
    ty: &AstType,
    _callbacks: &[&TraitDesc],
    ret_name: &str,
) -> Result<TokenStream> {
    let ret_name_ident = ident!(ret_name);

    let convert = SwiftConvert { ty: ty.clone() }
        .rust_to_transferable(quote! {#ret_name_ident}, Direction::Down);
    let convert = quote! {
        let r_result = #convert;
    };

    Ok(convert)
}
