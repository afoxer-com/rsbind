use crate::ast::contract::desc::{ArgDesc, MethodDesc, TraitDesc};
use crate::base::{Convertible, Direction};
use crate::errors::*;
use crate::swift::converter::SwiftConvert;
use rstgen::swift::Swift;
use rstgen::Tokens;

///
/// C to Swift data convert.
///
pub(crate) fn fill_arg_convert<'a, 'b>(
    cb_arg: &'a ArgDesc,
    _trait_desc: &'a TraitDesc,
    method_body: &'b mut Tokens<'a, Swift<'a>>,
) -> Result<()> {
    let mut fn_body = toks!();
    nested_f!(fn_body, "let c_{} = ", cb_arg.name);
    fn_body.append(
        SwiftConvert {
            ty: cb_arg.ty.clone(),
        }
        .transfer_to_artifact(cb_arg.name.clone(), Direction::Push),
    );
    method_body.push(fn_body);
    Ok(())
}

pub(crate) fn fill_return_convert(
    cb_method: &MethodDesc,
    _callbacks: &[&TraitDesc],
    method_body: &mut Tokens<Swift>,
) -> Result<()> {
    push!(method_body, "let r_result = ");
    method_body.append(
        SwiftConvert {
            ty: cb_method.return_type.clone(),
        }
        .artifact_to_transfer("result".to_string(), Direction::Push),
    );
    Ok(())
}
