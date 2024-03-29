use heck::ToLowerCamelCase;
use rstgen::swift::{self, *};
use rstgen::{IntoTokens, Tokens};

use crate::ast::contract::desc::{MethodDesc, TraitDesc};
use crate::ast::types::AstType;
use crate::base::lang::Convertible;
use crate::errors::*;
use crate::swift::converter::SwiftConvert;
use crate::swift::types::to_swift_file;

pub(crate) struct WrapperGen<'a> {
    pub desc: &'a TraitDesc,
}

impl<'a> WrapperGen<'a> {
    pub fn gen(&self) -> Result<String> {
        let inner_cls = format!("Internal{}", &self.desc.name);
        let wrapper_cls = format!("Rust{}", &self.desc.name);
        let mut class = Class::new(wrapper_cls);
        class.modifiers = vec![Modifier::Public];
        class.implements.push(swift::local(self.desc.name.clone()));

        let mut constructor = Constructor::new();
        constructor.modifiers = vec![Modifier::Internal];
        class.constructors.push(constructor);

        // let mut sel_callbacks = vec![];
        for method in self.desc.methods.iter() {
            println!("generate swift protocol method for {}", &method.name);
            // Method signature
            let mut m = self.fill_method_sig(method)?;
            let mut body: Tokens<Swift> = Tokens::new();
            self.fill_call_internal_method(inner_cls.clone(), &mut body, method)?;
            m.body = body;
            class.methods.push(m);
        }

        to_swift_file(class.into_tokens())
    }

    fn fill_method_sig(&self, method: &MethodDesc) -> Result<Method> {
        let mut m = Method::new(method.name.to_lower_camel_case());
        m.modifiers = vec![Modifier::Public];
        m.returns(
            SwiftConvert {
                ty: method.return_type.clone(),
            }
            .native_type(),
        );

        let args = method.args.clone();
        for arg in args.iter() {
            let argument = swift::Argument::new(
                SwiftConvert { ty: arg.ty.clone() }.native_type(),
                arg.name.clone(),
            );
            m.arguments.push(argument);
        }

        Ok(m)
    }

    fn fill_call_internal_method(
        &self,
        inner_cls_name: String,
        method_body: &mut Tokens<Swift>,
        method: &MethodDesc,
    ) -> Result<()> {
        match method.return_type.clone() {
            AstType::Void => {
                push_f!(
                    method_body,
                    "{}.{}(",
                    inner_cls_name,
                    method.name.to_lower_camel_case(),
                );
            }
            _ => {
                push_f!(
                    method_body,
                    "return {}.{}(",
                    inner_cls_name,
                    method.name.to_lower_camel_case(),
                );
            }
        }

        for (index, item) in method.args.clone().into_iter().enumerate() {
            if index == method.args.len() - 1 {
                method_body.append(toks_f!("{} : {}", item.name, item.name));
            } else {
                method_body.append(toks_f!("{} : {},", item.name, item.name));
            }
        }

        method_body.append(")");
        Ok(())
    }
}
