use heck::ToLowerCamelCase;
use std::fs;
use std::path::PathBuf;

use rstgen::swift::{self, *};
use rstgen::{Custom, Formatter, IntoTokens, Tokens};

use crate::ast::contract::desc::{ArgDesc, MethodDesc, StructDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::ast::AstResult;
use crate::errors::*;
use crate::swift::arg_cb::ArgCbGen;
use crate::swift::callback::CallbackGen;
use crate::swift::mapping::SwiftMapping;
use crate::swift::struct_::StructGen;
use crate::swift::types::{to_swift_file, SwiftType};
use crate::ErrorKind::GenerateError;

pub(crate) struct TraitGen<'a> {
    pub desc: &'a TraitDesc,
    pub callbacks: Vec<TraitDesc>,
}

impl<'a> TraitGen<'a> {
    pub fn gen(&'a self) -> Result<String> {
        let class_name = format!("Internal{}", &self.desc.name);
        let mut class = Class::new(class_name.clone());
        class.modifiers = vec![Modifier::Internal];

        let mut tokens = toks!();
        self.fill_global_block(&mut tokens)?;

        // let mut sel_callbacks = vec![];
        for method in self.desc.methods.iter() {
            println!("generate swift codes for {}", &method.name);
            // Method signature
            let mut m = self.fill_method_sig(method)?;

            let mut method_body: Tokens<Swift> = Tokens::new();

            let mut byte_count = 0;
            for arg in method.args.iter() {
                if let AstType::Vec(AstBaseType::Byte(_)) = arg.ty.clone() {
                    byte_count += 1;
                    method_body.push(toks!(
                        arg.name.clone(),
                        ".withUnsafeBufferPointer { ",
                        arg.name.clone(),
                        "_buffer in"
                    ));
                }
            }

            // Argument convert
            self.fill_arg_convert(&mut method_body, method)?;
            // Call native method
            self.fill_call_native_method(&mut method_body, method)?;
            // Return type convert
            self.fill_return_type_convert(&mut method_body, method)?;

            for _i in 0..byte_count {
                method_body.push("}");
            }

            m.body = method_body;
            class.methods.push(m);
        }

        tokens.push(class.into_tokens());

        to_swift_file(tokens)
    }

    fn fill_global_block(&self, tokens: &mut Tokens<Swift>) -> Result<()> {
        let global_vars = toks!(
            "private  var globalIndex : Int64 = 0\n",
            "private  var globalCallbacks : [Int64: Any] = [Int64: Any]()\n"
        );
        tokens.push(global_vars);
        Ok(())
    }

    fn fill_method_sig(&self, method: &MethodDesc) -> Result<Method> {
        let mut m = Method::new(method.name.to_lower_camel_case());
        m.modifiers = vec![Modifier::Internal, Modifier::Static];
        m.returns(SwiftMapping::map_sig_type(&method.return_type));

        let args = method.args.clone();
        for arg in args.iter() {
            let argument =
                swift::Argument::new(SwiftMapping::map_sig_type(&arg.ty), arg.name.clone());
            m.arguments.push(argument);
        }

        Ok(m)
    }

    fn fill_arg_convert(
        &'a self,
        method_body: &mut Tokens<'a, Swift<'a>>,
        method: &'a MethodDesc,
    ) -> Result<()> {
        for arg in method.args.iter() {
            // Argument convert
            println!("quote arg convert for {}", arg.name.clone());
            let s_arg_name = format!("s_{}", &arg.name);
            match arg.ty.clone() {
                AstType::Void => {}
                AstType::Boolean => method_body.push(toks!(
                    "let ",
                    s_arg_name.clone(),
                    ": Int32 = ",
                    arg.name.clone(),
                    " ? 1 : 0"
                )),
                AstType::Byte(_) => method_body.push(toks!(
                    "let ",
                    s_arg_name.clone(),
                    " = Int8(",
                    arg.name.clone(),
                    ")"
                )),
                AstType::Int(_) => method_body.push(toks!(
                    "let ",
                    s_arg_name.clone(),
                    " = Int32(",
                    arg.name.clone(),
                    ")"
                )),
                AstType::Long(_) => method_body.push(toks!(
                    "let ",
                    s_arg_name.clone(),
                    " = Int64(",
                    arg.name.clone(),
                    ")"
                )),
                AstType::Float(_) => method_body.push(toks!(
                    "let ",
                    s_arg_name.clone(),
                    " = Float32(",
                    arg.name.clone(),
                    ")"
                )),
                AstType::Double(_) => method_body.push(toks!(
                    "let ",
                    s_arg_name.clone(),
                    " = Float64(",
                    arg.name.clone(),
                    ")"
                )),
                AstType::String => {
                    method_body.push(toks!("let ", s_arg_name.clone(), " = ", arg.name.clone()))
                }
                AstType::Vec(AstBaseType::Byte(_)) => {
                    let arg_buffer_name = format!("{}_buffer", &arg.name);
                    method_body.push(toks!(
                        "let ",
                        s_arg_name.clone(),
                        " = CInt8Array(ptr: ",
                        arg_buffer_name.clone(),
                        ".baseAddress, len: Int32(",
                        arg_buffer_name.clone(),
                        ".count))"
                    ))
                }
                AstType::Vec(AstBaseType::Struct(origin)) => {
                    let encoder_name = format!("{}_encoder", &arg.name);
                    method_body.push(toks!("let ", encoder_name.clone(), " = JSONEncoder()"));
                    method_body.push(toks!(
                        "let ",
                        format!("data_{}", &arg.name),
                        " = try! ",
                        encoder_name.clone(),
                        ".encode(",
                        arg.name.clone(),
                        ")"
                    ));
                    method_body.push(toks!(
                        "let ",
                        format!("s_{}", &arg.name),
                        " = String(data: ",
                        format!("data_{}", &arg.name),
                        ", encoding: .utf8)!"
                    ))
                }
                AstType::Vec(_) | AstType::Struct(_) => {
                    let encoder_name = format!("{}_encoder", &arg.name);
                    method_body.push(toks!("let ", encoder_name.clone(), " = JSONEncoder()"));
                    method_body.push(toks!(
                        "let ",
                        format!("data_{}", &arg.name),
                        " = try! ",
                        encoder_name.clone(),
                        ".encode(",
                        arg.name.clone(),
                        ")"
                    ));
                    method_body.push(toks!(
                        "let ",
                        format!("s_{}", &arg.name),
                        " = String(data: ",
                        format!("data_{}", &arg.name),
                        ", encoding: .utf8)!"
                    ))
                }
                AstType::Callback(_) => {
                    let callback = self
                        .find_callback(arg)
                        .ok_or_else(|| GenerateError("Can't find Callback".to_string()))?;
                    let arg_cb = ArgCbGen {
                        desc: self.desc,
                        arg,
                        callback,
                    }
                    .gen()?;
                    method_body.push(arg_cb);
                }
            }
        }
        Ok(())
    }

    fn find_callback(&'a self, arg: &'a ArgDesc) -> Option<&'a TraitDesc> {
        // Find the callback.
        let callbacks = self
            .callbacks
            .iter()
            .filter(|callback| callback.name == arg.ty.origin())
            .collect::<Vec<&TraitDesc>>();
        if callbacks.is_empty() {
            panic!("No Callback {} found!", arg.ty.origin());
        }

        if callbacks.len() > 1 {
            panic!("More than one Callback {} found!", arg.ty.origin());
        }

        let callback = callbacks.get(0);
        if let Some(&callback) = callback {
            Some(callback)
        } else {
            println!("Can't find Callback {}", arg.ty.origin());
            None
        }
    }

    fn fill_call_native_method(
        &self,
        method_body: &mut Tokens<Swift>,
        method: &MethodDesc,
    ) -> Result<()> {
        let method_name = format!("{}_{}", &self.desc.mod_name, &method.name);
        match method.return_type.clone() {
            AstType::Void => {
                method_body.push(toks!(method_name, "("));
            }
            _ => {
                println!("quote method call for {}", method_name);
                method_body.push(toks!("let result = ", method_name, "("));
            }
        }

        for (index, item) in method.args.clone().into_iter().enumerate() {
            let converted = format!("s_{}", &item.name);
            if index == method.args.len() - 1 {
                method_body.append(toks!(converted));
            } else {
                method_body.append(toks!(converted, ", "));
            }
        }

        method_body.append(")");
        Ok(())
    }

    fn fill_return_type_convert(
        &self,
        method_body: &mut Tokens<Swift>,
        method: &MethodDesc,
    ) -> Result<()> {
        let crate_name = self.desc.crate_name.replace('-', "_");
        match method.return_type.clone() {
            AstType::Void => {}
            AstType::Byte(_) => {
                method_body.push(toks!("let s_result = Int8(result)"));
            }
            AstType::Int(_) => {
                method_body.push(toks!("let s_result = Int32(result)"));
            }
            AstType::Long(_) => {
                method_body.push(toks!("let s_result = Int64(result)"));
            }
            AstType::Float(_) => {
                method_body.push(toks!("let s_result = Float(result)"));
            }
            AstType::Double(_) => {
                method_body.push(toks!("let s_result = Double(result)"));
            }
            AstType::Boolean => {
                method_body.push(toks!("let s_result = result > 0 ? true : false"));
            }
            AstType::String => {
                method_body.push(toks!("let s_result = String(cString:result!)"));
                method_body.push(toks!(format!("{}_free_str(result!)", &crate_name)));
            }
            AstType::Vec(_) => {
                let return_ty = SwiftType::new(method.return_type.clone());
                method_body.push(toks!("let ret_str = String(cString:result!)"));
                method_body.push(toks!(format!("{}_free_str(result!)", &crate_name)));
                method_body.push(toks!(
                    "var s_tmp_result:",
                    Swift::from(return_ty.clone()),
                    "?"
                ));
                method_body.push(toks!("autoreleasepool {"));
                method_body.nested(toks!("let ret_str_json = ret_str.data(using: .utf8)!"));
                method_body.nested(toks!("let decoder = JSONDecoder()"));
                method_body.nested(toks!(
                    "s_tmp_result = try! decoder.decode(",
                    Swift::from(return_ty),
                    ".self, from: ret_str_json)"
                ));
                method_body.push(toks!("}"));
                method_body.push(toks!("let s_result = s_tmp_result!"));
            }
            AstType::Callback(_) => {}
            AstType::Struct(struct_name) => {
                method_body.push(toks!("let ret_str = String(cString:result!)"));
                method_body.push(toks!(format!("{}_free_str(result!)", &crate_name)));
                method_body.push(toks!("var s_tmp_result: ", struct_name.clone(), "?"));
                method_body.push(toks!("autoreleasepool {"));
                method_body.nested(toks!("let ret_str_json = ret_str.data(using: .utf8)!"));
                method_body.nested(toks!("let decoder = JSONDecoder()"));
                method_body.nested(toks!(
                    "s_tmp_result = try! decoder.decode(",
                    struct_name,
                    ".self, from: ret_str_json)"
                ));
                method_body.push(toks!("}"));
                method_body.push(toks!("let s_result = s_tmp_result!"));
            }
        }

        match method.return_type.clone() {
            AstType::Void => {}
            _ => method_body.push(toks!("return s_result")),
        }
        Ok(())
    }
}
