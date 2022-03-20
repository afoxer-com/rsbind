use heck::ToLowerCamelCase;
use rstgen::{IntoTokens, swift, Tokens};
use rstgen::swift::Swift;

use crate::ast::contract::desc::{MethodDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::errors::*;
use crate::swift::mapping::SwiftMapping;

/// Generate callback Return handling for swift code.
pub(crate) struct ReturnCbGen<'a> {
    pub(crate) desc: &'a TraitDesc,
    pub(crate) method: &'a MethodDesc,
    pub(crate) callback: &'a TraitDesc,
}

impl<'a> ReturnCbGen<'a> {
    pub(crate) fn gen<'b>(&self) -> Result<Tokens<'b, Swift<'b>>> {
        let mut body = Tokens::new();

        let class_name = format!("Return{}", self.callback.name);
        let mut class = swift::Class::new(class_name.clone());
        class.modifiers = vec![];
        class
            .implements
            .push(swift::local(self.callback.name.clone()));

        let callback_model_str =
            format!("{}_{}_Model", &self.callback.mod_name, &self.callback.name);
        let mut proxy_field = swift::Field::new(swift::local(callback_model_str.clone()), "proxy");
        proxy_field.modifiers = vec![swift::Modifier::Private];
        class.fields.push(proxy_field);

        let mut constructor = swift::Constructor::new();
        let constructor_arg =
            swift::Argument::new(swift::local(callback_model_str), "proxy");
        constructor.arguments.push(constructor_arg);
        constructor.body.push(toks!("self.proxy = proxy"));
        class.constructors.push(constructor);

        for method in self.callback.methods.iter() {
            let mut cls_method = swift::Method::new(method.name.to_lower_camel_case());
            for arg in method.args.iter() {
                let cls_method_arg = swift::Argument::new(
                    SwiftMapping::map_swift_sig_type(&arg.ty),
                    arg.name.clone(),
                );
                cls_method.arguments.push(cls_method_arg);
            }
            cls_method.returns(SwiftMapping::map_swift_sig_type(&method.return_type));

            let mut method_body = Tokens::new();

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

            // argument convert
            for arg in method.args.iter() {
                crate::swift::artifact_s2c::fill_arg_convert(&mut method_body, arg)?;
            }

            // call native method
            self.fill_call_native_method(&mut method_body, method)?;

            // return convert
            crate::swift::artifact_s2c::fill_return_type_convert(
                &mut method_body,
                &method.return_type,
                &self.callback.crate_name,
            )?;

            for _i in 0..byte_count {
                method_body.push("}");
            }

            cls_method.body = method_body;
            class.methods.push(cls_method);
        }
        body.push(class.into_tokens());
        body.push(toks!("return ", class_name, "(proxy: result)"));
        Ok(body)
    }

    fn fill_call_native_method(
        &self,
        method_body: &mut Tokens<Swift>,
        method: &MethodDesc,
    ) -> Result<()> {
        let method_name = method.name.clone();
        match method.return_type.clone() {
            AstType::Void => {
                method_body.push(toks!("self.proxy.", method_name, "("));
            }
            _ => {
                println!("quote method call for {}", method_name);
                method_body.push(toks!("let result = self.proxy.", method_name, "("));
            }
        }

        method_body.append(toks!("self.proxy.index"));
        if !method.args.is_empty() {
            method_body.append(toks!(","));
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
}
