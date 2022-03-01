use std::fs;
use std::path::PathBuf;
use std::process::Command;

use rsgen::swift::Swift::Map;
use rsgen::swift::{self, *};
use rsgen::{Custom, Formatter, IntoTokens, Tokens};
use syn::token::Token;

use ast::contract::desc::{MethodDesc, StructDesc, TraitDesc};
use ast::types::{AstBaseType, AstType};
use ast::AstResult;
use errors::ErrorKind::*;
use errors::*;

pub(crate) struct SwiftCodeGen<'a> {
    pub origin_prj: &'a PathBuf,
    pub swift_gen_dir: &'a PathBuf,
    pub ast: &'a AstResult,
    pub module_name: String,
}

impl<'a> SwiftCodeGen<'a> {
    pub(crate) fn gen_swift_code(&self) -> Result<()> {
        // collect all the callbacks.
        let mut callbacks = vec![];
        for desc in self.ast.trait_descs.iter() {
            let descs = desc.1;
            for each in descs.iter() {
                if each.is_callback {
                    callbacks.push(each.clone());
                }
            }
        }

        // generate all the callbacks.
        for each in callbacks.clone().iter() {
            let gen = CallbackGen { desc: &each };

            let callback_str = gen.gen()?;
            let file_name = format!("{}.swift", &each.name);
            let callback_path = self.swift_gen_dir.clone().join(file_name);
            fs::write(callback_path, callback_str)?;
        }

        // generate all the traits.
        for desc in self.ast.trait_descs.iter() {
            let descs = desc.1;
            for each in descs.iter() {
                if !each.is_callback {
                    let gen = TraitGen {
                        desc: each,
                        callbacks: callbacks.clone(),
                    };
                    let str = gen.gen()?;
                    let file_name = format!("{}.swift", &each.name);
                    let path = self.swift_gen_dir.clone().join(file_name);
                    fs::write(path, str)?;
                }
            }
        }

        // generate all the structs
        for (_key, struct_descs) in self.ast.struct_descs.iter() {
            for struct_desc in struct_descs.iter() {
                let gen = StructGen { desc: struct_desc };

                let struct_str = gen.gen()?;
                let file_name = format!("{}.swift", &struct_desc.name);
                let path = self.swift_gen_dir.join(file_name);
                fs::write(path, struct_str)?
            }
        }

        Ok(())
    }
}

pub(crate) struct StructGen<'a> {
    pub desc: &'a StructDesc,
}

impl<'a> StructGen<'a> {
    pub(crate) fn gen(&self) -> Result<String> {
        let mut struct_ = swift::Struct::new(self.desc.name.clone());
        struct_.modifiers.push(Modifier::Public);
        struct_.implements.push(local("Codable"));

        for field in self.desc.fields.iter() {
            let field_ty = SwiftType::new(field.ty, field.origin_ty.clone());
            let mut swift_field = Field::new(Swift::from(field_ty), field.name.clone());
            swift_field.modifiers = vec![Modifier::Public];
            struct_.fields.push(swift_field);
        }

        to_swift_file(struct_.into_tokens())
    }
}

pub(crate) struct CallbackGen<'a> {
    pub desc: &'a TraitDesc,
}

impl<'a> CallbackGen<'a> {
    pub(crate) fn gen(&self) -> Result<String> {
        let mut protocol = Protocol::new(self.desc.name.clone());
        protocol.modifiers.push(Modifier::Public);

        for method in self.desc.methods.iter() {
            let mut m = Method::new(method.name.clone());
            m.modifiers = vec![];
            m.returns = Some(Swift::from(SwiftType::new(
                method.return_type,
                method.origin_return_ty.clone(),
            )));
            for arg in method.args.iter() {
                let arg_ty = Swift::from(SwiftType::new(arg.ty, arg.origin_ty.clone()));
                let argument = swift::Argument::new(arg_ty, arg.name.as_ref());
                m.arguments.push(argument)
            }
            protocol.methods.push(m);
        }

        to_swift_file(protocol.into_tokens())
    }
}

pub(crate) struct TraitGen<'a> {
    pub desc: &'a TraitDesc,
    pub callbacks: Vec<TraitDesc>,
}

impl<'a> TraitGen<'a> {
    pub(crate) fn gen(&self) -> Result<String> {
        let mut class = Class::new(self.desc.name.clone());
        class.modifiers = vec![Modifier::Public];

        let mut tokens = toks!();
        self.fill_global_block(&mut tokens);

        // let mut sel_callbacks = vec![];
        for method in self.desc.methods.iter() {
            println!("generate swift codes for {}", &method.name);
            // Method signature
            let mut m = self.fill_method_sig(&method)?;

            let mut method_body: Tokens<Swift> = Tokens::new();

            let mut byte_count = 0;
            for arg in method.args.iter() {
                if let AstType::Vec(base) = arg.ty.clone() {
                    if let AstBaseType::Byte = base.clone() {
                        byte_count = byte_count + 1;
                        method_body.push(toks!(
                            arg.name.clone(),
                            ".withUnsafeBufferPointer { ",
                            arg.name.clone(),
                            "_buffer in"
                        ));
                    }
                }
            }

            // Argument convert
            self.fill_arg_convert(&mut method_body, &method)?;

            // Call native method
            self.fill_call_native_method(&mut method_body, &method)?;

            // Return type convert
            self.fill_return_type_convert(&mut method_body, &method)?;

            for i in 0..byte_count {
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
        let mut m = Method::new(method.name.clone());
        m.modifiers = vec![Modifier::Public, Modifier::Static];
        let return_ty = SwiftType::new(method.return_type, method.origin_return_ty.clone());
        m.returns(Swift::from(return_ty.clone()));

        for arg in method.args.clone().iter() {
            match arg.ty {
                AstType::Void => (),
                _ => {
                    let swift = SwiftType::new(arg.ty.clone(), arg.origin_ty.clone());
                    let argument = swift::Argument::new(swift, arg.name.clone());
                    m.arguments.push(argument);
                }
            }
        }

        Ok(m)
    }

    fn fill_arg_convert(&self, method_body: &mut Tokens<Swift>, method: &MethodDesc) -> Result<()> {
        for arg in method.args.clone().iter() {
            // Argument convert
            println!("quote arg convert for {}", arg.name.clone());
            let s_arg_name = format!("s_{}", &arg.name);
            match arg.ty {
                AstType::Void => {}
                AstType::Boolean => method_body.push(toks!(
                    "let ",
                    s_arg_name.clone(),
                    ": Int32 = ",
                    arg.name.clone(),
                    " ? 1 : 0"
                )),
                AstType::Vec(base) => {
                    if let AstBaseType::Byte = base {
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
                    // } else if arg.origin_ty.clone().contains("u8") {
                    //     let arg_buffer_name = format!("{}_buffer", &arg.name);
                    //     method_body.push(toks!(
                    //         "let ",
                    //         s_arg_name.clone(),
                    //         " = CUInt8Array(ptr: ",
                    //         arg_buffer_name.clone(),
                    //         ".baseAddress, len: Int32(",
                    //         arg_buffer_name.clone(),
                    //         ".count))"
                    //     ))
                    } else {
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
                }
                AstType::Byte => method_body.push(toks!(
                    "let ",
                    s_arg_name.clone(),
                    " = Int8(",
                    arg.name.clone(),
                    ")"
                )),
                AstType::Int => method_body.push(toks!(
                    "let ",
                    s_arg_name.clone(),
                    " = Int32(",
                    arg.name.clone(),
                    ")"
                )),
                AstType::Long => method_body.push(toks!(
                    "let ",
                    s_arg_name.clone(),
                    " = Int64(",
                    arg.name.clone(),
                    ")"
                )),
                AstType::Float => method_body.push(toks!(
                    "let ",
                    s_arg_name.clone(),
                    " = Float32(",
                    arg.name.clone(),
                    ")"
                )),
                AstType::Double => method_body.push(toks!(
                    "let ",
                    s_arg_name.clone(),
                    " = Float64(",
                    arg.name.clone(),
                    ")"
                )),
                AstType::String => {
                    method_body.push(toks!("let ", s_arg_name.clone(), " = ", arg.name.clone()))
                }
                AstType::Struct => {}
                AstType::Callback => {
                    // Store the callback to global callback map.
                    let index_name = format!("{}_index", &arg.name);
                    method_body.push(toks!("let ", index_name.clone(), " = globalIndex + 1"));
                    method_body.push(toks!("globalIndex = ", index_name.clone()));
                    method_body.push(toks!(
                        "globalCallbacks[",
                        index_name.clone(),
                        "] = ",
                        arg.name.clone()
                    ));

                    // Find the callback.
                    let callbacks = self
                        .callbacks
                        .iter()
                        .filter(|callback| callback.name == arg.origin_ty)
                        .collect::<Vec<&TraitDesc>>();
                    if callbacks.len() <= 0 {
                        panic!("No Callback {} found!", arg.origin_ty.clone());
                    }

                    if callbacks.len() > 1 {
                        panic!("More than one Callback {} found!", arg.origin_ty.clone());
                    }

                    let cb;
                    let callback = callbacks.get(0);
                    if let Some(callback) = callback {
                        cb = callback;
                    } else {
                        panic!("Can't find Callback {}", arg.origin_ty.clone());
                    }

                    let mut cb_args_model = "".to_string();
                    for cb_method in cb.methods.iter() {
                        let mut arg_params = "(index".to_owned();
                        let mut args_str = "(Int64".to_owned();
                        for cb_arg in cb_method.args.iter() {
                            let cb_arg_ty = map_cb_type(&cb_arg.ty);
                            arg_params = format!("{}, {}", &arg_params, &cb_arg.name);
                            args_str = format!("{}, {}", &args_str, &cb_arg_ty);
                        }
                        arg_params = format!("{})", &arg_params);
                        args_str = format!("{})", &args_str);

                        let cb_return_ty = map_cb_type(&cb_method.return_type);
                        let closure = format!("{} -> {}", &args_str, &cb_return_ty);
                        arg_params = format!("{} -> {}", &arg_params, &cb_return_ty);

                        method_body.push(toks!(
                            "let ",
                            format!("{}_{}", &arg.name, &cb_method.name),
                            ": @convention(c) ",
                            closure,
                            " = {"
                        ));
                        method_body.push(toks!(
                            arg_params.clone(),
                            " in\n",
                            "let ",
                            format!("{}_callback", &arg.name),
                            " = globalCallbacks[index] as! ",
                            cb.name.clone()
                        ));

                        let mut cb_method_call = "(".to_string();
                        for (index, cb_arg) in cb_method.args.iter().enumerate() {
                            let cb_arg_str = SwiftType {
                                ast_type: cb_arg.ty.clone(),
                                origin_ty: cb_arg.origin_ty.clone(),
                            }
                            .to_str();
                            match cb_arg.ty.clone() {
                                AstType::Void => {}
                                AstType::Byte => {
                                    method_body.push(toks!(
                                        "let ",
                                        format!("c_{}", &cb_arg.name),
                                        " = Int8(",
                                        cb_arg.name.clone(),
                                        ")"
                                    ));
                                }
                                AstType::Int => {
                                    method_body.push(toks!(
                                        "let ",
                                        format!("c_{}", &cb_arg.name),
                                        " = Int32(",
                                        cb_arg.name.clone(),
                                        ")"
                                    ));
                                }
                                AstType::Long => {
                                    method_body.push(toks!(
                                        "let ",
                                        format!("c_{}", &cb_arg.name),
                                        " = Int64(",
                                        cb_arg.name.clone(),
                                        ")"
                                    ));
                                }
                                AstType::Float => {
                                    method_body.push(toks!(
                                        "let ",
                                        format!("c_{}", &cb_arg.name),
                                        " = Float(",
                                        cb_arg.name.clone(),
                                        ")"
                                    ));
                                }
                                AstType::Double => {
                                    method_body.push(toks!(
                                        "let ",
                                        format!("c_{}", &cb_arg.name),
                                        " = Double(",
                                        cb_arg.name.clone(),
                                        ")"
                                    ));
                                }
                                AstType::Boolean => {
                                    method_body.push(toks!(
                                        "let ",
                                        format!("c_{}", &cb_arg.name),
                                        " : Bool = ",
                                        cb_arg.name.clone(),
                                        " > 0 ? true : false"
                                    ));
                                }
                                AstType::String => {
                                    method_body.push(toks!(
                                        "let ",
                                        format!("c_{}", &cb_arg.name),
                                        " = String(cString: ",
                                        cb_arg.name.clone(),
                                        "!)"
                                    ));
                                }
                                AstType::Callback => {
                                    panic!("Don't support callback argument in callback");
                                }
                                AstType::Vec(_) | AstType::Struct => {
                                    method_body.push(toks!(
                                        "let ",
                                        format!("c_tmp_{}", &cb_arg.name),
                                        " = String(cString:",
                                        cb_arg.name.clone(),
                                        "!)\n",
                                        "var ",
                                        format!("c_option_{}", &cb_arg.name),
                                        " : ",
                                        cb_arg_str.clone(),
                                        "?\n",
                                        "autoreleasepool {\n",
                                        "let ",
                                        format!("c_tmp_json_{}", &cb_arg.name),
                                        " = ",
                                        format!("c_tmp_{}", &cb_arg.name),
                                        ".data(using: .utf8)!\n",
                                        "let decoder = JSONDecoder()\n",
                                        format!("c_option_{}", &cb_arg.name),
                                        " = try! decoder.decode(",
                                        cb_arg_str.clone(),
                                        ".self, from: ",
                                        format!("c_tmp_json_{}", &cb_arg.name),
                                        ")\n",
                                        "}\n",
                                        "let ",
                                        format!("c_{}", &cb_arg.name),
                                        " = ",
                                        format!("c_option_{}", &cb_arg.name),
                                        "!"
                                    ));
                                }
                            }

                            cb_method_call =
                                format!("{}{}: c_{}", &cb_method_call, &cb_arg.name, &cb_arg.name);
                            if index != cb_method.args.len() - 1 {
                                cb_method_call = format!("{}, ", &cb_method_call);
                            }
                        }
                        cb_method_call = format!("{})", &cb_method_call);

                        match cb_method.return_type.clone() {
                            AstType::Void => {
                                method_body.push(toks!(
                                    format!("{}_callback", &arg.name),
                                    ".",
                                    cb_method.name.clone(),
                                    cb_method_call
                                ));
                            }
                            _ => {
                                method_body.push(toks!(
                                    "let result = ",
                                    format!("{}_callback", &arg.name),
                                    ".",
                                    cb_method.name.clone(),
                                    cb_method_call
                                ));
                            }
                        }

                        match cb_method.return_type.clone() {
                            AstType::Void => {}
                            AstType::Byte => {
                                method_body.push(toks!("return Int8(result)"));
                            }
                            AstType::Int => {
                                method_body.push(toks!("return Int32(result)"));
                            }
                            AstType::Long => {
                                method_body.push(toks!("return Int64(result)"));
                            }
                            AstType::Float => {
                                method_body.push(toks!("return Float(result)"));
                            }
                            AstType::Double => {
                                method_body.push(toks!("return Double(result)"));
                            }
                            AstType::Boolean => {
                                method_body.push(toks!("return result ? 1 : 0"));
                            }
                            AstType::String => {
                                method_body.push(toks!("return result"));
                            }
                            AstType::Vec(_) => {
                                panic!("Don't support Vec in callback return.")
                            }
                            AstType::Callback => {
                                panic!("Don't support Callback in callback return.")
                            }
                            AstType::Struct => {
                                panic!("Don't support Struct in callback return.")
                            }
                        }
                        method_body.push(toks!("}"));

                        cb_args_model = format!(
                            "{}{}:{}_{},",
                            cb_args_model, &cb_method.name, &arg.name, &cb_method.name
                        );
                    }

                    let free_fn_name = format!("{}_callback_free", &arg.name);
                    method_body.push(toks!(
                        "let ",
                        free_fn_name.clone(),
                        " : @convention(c)(Int64) -> () = {\n",
                        "(index) in\n",
                        "globalCallbacks.removeValue(forKey: index)\n",
                        "}\n",
                        format!(
                            "let s_{} = {}_{}_Model({}free_callback: {}, index: {}_index)\n",
                            &arg.name,
                            &self.desc.mod_name,
                            &cb.name,
                            cb_args_model,
                            &free_fn_name,
                            &arg.name
                        )
                    ));
                }
            }
        }
        Ok(())
    }

    fn fill_call_native_method(
        &self,
        method_body: &mut Tokens<Swift>,
        method: &MethodDesc,
    ) -> Result<()> {
        let method_name = format!("{}_{}", &self.desc.mod_name, &method.name);
        match method.return_type.clone() {
            AstType::Void => {
                method_body.push(toks!(method_name.clone(), "("));
            }
            _ => {
                println!("quote method call for {}", method_name.clone());
                method_body.push(toks!("let result = ", method_name.clone(), "("));
                for (index, item) in method.args.clone().into_iter().enumerate() {
                    let converted = format!("s_{}", &item.name);
                    if index == method.args.len() - 1 {
                        method_body.append(toks!(converted));
                    } else {
                        method_body.append(toks!(converted, ", "));
                    }
                }
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
        let crate_name = self.desc.crate_name.replace("-", "_");
        match method.return_type.clone() {
            AstType::Void => {}
            AstType::Byte => {
                method_body.push(toks!("let s_result = Int8(result)"));
            }
            AstType::Int => {
                method_body.push(toks!("let s_result = Int32(result)"));
            }
            AstType::Long => {
                method_body.push(toks!("let s_result = Int64(result)"));
            }
            AstType::Float => {
                method_body.push(toks!("let s_result = Float(result)"));
            }
            AstType::Double => {
                method_body.push(toks!("let s_result = Double(result)"));
            }
            AstType::Boolean => {
                method_body.push(toks!("let s_result = result > 0 ? true : false"));
            }
            AstType::String => {
                method_body.push(toks!("let s_result = String(cString:result!)"));
                method_body.push(toks!(format!("{}_free_str(result!)", &crate_name)));
            }
            AstType::Vec(base) => {
                let return_ty = SwiftType::new(method.return_type, method.origin_return_ty.clone());
                method_body.push(toks!(
                    "let ret_str = String(cString:result!)\n",
                    format!("{}_free_str(result!)\n", &crate_name),
                    "var s_tmp_result:",
                    Swift::from(return_ty.clone()),
                    "?\n",
                    "autoreleasepool {\n",
                    "let ret_str_json = ret_str.data(using: .utf8)!\n",
                    "let decoder = JSONDecoder()\n",
                    "s_tmp_result = try! decoder.decode(",
                    Swift::from(return_ty.clone()),
                    ".self, from: ret_str_json)\n",
                    "}\n",
                    "let s_result = s_tmp_result!"
                ));
            }
            AstType::Callback => {}
            AstType::Struct => {
                method_body.push(toks!(
                    "let ret_str = String(cString:result!)\n",
                    format!("{}_free_str(result!)\n", &crate_name),
                    "var s_tmp_result: ",
                    method.origin_return_ty.to_owned(),
                    "?\n",
                    "autoreleasepool {\n",
                    "let ret_str_json = ret_str.data(using: .utf8)!\n",
                    "let decoder = JSONDecoder()\n",
                    "s_tmp_result = try! decoder.decode(",
                    method.origin_return_ty.to_owned(),
                    ".self, from: ret_str_json)\n",
                    "}\n",
                    "let s_result = s_tmp_result!\n"
                ));
            }
        }

        match method.return_type.clone() {
            AstType::Void => {}
            _ => method_body.push(toks!("return s_result")),
        }
        Ok(())
    }
}

fn map_cb_type(ty: &AstType) -> String {
    match ty {
        AstType::Void => {
            return "()".to_string();
        }
        AstType::Byte => {
            return "Int8".to_string();
        }
        AstType::Int => {
            return "Int32".to_string();
        }
        AstType::Long => {
            return "Int64".to_string();
        }
        AstType::Float => {
            return "Float".to_string();
        }
        AstType::Double => {
            return "Double".to_string();
        }
        AstType::Boolean => {
            return "Int32".to_string();
        }
        AstType::String => {
            return "UnsafePointer<Int8>?".to_string();
        }
        AstType::Vec(_) => {
            return "UnsafePointer<Int8>?".to_string();
        }
        AstType::Callback => {
            panic!("Don't support callback in callback argument.");
        }
        AstType::Struct => {
            return "UnsafePointer<Int8>?".to_string();
        }
    }
}

#[derive(Clone)]
struct SwiftType {
    pub ast_type: AstType,
    pub origin_ty: String,
}

impl SwiftType {
    pub(crate) fn new(ast_type: AstType, origin_ty: String) -> SwiftType {
        SwiftType {
            ast_type,
            origin_ty,
        }
    }

    pub(crate) fn to_array(&self) -> Swift<'static> {
        let base_name = Swift::from(self.clone());
        self.to_swift_array(base_name)
    }

    pub(crate) fn to_str(&self) -> String {
        return match self.ast_type.clone() {
            AstType::Void => "Void".to_string(),
            AstType::Byte => "Int8".to_string(),
            AstType::Int => "Int32".to_string(),
            AstType::Long => "Int64".to_string(),
            AstType::Float => "Float".to_string(),
            AstType::Double => "Double".to_string(),
            AstType::Boolean => "Bool".to_string(),
            AstType::String => "String".to_string(),
            AstType::Vec(base) => {
                let sub_origin_ty = self.origin_ty.replace("Vec<", "").replace(">", "");
                let base_ty = SwiftType {
                    ast_type: AstType::from(base),
                    origin_ty: sub_origin_ty,
                };
                format!("[{}]", base_ty.to_str())
            }
            AstType::Callback => self.origin_ty.to_string(),
            AstType::Struct => self.origin_ty.to_string(),
        };
    }

    pub(crate) fn to_transfer(&self) -> Swift<'static> {
        match self.ast_type {
            AstType::Boolean => swift::INTEGER,
            AstType::Vec(base) => match base {
                AstBaseType::Byte => Swift::from(self.clone()),
                _ => swift::local("String"),
            },
            AstType::Struct => swift::local("String"),
            AstType::Callback => swift::LONG,
            _ => Swift::from(self.clone()),
        }
    }

    /// If JavaType is an Vec(base), we will return base, else we will return itself.
    pub(crate) fn get_base_ty(&self) -> Swift<'static> {
        match self.ast_type {
            AstType::Vec(base) => match base {
                AstBaseType::Struct => {
                    let sub_origin_ty = self.origin_ty.replace("Vec<", "").replace(">", "");
                    swift::local(sub_origin_ty)
                }
                _ => Swift::from(SwiftType::new(AstType::from(base), self.origin_ty.clone())),
            },
            _ => Swift::from(self.clone()),
        }
    }

    fn to_swift_array(&self, swift: Swift<'static>) -> Swift<'static> {
        Swift::Array {
            inner: Box::new(swift),
        }
    }
}

impl From<SwiftType> for Swift<'static> {
    fn from(item: SwiftType) -> Self {
        match item.ast_type {
            AstType::Void => swift::VOID,
            AstType::Byte => swift::BYTE,
            AstType::Int => swift::INTEGER,
            AstType::Long => swift::LONG,
            AstType::Float => swift::FLOAT,
            AstType::Double => swift::DOUBLE,
            AstType::Boolean => swift::BOOLEAN,
            AstType::String => swift::local("String"),
            AstType::Vec(base) => match base {
                AstBaseType::Struct => {
                    let sub_origin_ty = item.origin_ty.replace("Vec<", "").replace(">", "");
                    SwiftType::new(AstType::from(base), sub_origin_ty.clone()).to_array()
                }
                AstBaseType::Byte => {
                    SwiftType::new(AstType::from(base), item.origin_ty.clone()).to_array()
                }
                _ => SwiftType::new(AstType::from(base), item.origin_ty.clone()).to_array(),
            },
            AstType::Callback | AstType::Struct => swift::local(item.origin_ty.clone()),
        }
    }
}

fn to_swift_file(tokens: Tokens<Swift>) -> Result<String> {
    let mut buf = String::new();
    {
        let mut formatter = Formatter::new(&mut buf);
        let mut extra = ();
        swift::Swift::write_file(tokens, &mut formatter, &mut extra, 0);
    }
    Ok(buf)
}
