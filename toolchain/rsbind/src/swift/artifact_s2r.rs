use crate::ast::contract::desc::{ArgDesc, TraitDesc};
use crate::ast::types::AstType;
use crate::base::{Convertible, Direction};
use crate::errors::*;
use crate::swift::converter::SwiftConvert;
use rstgen::swift::Swift;
use rstgen::Tokens;

///
/// Swift to C data convert.
///
pub(crate) fn fill_arg_convert<'a, 'b>(
    method_body: &'b mut Tokens<'a, Swift<'a>>,
    arg: &'a ArgDesc,
    _callbacks: &'a [&'a TraitDesc],
) -> Result<()> {
    println!("quote arg convert for {}", arg.name.clone());
    push_f!(method_body, "let s_{} = ", arg.name);
    method_body.append(
        SwiftConvert { ty: arg.ty.clone() }
            .artifact_to_transfer(arg.name.clone(), Direction::Invoke),
    );

    Ok(())
}

pub(crate) fn fill_return_type_convert<'a, 'b>(
    method_body: &'b mut Tokens<'a, Swift<'a>>,
    return_type: &'a AstType,
    crate_name: &str,
    _callbacks: &'a [&'a TraitDesc],
) -> Result<()> {
    push_f!(method_body, "let r_result = ");
    method_body.append(
        SwiftConvert {
            ty: return_type.clone(),
        }
        .transfer_to_artifact("result".to_string(), Direction::Invoke),
    );
    push!(method_body, "return r_result");
    Ok(())
}
