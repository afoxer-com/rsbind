use proc_macro2::TokenStream;
use rstgen::{java, Java, Tokens};

use crate::ast::contract::desc::{ArgDesc, MethodDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::errors::*;
use crate::java::types::JavaType;

pub(crate) fn fill_arg_convert(
    method_body: &mut Tokens<Java>,
    arg: &ArgDesc,
    pkg: &str,
) -> Result<TokenStream> {
    let converted = format!("r_{}", &arg.name);
    match arg.ty.clone() {
        AstType::Void => (),
        AstType::Callback(ref origin) => {
            push!(
                method_body,
                "long ",
                converted,
                " = Internal",
                origin.to_string(),
                ".pushGlobalCallback(",
                arg.name.clone(),
                ");"
            );
        }
        AstType::Boolean => {
            push!(
                method_body,
                "int ",
                converted,
                " = ",
                arg.name.clone(),
                " ? 1 : 0;"
            );
        }
        AstType::Vec(AstBaseType::Byte(_)) => {
            let java = JavaType::new(arg.ty.clone(), pkg.to_string());
            let java = Java::from(java);
            push!(
                method_body,
                java,
                " ",
                converted,
                " = ",
                arg.name.clone(),
                ";"
            );
        }
        AstType::Vec(_) => {
            let json_cls = java::imported("com.google.gson", "Gson");
            push!(
                method_body,
                "String ",
                converted,
                " = new ",
                json_cls,
                "().toJson(",
                arg.name.clone(),
                ");"
            );
        }
        AstType::Struct(_) => {
            let json_cls = java::imported("com.google.gson", "Gson");
            push!(
                method_body,
                "String ",
                converted,
                " = new ",
                json_cls,
                "().toJson(",
                arg.name.clone(),
                ");"
            );
        }
        _ => {
            let java = JavaType::new(arg.ty.clone(), pkg.to_string());
            let java = Java::from(java);
            push!(
                method_body,
                java,
                " ",
                converted,
                " = ",
                arg.name.clone(),
                ";"
            );
        }
    }

    Ok(quote! {})
}

pub(crate) fn fill_return_convert(
    method_body: &mut Tokens<Java>,
    method: &MethodDesc,
    pkg: &str,
) -> Result<()> {
    let return_ty = JavaType::new(method.return_type.clone(), pkg.to_string());

    match return_ty.ast_type.clone() {
        AstType::Void => (),
        AstType::Vec(AstBaseType::Byte(_)) => {
            push!(method_body, "return ret;");
        }
        AstType::Vec(_) => {
            let sub_ty = return_ty.get_base_ty();
            let json = java::imported("com.google.gson", "Gson");
            push!(
                method_body,
                "return new ",
                json,
                "().fromJson(ret, ",
                sub_ty.clone().as_boxed(),
                "[].class);"
            );
        }
        AstType::Boolean => {
            push!(method_body, "return ret > 0 ? true : false;");
        }
        AstType::Struct(origin) => {
            let json = java::imported("com.google.gson", "Gson");
            push!(
                method_body,
                "return new ",
                json,
                "().fromJson(ret,",
                origin,
                ".class);"
            );
        }
        AstType::Callback(ref origin) => {
            push!(
                method_body,
                "return new Internal",
                origin.to_string(),
                ".J2R",
                origin.to_string(),
                "Wrapper(ret);"
            );
        }
        _ => {
            push!(method_body, "return ret;");
        }
    }

    Ok(())
}
