use rstgen::swift::Swift;
use rstgen::Tokens;
use syn::__private::bool;

use crate::ast::contract::desc::{ArgDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::base::Convertible;
use crate::errors::*;
use crate::swift::mapping::{RustMapping, SwiftMapping};
use crate::swift::types::SwiftType;

///
/// Swift to C data convert.
///
pub(crate) fn fill_arg_convert<'a, 'b>(
    method_body: &'b mut Tokens<'a, Swift<'a>>,
    arg: &'a ArgDesc,
    callbacks: &'a [&'a TraitDesc],
) -> Result<()> {
    println!("quote arg convert for {}", arg.name.clone());
    push_f!(method_body, "let s_{} = ", arg.name);
    method_body.append(arg.ty.swift_to_transfer(arg.name.clone()));

    Ok(())
}

pub(crate) fn fill_return_type_convert<'a, 'b>(
    method_body: &'b mut Tokens<'a, Swift<'a>>,
    return_type: &'a AstType,
    crate_name: &str,
    callbacks: &'a [&'a TraitDesc],
) -> Result<()> {
    let crate_name = crate_name.replace('-', "_");
    push_f!(method_body, "let r_result = ");
    method_body.append(return_type.transfer_to_swift("result".to_string()));
    push!(method_body, "return r_result");
    Ok(())
}
