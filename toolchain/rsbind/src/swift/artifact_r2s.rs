use rstgen::swift::Swift;
use rstgen::Tokens;

use crate::ast::contract::desc::{ArgDesc, MethodDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::errors::*;
use crate::swift::mapping::SwiftMapping;
use crate::swift::types::SwiftType;
use crate::ErrorKind::GenerateError;

///
/// C to Swift data convert.
///
pub(crate) fn fill_arg_convert<'a, 'b>(
    cb_arg: &'a ArgDesc,
    callbacks: &'a [&'a TraitDesc],
    method_body: &'b mut Tokens<'a, Swift<'a>>,
) -> Result<()> {
    let mut fn_body = toks!();
    let cb_arg_str = SwiftType {
        ast_type: cb_arg.ty.clone(),
    }
    .to_str();
    match cb_arg.ty.clone() {
        AstType::Void => {}
        AstType::Byte(_)
        | AstType::Short(_)
        | AstType::Int(_)
        | AstType::Long(_)
        | AstType::Float(_)
        | AstType::Double(_) => {
            let ty = SwiftMapping::map_swift_sig_type(&cb_arg.ty);
            fn_body.nested(toks!(
                "let ",
                format!("c_{}", &cb_arg.name),
                " = ",
                ty,
                "(",
                cb_arg.name.clone(),
                ")"
            ));
        }
        AstType::Boolean => {
            fn_body.nested(toks!(
                "let ",
                format!("c_{}", &cb_arg.name),
                " : Bool = ",
                cb_arg.name.clone(),
                " > 0 ? true : false"
            ));
        }
        AstType::String => {
            method_body.nested(toks!(
                "let ",
                format!("c_{}", &cb_arg.name),
                " = String(cString: ",
                cb_arg.name.clone(),
                "!)"
            ));
        }
        AstType::Callback(ref origin) => {
            method_body.nested(toks!(
                "let ",
                format!("c_{}", &cb_arg.name),
                " = Internal",
                origin.to_string(),
                ".modelToCallback(model: ",
                cb_arg.name.clone(),
                ")"
            ));
        }
        AstType::Vec(AstBaseType::Byte(_))
        | AstType::Vec(AstBaseType::Short(_))
        | AstType::Vec(AstBaseType::Int(_))
        | AstType::Vec(AstBaseType::Long(_)) => {
            let ty = SwiftMapping::map_swift_sig_type(&cb_arg.ty);
            fn_body.nested(toks!(
                "let ",
                format!("c_{}", &cb_arg.name),
                " = ",
                ty,
                "(UnsafeBufferPointer(start: ",
                cb_arg.name.clone(),
                ".ptr, count: Int(",
                cb_arg.name.clone(),
                ".len)))"
            ));
        }
        AstType::Vec(_) => {
            fn_body.nested(toks!(
                "let ",
                format!("c_tmp_{}", &cb_arg.name),
                " = String(cString:",
                cb_arg.name.clone(),
                "!)"
            ));
            fn_body.nested(toks!(
                "var ",
                format!("c_option_{}", &cb_arg.name),
                " : ",
                cb_arg_str.clone(),
                "?"
            ));
            fn_body.nested(toks!("autoreleasepool {"));
            fn_body.nested({
                let mut body = toks!();
                body.nested(toks!(
                    "let ",
                    format!("c_tmp_json_{}", &cb_arg.name),
                    " = ",
                    format!("c_tmp_{}", &cb_arg.name),
                    ".data(using: .utf8)!"
                ));
                body.nested(toks!("let decoder = JSONDecoder()"));
                body.nested(toks!(
                    format!("c_option_{}", &cb_arg.name),
                    " = try! decoder.decode(",
                    cb_arg_str,
                    ".self, from: ",
                    format!("c_tmp_json_{}", &cb_arg.name),
                    ")"
                ));

                body
            });
            fn_body.nested(toks!("}"));
            fn_body.nested(toks!(
                "let ",
                format!("c_{}", &cb_arg.name),
                " = ",
                format!("c_option_{}", &cb_arg.name),
                "!"
            ));
        }
        AstType::Struct(_) => {
            fn_body.nested(toks!(
                "let ",
                format!("c_tmp_{}", &cb_arg.name),
                " = String(cString:",
                cb_arg.name.clone(),
                "!)"
            ));
            fn_body.nested(toks!(
                "var ",
                format!("c_option_{}", &cb_arg.name),
                " : ",
                cb_arg_str.clone(),
                "?"
            ));
            fn_body.nested(toks!("autoreleasepool {"));
            fn_body.nested({
                let mut body = toks!();
                body.nested(toks!(
                    "let ",
                    format!("c_tmp_json_{}", &cb_arg.name),
                    " = ",
                    format!("c_tmp_{}", &cb_arg.name),
                    ".data(using: .utf8)!"
                ));
                body.nested(toks!("let decoder = JSONDecoder()"));
                body.nested(toks!(
                    format!("c_option_{}", &cb_arg.name),
                    " = try! decoder.decode(",
                    cb_arg_str,
                    ".self, from: ",
                    format!("c_tmp_json_{}", &cb_arg.name),
                    ")"
                ));
                body
            });
            fn_body.nested(toks!("}"));
            fn_body.nested(toks!(
                "let ",
                format!("c_{}", &cb_arg.name),
                " = ",
                format!("c_option_{}", &cb_arg.name),
                "!"
            ));
        }
    }
    method_body.push(fn_body);
    Ok(())
}

pub(crate) fn fill_return_convert(
    cb_method: &MethodDesc,
    callbacks: &[&TraitDesc],
    method_body: &mut Tokens<Swift>,
) -> Result<()> {
    match cb_method.return_type.clone() {
        AstType::Void => {}
        AstType::Byte(_)
        | AstType::Short(_)
        | AstType::Int(_)
        | AstType::Long(_)
        | AstType::Float(_)
        | AstType::Double(_) => {
            let ty = SwiftMapping::map_transfer_type(&cb_method.return_type, callbacks);
            method_body.nested(toks!("return ", ty, "(result)"));
        }
        AstType::Boolean => {
            method_body.nested(toks!("return result ? 1 : 0"));
        }
        AstType::String => {
            method_body.nested(toks!("return result.withCString { $0 }"));
        }
        AstType::Vec(AstBaseType::Byte(_))
        | AstType::Vec(AstBaseType::Short(_))
        | AstType::Vec(AstBaseType::Int(_))
        | AstType::Vec(AstBaseType::Long(_)) => {
            let transfer_ty = SwiftMapping::map_transfer_type(&cb_method.return_type, callbacks);
            let base_ty = match cb_method.return_type.clone() {
                AstType::Vec(base) => {
                    SwiftMapping::map_transfer_type(&AstType::from(base), callbacks)
                }
                _ => "".to_string(),
            };
            method_body.nested(toks!(
                "let tmp_ptr = UnsafeMutablePointer<",
                base_ty,
                ">.allocate(capacity: result.count)"
            ));
            method_body.nested(toks!(
                "tmp_ptr.initialize(from: result, count: result.count)"
            ));
            method_body.nested(toks!(
                "return ",
                transfer_ty,
                "(ptr: tmp_ptr, len: Int32(result.count))"
            ));
        }
        AstType::Vec(_) => {}
        AstType::Callback(ref origin) => {
            method_body.push(toks!(
                "return Internal",
                origin.clone(),
                ".callbackToModel(callback:  result)"
            ));
        }
        AstType::Struct(_) => {
            panic!("Don't support Struct in callback return.");
        }
    }
    Ok(())
}