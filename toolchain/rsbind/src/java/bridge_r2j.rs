use crate::ast::contract::desc::{ArgDesc, MethodDesc};
use crate::ast::types::AstType;
use crate::base::lang::{Convertible, Direction};
use crate::errors::*;
use crate::ident;
use crate::java::converter::JavaConvert;
use proc_macro2::TokenStream;

pub(crate) fn arg_convert(cb_arg: &ArgDesc) -> Result<TokenStream> {
    if let AstType::Void = cb_arg.ty.clone() {
        return Ok(quote! {});
    }

    let cb_arg_name = ident!(&format!("j_{}", cb_arg.name));
    let cb_origin_arg_name = ident!(&cb_arg.name);

    let convert = JavaConvert {
        ty: cb_arg.ty.clone(),
    }
    .rust_to_transferable(quote! {#cb_origin_arg_name}, Direction::Up);
    Ok(quote! {
        let #cb_arg_name = #convert;
    })
}

pub(crate) fn return_convert(method: &MethodDesc) -> Result<TokenStream> {
    if let AstType::Void = method.return_type.clone() {
        return Ok(quote! {});
    }

    let convert = JavaConvert {
        ty: method.return_type.clone(),
    }
    .transferable_to_rust(quote! {result}, Direction::Up);
    Ok(quote! {
        let r_result = #convert;
    })
}
