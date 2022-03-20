use crate::ast::contract::desc::{ArgDesc, MethodDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::errors::*;
use crate::swift::mapping::SwiftMapping;
use crate::swift::types::SwiftType;
use heck::ToLowerCamelCase;
use rstgen::swift::Swift;
use rstgen::Tokens;

/// Generate callback argument handling for swift code.
pub(crate) struct ArgCbGen<'a> {
    pub(crate) desc: &'a TraitDesc,
    pub(crate) arg: &'a ArgDesc,
    pub(crate) callback: &'a TraitDesc,
}

impl<'a> ArgCbGen<'a> {
    pub(crate) fn gen(&self) -> Result<Tokens<'a, Swift<'a>>> {
        let mut method_body: Tokens<Swift> = toks!();

        // Store the callback to global callback map.
        self.fill_callback_index(self.arg, &mut method_body)?;

        let cb = self.callback;

        let mut cb_args_model = "".to_string();
        for cb_method in cb.methods.iter() {
            self.fill_cb_closure_method_sig(cb_method, self.arg, &mut method_body)?;

            method_body.nested(toks!(
                "let ",
                format!("{}_callback", &self.arg.name),
                " = globalCallbacks[index] as! ",
                cb.name.clone()
            ));

            for cb_arg in cb_method.args.iter() {
                self.fill_cb_closure_arg_convert(cb_arg, &mut method_body)?;
            }

            self.fill_cb_closure_call(cb_method, self.arg, &mut method_body)?;

            self.fill_cb_closure_return_convert(cb_method, &mut method_body)?;

            method_body.push(toks!("}"));

            cb_args_model = format!(
                "{}{}:{}_{},",
                cb_args_model, &cb_method.name, &self.arg.name, &cb_method.name
            );
        }
        self.fill_cb_closure_free_fn(self.arg, &mut method_body)?;
        self.fill_cb_closure_free_ptr_fn(self.arg, &mut method_body)?;

        let free_fn_name = format!("{}_callback_free", &self.arg.name);
        let free_ptr_name = format!("{}_ptr_free", &self.arg.name);
        method_body.push(toks!(format!(
            "let s_{} = {}_{}_Model({}free_callback: {}, free_ptr: {}, index: {}_index)\n",
            &self.arg.name,
            &self.desc.mod_name,
            &cb.name,
            cb_args_model,
            &free_fn_name,
            &free_ptr_name,
            &self.arg.name
        )));

        Ok(method_body)
    }

    fn fill_callback_index(&self, arg: &ArgDesc, method_body: &mut Tokens<Swift>) -> Result<()> {
        let index_name = format!("{}_index", &arg.name);
        method_body.push(toks!("let ", index_name.clone(), " = globalIndex + 1"));
        method_body.push(toks!("globalIndex = ", index_name.clone()));
        method_body.push(toks!(
            "globalCallbacks[",
            index_name,
            "] = ",
            arg.name.clone()
        ));

        Ok(())
    }

    fn fill_cb_closure_method_sig(
        &self,
        cb_method: &MethodDesc,
        arg: &ArgDesc,
        method_body: &mut Tokens<Swift>,
    ) -> Result<()> {
        let mut arg_params = "(index".to_owned();
        let mut args_str = "(Int64".to_owned();
        for cb_arg in cb_method.args.iter() {
            let cb_arg_ty = SwiftMapping::map_transfer_type(&cb_arg.ty);
            arg_params = format!("{}, {}", &arg_params, &cb_arg.name);
            args_str = format!("{}, {}", &args_str, &cb_arg_ty);
        }
        arg_params = format!("{})", &arg_params);
        args_str = format!("{})", &args_str);

        let cb_return_ty = SwiftMapping::map_transfer_type(&cb_method.return_type);
        let closure = format!("{} -> {}", &args_str, &cb_return_ty);
        arg_params = format!("{} -> {}", &arg_params, &cb_return_ty);

        method_body.push(toks!(
            "let ",
            format!("{}_{}", &arg.name, &cb_method.name),
            ": @convention(c) ",
            closure,
            " = {"
        ));
        method_body.nested(toks!(arg_params, " in\n"));
        Ok(())
    }

    fn fill_cb_closure_arg_convert(
        &self,
        cb_arg: &ArgDesc,
        method_body: &mut Tokens<Swift>,
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
            AstType::Callback(_) => {
                panic!("Don't support callback argument in callback");
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

    fn fill_cb_closure_call(
        &self,
        cb_method: &MethodDesc,
        arg: &ArgDesc,
        method_body: &mut Tokens<Swift>,
    ) -> Result<()> {
        let mut cb_method_call = "(".to_string();
        for (index, cb_arg) in cb_method.args.iter().enumerate() {
            cb_method_call = format!("{}{}: c_{}", &cb_method_call, &cb_arg.name, &cb_arg.name);
            if index != cb_method.args.len() - 1 {
                cb_method_call = format!("{}, ", &cb_method_call);
            }
        }

        cb_method_call = format!("{})", &cb_method_call);

        match cb_method.return_type.clone() {
            AstType::Void => {
                method_body.nested(toks!(
                    format!("{}_callback", &arg.name),
                    ".",
                    cb_method.name.to_lower_camel_case(),
                    cb_method_call
                ));
            }
            _ => {
                method_body.nested(toks!(
                    "let result = ",
                    format!("{}_callback", &arg.name),
                    ".",
                    cb_method.name.to_lower_camel_case(),
                    cb_method_call
                ));
            }
        }

        Ok(())
    }

    fn fill_cb_closure_return_convert(
        &self,
        cb_method: &MethodDesc,
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
                let ty = SwiftMapping::map_transfer_type(&cb_method.return_type);
                method_body.nested(toks!("return ", ty, "(result)"));
            }
            AstType::Boolean => {
                method_body.nested(toks!("return result ? 1 : 0"));
            }
            AstType::String => {
                method_body.nested(toks!("return result.withCString { $0 }"));
            }
            AstType::Vec(AstBaseType::Byte(ref origin))
            | AstType::Vec(AstBaseType::Short(ref origin))
            | AstType::Vec(AstBaseType::Int(ref origin))
            | AstType::Vec(AstBaseType::Long(ref origin)) => {
                let transfer_ty = SwiftMapping::map_transfer_type(&cb_method.return_type);
                let base_ty = match cb_method.return_type.clone() {
                    AstType::Vec(base) => SwiftMapping::map_transfer_type(&AstType::from(base)),
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
            AstType::Callback(_) => {
                panic!("Don't support Callback in callback return.");
            }
            AstType::Struct(_) => {
                panic!("Don't support Struct in callback return.");
            }
        }
        Ok(())
    }

    fn fill_cb_closure_free_fn(
        &self,
        arg: &ArgDesc,
        method_body: &mut Tokens<Swift>,
    ) -> Result<()> {
        let free_fn_name = format!("{}_callback_free", &arg.name);
        method_body.push(toks!(
            "let ",
            free_fn_name,
            " : @convention(c)(Int64) -> () = {"
        ));
        method_body.nested(toks!("(index) in"));
        method_body.nested(toks!("globalCallbacks.removeValue(forKey: index)"));
        method_body.push(toks!("}"));
        Ok(())
    }

    fn fill_cb_closure_free_ptr_fn(
        &self,
        arg: &ArgDesc,
        method_body: &mut Tokens<Swift>,
    ) -> Result<()> {
        let free_ptr_name = format!("{}_ptr_free", &arg.name);
        method_body.push(toks!(
            "let ",
            free_ptr_name,
            " : @convention(c) (UnsafeMutablePointer<Int8>?, Int32) -> () = {"
        ));
        method_body.nested(toks!("(ptr, count) in"));
        method_body.nested(toks!("ptr?.deinitialize(count: Int(count))"));
        method_body.nested(toks!("ptr?.deallocate()"));
        method_body.push(toks!("}"));
        Ok(())
    }
}
