use std::fs;
use std::path::PathBuf;

use rstgen::swift::{self, *};
use rstgen::{Custom, Formatter, IntoTokens, Tokens};

use crate::ast::contract::desc::{ArgDesc, MethodDesc, StructDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::ast::AstResult;
use crate::errors::*;
use crate::ios::mapping::SwiftMapping;

pub(crate) struct SwiftCodeGen<'a> {
    pub swift_gen_dir: &'a PathBuf,
    pub ast: &'a AstResult,
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
            let gen = CallbackGen { desc: each };

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
            let field_ty = SwiftType::new(field.ty.clone());
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
            m.returns = Some(Swift::from(SwiftType::new(method.return_type.clone())));
            for arg in method.args.iter() {
                let arg_ty = Swift::from(SwiftType::new(arg.ty.clone()));
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
        self.fill_global_block(&mut tokens)?;

        // let mut sel_callbacks = vec![];
        for method in self.desc.methods.iter() {
            println!("generate swift codes for {}", &method.name);
            // Method signature
            let mut m = self.fill_method_sig(method)?;

            let mut method_body: Tokens<Swift> = Tokens::new();

            let mut byte_count = 0;
            for arg in method.args.iter() {
                if let AstType::Vec(base) = arg.ty.clone() {
                    if let AstBaseType::Byte(_) = base.clone() {
                        byte_count += 1;
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
        let mut m = Method::new(method.name.clone());
        m.modifiers = vec![Modifier::Public, Modifier::Static];
        m.returns(SwiftMapping::map_sig_type(&method.return_type));

        let args = method.args.clone();
        for arg in args.iter() {
            let argument =
                swift::Argument::new(SwiftMapping::map_sig_type(&arg.ty), arg.name.clone());
            m.arguments.push(argument);
        }

        Ok(m)
    }

    fn fill_arg_convert(&self, method_body: &mut Tokens<Swift>, method: &MethodDesc) -> Result<()> {
        for arg in method.args.clone().iter() {
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
                AstType::Vec(base) => {
                    if let AstBaseType::Byte(_) = base {
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
                AstType::Struct(_) => {}
                AstType::Callback(_) => {
                    // Store the callback to global callback map.
                    self.fill_callback_index(arg, method_body)?;

                    let cb = self
                        .find_callback(arg)
                        .ok_or(format!("Could not find callback for {}", &arg.name))?;

                    let mut cb_args_model = "".to_string();
                    for cb_method in cb.methods.iter() {
                        self.fill_cb_closure_method_sig(cb_method, arg, method_body)?;

                        method_body.push(toks!(
                            "let ",
                            format!("{}_callback", &arg.name),
                            " = globalCallbacks[index] as! ",
                            cb.name.clone()
                        ));

                        for cb_arg in cb_method.args.iter() {
                            self.fill_cb_closure_arg_convert(cb_arg, method_body)?;
                        }

                        self.fill_cb_closure_call(cb_method, arg, method_body)?;

                        self.fill_cb_closure_return_convert(cb_method, method_body)?;

                        method_body.push(toks!("}"));

                        cb_args_model = format!(
                            "{}{}:{}_{},",
                            cb_args_model, &cb_method.name, &arg.name, &cb_method.name
                        );
                    }
                    self.fill_cb_closure_free_fn(arg, method_body)?;

                    let free_fn_name = format!("{}_callback_free", &arg.name);
                    method_body.push(toks!(format!(
                        "let s_{} = {}_{}_Model({}free_callback: {}, index: {}_index)\n",
                        &arg.name,
                        &self.desc.mod_name,
                        &cb.name,
                        cb_args_model,
                        &free_fn_name,
                        &arg.name
                    )));
                }
            }
        }
        Ok(())
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

    fn find_callback(&self, arg: &ArgDesc) -> Option<&TraitDesc> {
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

    fn fill_cb_closure_method_sig(
        &self,
        cb_method: &MethodDesc,
        arg: &ArgDesc,
        method_body: &mut Tokens<Swift>,
    ) -> Result<()> {
        let mut arg_params = "(index".to_owned();
        let mut args_str = "(Int64".to_owned();
        for cb_arg in cb_method.args.iter() {
            let cb_arg_ty = SwiftMapping::map_cb_closure_sig_type(&cb_arg.ty);
            arg_params = format!("{}, {}", &arg_params, &cb_arg.name);
            args_str = format!("{}, {}", &args_str, &cb_arg_ty);
        }
        arg_params = format!("{})", &arg_params);
        args_str = format!("{})", &args_str);

        let cb_return_ty = SwiftMapping::map_cb_closure_sig_type(&cb_method.return_type);
        let closure = format!("{} -> {}", &args_str, &cb_return_ty);
        arg_params = format!("{} -> {}", &arg_params, &cb_return_ty);

        method_body.push(toks!(
            "let ",
            format!("{}_{}", &arg.name, &cb_method.name),
            ": @convention(c) ",
            closure,
            " = {"
        ));
        method_body.push(toks!(arg_params, " in\n"));
        Ok(())
    }

    fn fill_cb_closure_arg_convert(
        &self,
        cb_arg: &ArgDesc,
        method_body: &mut Tokens<Swift>,
    ) -> Result<()> {
        let cb_arg_str = SwiftType {
            ast_type: cb_arg.ty.clone(),
        }
        .to_str();
        match cb_arg.ty.clone() {
            AstType::Void => {}
            AstType::Byte(_) => {
                method_body.push(toks!(
                    "let ",
                    format!("c_{}", &cb_arg.name),
                    " = Int8(",
                    cb_arg.name.clone(),
                    ")"
                ));
            }
            AstType::Int(_) => {
                method_body.push(toks!(
                    "let ",
                    format!("c_{}", &cb_arg.name),
                    " = Int32(",
                    cb_arg.name.clone(),
                    ")"
                ));
            }
            AstType::Long(_) => {
                method_body.push(toks!(
                    "let ",
                    format!("c_{}", &cb_arg.name),
                    " = Int64(",
                    cb_arg.name.clone(),
                    ")"
                ));
            }
            AstType::Float(_) => {
                method_body.push(toks!(
                    "let ",
                    format!("c_{}", &cb_arg.name),
                    " = Float(",
                    cb_arg.name.clone(),
                    ")"
                ));
            }
            AstType::Double(_) => {
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
            AstType::Callback(_) => {
                panic!("Don't support callback argument in callback");
            }
            AstType::Vec(base) => match base {
                AstBaseType::Byte(_) => method_body.push(toks!(
                    "let ",
                    format!("c_{}", &cb_arg.name),
                    " = Array<Int8>(UnsafeBufferPointer(start: ",
                    cb_arg.name.clone(),
                    ".ptr, count: Int(",
                    cb_arg.name.clone(),
                    ".len)))"
                )),
                _ => {
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
                        cb_arg_str,
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
            },
            AstType::Struct(_) => {
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
                    cb_arg_str,
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

        Ok(())
    }

    fn fill_cb_closure_return_convert(
        &self,
        cb_method: &MethodDesc,
        method_body: &mut Tokens<Swift>,
    ) -> Result<()> {
        match cb_method.return_type.clone() {
            AstType::Void => {}
            AstType::Byte(_) => {
                method_body.push(toks!("return Int8(result)"));
            }
            AstType::Int(_) => {
                method_body.push(toks!("return Int32(result)"));
            }
            AstType::Long(_) => {
                method_body.push(toks!("return Int64(result)"));
            }
            AstType::Float(_) => {
                method_body.push(toks!("return Float(result)"));
            }
            AstType::Double(_) => {
                method_body.push(toks!("return Double(result)"));
            }
            AstType::Boolean => {
                method_body.push(toks!("return result ? 1 : 0"));
            }
            AstType::String => {
                method_body.push(toks!("return result"));
            }
            AstType::Vec(_) => {
                panic!("Don't support Vec in callback return.");
            }
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
            " : @convention(c)(Int64) -> () = {\n",
            "(index) in\n",
            "globalCallbacks.removeValue(forKey: index)\n",
            "}\n"
        ));
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
                method_body.push(toks!(method_name, "("));
            }
            _ => {
                println!("quote method call for {}", method_name);
                method_body.push(toks!("let result = ", method_name, "("));
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
                    Swift::from(return_ty),
                    ".self, from: ret_str_json)\n",
                    "}\n",
                    "let s_result = s_tmp_result!"
                ));
            }
            AstType::Callback(_) => {}
            AstType::Struct(struct_name) => {
                method_body.push(toks!(
                    "let ret_str = String(cString:result!)\n",
                    format!("{}_free_str(result!)\n", &crate_name),
                    "var s_tmp_result: ",
                    struct_name.clone(),
                    "?\n",
                    "autoreleasepool {\n",
                    "let ret_str_json = ret_str.data(using: .utf8)!\n",
                    "let decoder = JSONDecoder()\n",
                    "s_tmp_result = try! decoder.decode(",
                    struct_name,
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

#[derive(Clone)]
struct SwiftType {
    pub ast_type: AstType,
}

impl SwiftType {
    pub(crate) fn new(ast_type: AstType) -> SwiftType {
        SwiftType { ast_type }
    }

    pub(crate) fn to_array(&self) -> Swift<'static> {
        let base_name = Swift::from(self.clone());
        self.to_swift_array(base_name)
    }

    pub(crate) fn to_str(&self) -> String {
        return match self.ast_type.clone() {
            AstType::Void => "Void".to_string(),
            AstType::Byte(_) => "Int8".to_string(),
            AstType::Int(_) => "Int32".to_string(),
            AstType::Long(_) => "Int64".to_string(),
            AstType::Float(_) => "Float".to_string(),
            AstType::Double(_) => "Double".to_string(),
            AstType::Boolean => "Bool".to_string(),
            AstType::String => "String".to_string(),
            AstType::Vec(ref base) => {
                let base_ty = SwiftType {
                    ast_type: AstType::from(base.clone()),
                };
                format!("[{}]", base_ty.to_str())
            }
            AstType::Callback(origin) => origin,
            AstType::Struct(origin) => origin,
        };
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
            AstType::Byte(_) => swift::BYTE,
            AstType::Int(_) => swift::INTEGER,
            AstType::Long(_) => swift::LONG,
            AstType::Float(_) => swift::FLOAT,
            AstType::Double(_) => swift::DOUBLE,
            AstType::Boolean => swift::BOOLEAN,
            AstType::String => swift::local("String"),
            AstType::Vec(base) => match base {
                AstBaseType::Struct(_) => SwiftType::new(AstType::from(base)).to_array(),
                AstBaseType::Byte(_) => SwiftType::new(AstType::from(base)).to_array(),
                _ => SwiftType::new(AstType::from(base)).to_array(),
            },
            AstType::Callback(origin) | AstType::Struct(origin) => swift::local(origin),
        }
    }
}

fn to_swift_file(tokens: Tokens<Swift>) -> Result<String> {
    let mut buf = String::new();
    {
        let mut formatter = Formatter::new(&mut buf);
        let mut extra = ();
        swift::Swift::write_file(tokens, &mut formatter, &mut extra, 0)?;
    }
    Ok(buf)
}
