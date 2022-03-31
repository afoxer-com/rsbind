use rstgen::{java, Java, Tokens};

use crate::ast::contract::desc::{ArgDesc, MethodDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::errors::*;
use crate::java::types::JavaType;

pub(crate) fn fill_arg_convert(arg: &ArgDesc, cb_body: &mut Tokens<Java>, pkg: &str) -> Result<()> {
    match arg.ty.clone() {
        AstType::Boolean => {
            push!(
                cb_body,
                "boolean ",
                "j_",
                arg.name.clone(),
                " = ",
                arg.name.clone(),
                " > 0 ? true : false;"
            );
        }
        AstType::Struct(sub) => {
            let json = java::imported("com.google.gson", "Gson");
            push!(
                cb_body,
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
            );
        }
        AstType::Vec(AstBaseType::Byte(_)) => {
            let java = JavaType::new(arg.ty.clone(), pkg.to_string());
            push!(
                cb_body,
                java.get_base_ty(),
                "[] ",
                "j_",
                arg.name.clone(),
                " = ",
                arg.name.clone(),
                ";"
            );
        }
        AstType::Vec(_) => {
            let json = java::imported("com.google.gson", "Gson");
            let java = JavaType::new(arg.ty.clone(), pkg.to_string());
            push!(
                cb_body,
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
            );
        }
        AstType::Callback(ref origin) => {
            let java = JavaType::new(arg.ty.clone(), pkg.to_string());
            push!(
                cb_body,
                Java::from(java),
                " j_",
                arg.name.clone(),
                " = new Internal",
                origin.to_string(),
                ".J2R",
                origin.to_string(),
                "Wrapper(",
                arg.name.clone(),
                ");"
            );
        }
        AstType::Void
        | AstType::Byte(_)
        | AstType::Int(_)
        | AstType::Short(_)
        | AstType::Long(_)
        | AstType::Float(_)
        | AstType::Double(_)
        | AstType::String => {
            let java = JavaType::new(arg.ty.clone(), pkg.to_string());
            push!(
                cb_body,
                Java::from(java),
                " j_",
                arg.name.clone(),
                " = ",
                arg.name.clone(),
                ";"
            );
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
            push!(cb_body, "return result ? 1 : 0;");
        }
        AstType::Vec(AstBaseType::Byte(_)) => {
            push!(cb_body, "return result;");
        }
        AstType::Struct(_) | AstType::Vec(_) => {
            push!(cb_body, "return new Gson().toJson(result);");
        }
        AstType::Void => (),
        AstType::Callback(ref origin) => {
            push!(
                cb_body,
                "return Internal",
                origin.to_string(),
                ".pushGlobalCallback(result);"
            );
        }
        AstType::String
        | AstType::Byte(_)
        | AstType::Int(_)
        | AstType::Short(_)
        | AstType::Long(_)
        | AstType::Float(_)
        | AstType::Double(_) => {
            push!(cb_body, "return result;");
        }
    }
    Ok(())
}
