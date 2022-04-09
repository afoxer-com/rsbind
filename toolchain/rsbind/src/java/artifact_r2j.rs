use crate::ast::contract::desc::{ArgDesc, MethodDesc};
use crate::ast::types::AstType;
use crate::base::{Convertible, Direction};
use crate::errors::*;
use crate::java::converter::JavaConvert;
use crate::java::types::JavaType;
use rstgen::{Java, Tokens};

pub(crate) fn fill_arg_convert(arg: &ArgDesc, cb_body: &mut Tokens<Java>, pkg: &str) -> Result<()> {
    if let AstType::Void = arg.ty.clone() {
        return Ok(());
    }

    let java = Java::from(JavaType::new(arg.ty.clone(), pkg.to_string()));
    let convert =
        JavaConvert { ty: arg.ty.clone() }.transfer_to_artifact(arg.name.clone(), Direction::Push);
    push!(
        cb_body,
        java,
        " ",
        "j_",
        arg.name.clone(),
        " = ",
        convert,
        ";"
    );
    Ok(())
}

pub(crate) fn fill_return_convert(
    cb_body: &mut Tokens<Java>,
    cb_method: &MethodDesc,
) -> Result<()> {
    if let AstType::Void = cb_method.return_type.clone() {
        return Ok(());
    }

    let convert = JavaConvert {
        ty: cb_method.return_type.clone(),
    }
    .artifact_to_transfer("result".to_string(), Direction::Push);
    push!(cb_body, "return ", convert, ";");

    Ok(())
}
