use rstgen::swift::Swift;
use rstgen::Tokens;

use crate::ast::contract::desc::{ArgDesc, MethodDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::errors::*;
use crate::swift::mapping::SwiftMapping;
use crate::swift::types::SwiftType;

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
            nested!(
                fn_body,
                "let ",
                format!("c_{}", &cb_arg.name),
                " = ",
                ty,
                "(",
                cb_arg.name.clone(),
                ")"
            );
        }
        AstType::Boolean => {
            nested!(
                fn_body,
                "let ",
                format!("c_{}", &cb_arg.name),
                " : Bool = ",
                cb_arg.name.clone(),
                " > 0 ? true : false"
            );
        }
        AstType::String => {
            nested!(
                method_body,
                "let ",
                format!("c_{}", &cb_arg.name),
                " = String(cString: ",
                cb_arg.name.clone(),
                "!)"
            );
        }
        AstType::Callback(ref origin) => {
            nested!(
                method_body,
                "let ",
                format!("c_{}", &cb_arg.name),
                " = Internal",
                origin.to_string(),
                ".modelToCallback(model: ",
                cb_arg.name.clone(),
                ")"
            );
        }
        AstType::Vec(AstBaseType::Byte(_))
        | AstType::Vec(AstBaseType::Short(_))
        | AstType::Vec(AstBaseType::Int(_))
        | AstType::Vec(AstBaseType::Long(_)) => {
            let ty = SwiftMapping::map_swift_sig_type(&cb_arg.ty);
            nested!(
                fn_body,
                "let ",
                format!("c_{}", &cb_arg.name),
                " = ",
                ty,
                "(UnsafeBufferPointer(start: ",
                cb_arg.name.clone(),
                ".ptr, count: Int(",
                cb_arg.name.clone(),
                ".len)))"
            );
        }
        AstType::Vec(AstBaseType::Struct(ref origin)) => {
            let proxy_ty = format!("Proxy{}", origin);
            let c_array_ty = format!("C{}Array", origin);
            push!(method_body, "let ", format!("c_{}", &cb_arg.name) ," = { () -> [", origin.clone(), "] in");
            nested!(
                method_body,
                "let proxy_array = [", proxy_ty,"](UnsafeBufferPointer(start: ", cb_arg.name.clone() ,".ptr, count: Int(", cb_arg.name.clone() ,".len)))",
            );
            nested!(method_body, "let struct_arg = proxy_array.map { proxy in DemoStruct(proxy: proxy) }");
            nested!(method_body, "free_", c_array_ty, "(", cb_arg.name.clone(),")");
            nested!(method_body, "return struct_arg");
            push!(method_body, "}()");
        }
        AstType::Vec(_) => {
            nested!(
                fn_body,
                "let ",
                format!("c_tmp_{}", &cb_arg.name),
                " = String(cString:",
                cb_arg.name.clone(),
                "!)"
            );
            nested!(
                fn_body,
                "var ",
                format!("c_option_{}", &cb_arg.name),
                " : ",
                cb_arg_str.clone(),
                "?"
            );
            nested!(fn_body, "autoreleasepool {");
            fn_body.nested({
                let mut body = toks!();
                nested!(
                    body,
                    "let ",
                    format!("c_tmp_json_{}", &cb_arg.name),
                    " = ",
                    format!("c_tmp_{}", &cb_arg.name),
                    ".data(using: .utf8)!"
                );
                nested!(body, "let decoder = JSONDecoder()");
                nested!(
                    body,
                    format!("c_option_{}", &cb_arg.name),
                    " = try! decoder.decode(",
                    cb_arg_str,
                    ".self, from: ",
                    format!("c_tmp_json_{}", &cb_arg.name),
                    ")"
                );

                body
            });
            nested!(fn_body, "}");
            nested!(
                fn_body,
                "let ",
                format!("c_{}", &cb_arg.name),
                " = ",
                format!("c_option_{}", &cb_arg.name),
                "!"
            );
        }
        AstType::Struct(_) => {
            let ty = SwiftType::new(cb_arg.ty.clone());
            nested!(
                fn_body,
                "let ",
                format!("c_{}", &cb_arg.name),
                " = ",
                Swift::from(ty),
                "(proxy: ", cb_arg.name.clone(), ")"
            );
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
            nested!(method_body, "return ", ty, "(result)");
        }
        AstType::Boolean => {
            nested!(method_body, "return result ? 1 : 0");
        }
        AstType::String => {
            nested!(method_body, "return result.withCString { $0 }");
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
            nested!(
                method_body,
                "let tmp_ptr = UnsafeMutablePointer<",
                base_ty,
                ">.allocate(capacity: result.count)"
            );
            nested!(
                method_body,
                "tmp_ptr.initialize(from: result, count: result.count)"
            );
            nested!(
                method_body,
                "return ",
                transfer_ty,
                "(ptr: tmp_ptr, len: Int32(result.count))"
            );
        }
        AstType::Vec(AstBaseType::Struct(_)) => {
            let transfer_ty = SwiftMapping::map_transfer_type(&cb_method.return_type, callbacks);
            let base_ty = match cb_method.return_type.clone() {
                AstType::Vec(base) => {
                    SwiftMapping::map_transfer_type(&AstType::from(base), callbacks)
                }
                _ => "".to_string(),
            };

            nested!(method_body, "let proxy_result = result.map { each in each.intoProxy() }");
            nested!(
                method_body,
                "let tmp_ptr = UnsafeMutablePointer<",
                base_ty,
                ">.allocate(capacity: result.count)"
            );
            nested!(
                method_body,
                "tmp_ptr.initialize(from: proxy_result, count: proxy_result.count)"
            );

            nested!(
                method_body,
                "return ",
                transfer_ty,
                "(ptr: tmp_ptr, len: Int32(proxy_result.count))"
            );
        }
        AstType::Struct(_) => {
            push!(
                method_body,
                "return result.intoProxy()"
            );
        }
        AstType::Vec(_) => {
            push!(
                method_body,
                "return autoreleasepool { () -> UnsafePointer<Int8>? in",
            );
            nested!(method_body, "let encoder = JSONEncoder()");
            nested!(method_body, "let data_result = try! encoder.encode(result)");
            nested!(
                method_body,
                "let str_result = String(data: data_result, encoding: .utf8)"
            );
            nested!(method_body, "return str_result?.withCString{$0}");
            push!(method_body, "}");
        }
        AstType::Callback(ref origin) => {
            push!(
                method_body,
                "return Internal",
                origin.clone(),
                ".callbackToModel(callback:  result)"
            );
        }
    }
    Ok(())
}
