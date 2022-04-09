use crate::ast::contract::desc::{ArgDesc, MethodDesc};
use crate::ast::types::AstType;
use crate::base::{Convertible, Direction};
use crate::errors::*;
use crate::java::converter::JavaConvert;
use crate::java::types::JavaType;
use rstgen::{Java, Tokens};

pub(crate) fn fill_arg_convert(
    method_body: &mut Tokens<Java>,
    arg: &ArgDesc,
    pkg: &str,
) -> Result<()> {
    if let AstType::Void = arg.ty.clone() {
        return Ok(());
    }

    let java = JavaType::new(arg.ty.clone(), pkg.to_string()).to_transfer();
    let converted = format!("r_{}", &arg.name);
    let convert = JavaConvert { ty: arg.ty.clone() }
        .artifact_to_transfer(arg.name.clone(), Direction::Invoke);
    push!(method_body, java, "  ", converted, " = ", convert, ";");

    Ok(())
}

pub(crate) fn fill_return_convert(
    method_body: &mut Tokens<Java>,
    method: &MethodDesc,
    _pkg: &str,
) -> Result<()> {
    if let AstType::Void = method.return_type.clone() {
        return Ok(());
    }

    let convert = JavaConvert {
        ty: method.return_type.clone(),
    }
    .transfer_to_artifact("ret".to_string(), Direction::Invoke);
    push!(method_body, "return ", convert, ";");
    Ok(())
}
