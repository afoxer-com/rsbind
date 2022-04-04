///!
///! Rust to Swift data convert.
///!
use proc_macro2::{Ident, Span, TokenStream};

use crate::ast::contract::desc::{ArgDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::base::Convertible;
use crate::errors::*;
use crate::ident;
use crate::swift::mapping::RustMapping;
use crate::swift::ty::basic::{Basic, Bool};
use crate::swift::ty::str::Str;
use crate::swift::ty::struct_::Struct;
use crate::swift::ty::vec_base::VecBase;
use crate::swift::ty::vec_default::VecDefault;
use crate::swift::ty::vec_struct::VecStruct;

pub(crate) fn arg_convert(arg: &ArgDesc, callbacks: &[&TraitDesc]) -> Result<TokenStream> {
    let cb_arg_name = ident!(&format!("c_{}", arg.name));
    let cb_origin_arg_name = ident!(&arg.name);

    let convert = arg.ty.rust_to_transfer(quote! {#cb_origin_arg_name});
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
    let convert = return_type.transfer_to_rust(quote! {result});
    let convert = quote! {
        let r_result = #convert;
    };

    Ok(convert)
}
