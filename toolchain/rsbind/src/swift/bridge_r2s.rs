///!
///! Rust to Swift data convert.
///!
use proc_macro2::TokenStream;

use crate::ast::contract::desc::{ArgDesc, TraitDesc};
use crate::ast::types::AstType;
use crate::base::{Convertible, Direction};
use crate::errors::*;
use crate::ident;
use crate::swift::converter::SwiftConvert;

pub(crate) fn arg_convert(arg: &ArgDesc, _callbacks: &[&TraitDesc]) -> Result<TokenStream> {
    let cb_arg_name = ident!(&format!("c_{}", arg.name));
    let cb_origin_arg_name = ident!(&arg.name);

    let convert = SwiftConvert { ty: arg.ty.clone() }
        .rust_to_transferable(quote! {#cb_origin_arg_name}, Direction::Up);
    let convert = quote! {
        let #cb_arg_name = #convert;
    };

    Ok(match arg.ty.clone() {
        AstType::String | AstType::Vec(_) => {
            let ptr_arg = ident!(&format!("ptr_{}", &arg.name));
            quote! {
                #convert
                let #ptr_arg = #cb_arg_name.ptr;
            }
        }
        _ => convert,
    })
}

pub(crate) fn return_convert(return_type: &AstType) -> Result<TokenStream> {
    let convert = SwiftConvert {
        ty: return_type.clone(),
    }
    .transferable_to_rust(quote! {result}, Direction::Up);
    let convert = quote! {
        let r_result = #convert;
    };

    Ok(convert)
}
