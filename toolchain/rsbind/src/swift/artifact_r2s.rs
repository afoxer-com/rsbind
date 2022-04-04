use rstgen::swift::Swift;
use rstgen::Tokens;

use crate::ast::contract::desc::{ArgDesc, MethodDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::base::Convertible;
use crate::errors::*;
use crate::swift::mapping::SwiftMapping;
use crate::swift::types::SwiftType;

///
/// C to Swift data convert.
///
pub(crate) fn fill_arg_convert<'a, 'b>(
    cb_arg: &'a ArgDesc,
    trait_desc: &'a TraitDesc,
    method_body: &'b mut Tokens<'a, Swift<'a>>,
) -> Result<()> {
    let mut fn_body = toks!();
    nested_f!(fn_body, "let c_{} = ", cb_arg.name);
    fn_body.append(cb_arg.ty.transfer_to_swift(cb_arg.name.clone()));
    method_body.push(fn_body);
    Ok(())
}

pub(crate) fn fill_return_convert(
    cb_method: &MethodDesc,
    callbacks: &[&TraitDesc],
    method_body: &mut Tokens<Swift>,
) -> Result<()> {
    push!(method_body, "let r_result = ");
    method_body.append(
        cb_method
            .return_type
            .swift_to_transfer("result".to_string()),
    );
    Ok(())
}
