use rstgen::{java, Java, Tokens};

use crate::ast::contract::desc::{ArgDesc, MethodDesc};
use crate::ast::types::{AstBaseType, AstType};
/// Java to Rust data convert in Java.
use crate::errors::*;
use crate::java::types::JavaType;

pub(crate) fn fill_arg_convert(arg: &ArgDesc, cb_body: &mut Tokens<Java>, pkg: &str) -> Result<()> {
    match arg.ty.clone() {
        AstType::Boolean => {
            cb_body.push(toks!(
                "boolean ",
                "j_",
                arg.name.clone(),
                " = ",
                arg.name.clone(),
                " > 0 ? true : false;"
            ));
        }
        AstType::Struct(sub) => {
            let json = java::imported("com.google.gson", "Gson");
            cb_body.push(toks!(
                sub.clone(),
                " j_",
                arg.name.clone(),
                " = new ",
                json,
                "().fromJson(",
                arg.name.clone(),
                ", ",
                sub,
                ".class);"
            ));
        }
        AstType::Vec(AstBaseType::Byte(_)) => {
            let java = JavaType::new(arg.ty.clone(), pkg.to_string());
            cb_body.push(toks!(
                java.get_base_ty(),
                "[] ",
                "j_",
                arg.name.clone(),
                " = ",
                arg.name.clone(),
                ";"
            ));
        }
        AstType::Vec(_) => {
            let json = java::imported("com.google.gson", "Gson");
            let java = JavaType::new(arg.ty.clone(), pkg.to_string());
            cb_body.push(toks!(
                java.get_base_ty().as_boxed(),
                "[] ",
                "j_",
                arg.name.clone(),
                " = new ",
                json,
                "().fromJson(",
                arg.name.clone(),
                ", ",
                java.get_base_ty().as_boxed(),
                "[].class);"
            ));
        }
        AstType::Callback(ref origin) => {
            let java = JavaType::new(arg.ty.clone(), pkg.to_string());
            cb_body.push(toks!(
                Java::from(java), " j_", arg.name.clone(), " = new Internal", origin.to_string(), ".J2R", origin.to_string(), "Wrapper(", arg.name.clone(), ");"
            ));
        }
        _ => {
            let java = JavaType::new(arg.ty.clone(), pkg.to_string());
            cb_body.push(toks!(
                Java::from(java),
                " j_",
                arg.name.clone(),
                " = ",
                arg.name.clone(),
                ";"
            ));
        }
    }

    Ok(())
}

pub(crate) fn fill_return_convert(
    cb_body: &mut Tokens<Java>,
    cb_method: &MethodDesc,
) -> Result<()> {
    match cb_method.return_type.clone() {
        AstType::Boolean => {
            cb_body.push(toks!("return result ? 1 : 0;"));
        }
        AstType::Vec(AstBaseType::Byte(_)) => {
            cb_body.push(toks!("return result;"));
        }
        AstType::Vec(_) => {
            cb_body.push(toks!("return new Gson().toJson(result);"));
        }
        AstType::Void => (),
        AstType::Callback(ref origin) => {
            cb_body.push(toks!(
                "return Internal", origin.to_string(), ".pushGlobalCallback(result);"
            ));
        }
        _ => {
            cb_body.push(toks!("return result;"));
        }
    }
    Ok(())
}
