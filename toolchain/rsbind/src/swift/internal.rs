use heck::ToLowerCamelCase;
use rstgen::{IntoTokens, Tokens};
use rstgen::swift::{self, *};
use crate::ast::contract::desc::{MethodDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::ErrorKind::GenerateError;
use crate::errors::*;
use crate::swift::arg_cb::ArgCbGen;
use crate::swift::mapping::SwiftMapping;
use crate::swift::return_cb::ReturnCbGen;
use crate::swift::types::{to_swift_file};

pub(crate) struct TraitGen<'a> {
    pub desc: &'a TraitDesc,
    pub callbacks: Vec<TraitDesc>,
}

impl<'a> TraitGen<'a> {
    pub fn gen(&'a self) -> Result<String> {
        let class_name = format!("Internal{}", &self.desc.name);
        let mut class = Class::new(class_name);
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
                match arg.ty.clone() {
                    AstType::Vec(AstBaseType::Byte(_))
                    | AstType::Vec(AstBaseType::Short(_))
                    | AstType::Vec(AstBaseType::Int(_))
                    | AstType::Vec(AstBaseType::Long(_)) => {
                        byte_count += 1;
                        method_body.push(toks!(
                            arg.name.clone(),
                            ".withUnsafeBufferPointer { ",
                            arg.name.clone(),
                            "_buffer in"
                        ));
                    }
                    _ => {}
                }
            }

            self.fill_arg_convert(&mut method_body, method)?;
            self.fill_call_native_method(&mut method_body, method)?;
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
        m.returns(SwiftMapping::map_swift_sig_type(&method.return_type));

        let args = method.args.clone();
        for arg in args.iter() {
            let argument =
                swift::Argument::new(SwiftMapping::map_swift_sig_type(&arg.ty), arg.name.clone());
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
            match arg.ty.clone() {
                AstType::Callback(_) => {
                    let callback = self
                        .find_callback(&arg.ty.origin())
                        .ok_or_else(|| GenerateError("Can't find Callback".to_string()))?;
                    let arg_cb = ArgCbGen {
                        desc: self.desc,
                        arg,
                        callback,
                    }
                    .gen()?;
                    method_body.push(arg_cb);
                }
                _ => {
                    crate::swift::artifact_s2c::fill_arg_convert(method_body, arg)?;
                }
            }
        }
        Ok(())
    }

    fn find_callback(&self, origin: &str) -> Option<&TraitDesc> {
        // Find the callback.
        let callbacks = self
            .callbacks
            .iter()
            .filter(|callback| callback.name == *origin)
            .collect::<Vec<&TraitDesc>>();
        if callbacks.is_empty() {
            panic!("No Callback {} found!", origin);
        }

        if callbacks.len() > 1 {
            panic!("More than one Callback {} found!", origin);
        }

        let callback = callbacks.get(0);
        if let Some(&callback) = callback {
            Some(callback)
        } else {
            println!("Can't find Callback {}", origin);
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
        method: &'a MethodDesc,
    ) -> Result<()> {
        match method.return_type.clone() {
            AstType::Callback(_) => {
                let origin = self
                    .find_callback(&method.return_type.origin())
                    .ok_or_else(|| GenerateError("Can't find callback".to_string()))?;
                let ret = ReturnCbGen {
                    desc: self.desc,
                    method,
                    callback: origin,
                }
                .gen()?;
                method_body.push(ret);
            }
            _ => {
                crate::swift::artifact_s2c::fill_return_type_convert(
                    method_body,
                    &method.return_type,
                    &self.desc.crate_name,
                )?;
            }
        }
        Ok(())
    }
}
