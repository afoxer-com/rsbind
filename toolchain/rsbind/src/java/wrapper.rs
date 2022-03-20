use heck::ToLowerCamelCase;
use rstgen::{IntoTokens, java, Java, Tokens};
use rstgen::java::{Argument, Class, Constructor, Method, Modifier};

use crate::ast::contract::desc::{MethodDesc, TraitDesc};
use crate::ast::types::AstType;
use crate::errors::*;
use crate::java::types::{JavaType, to_java_file};

pub(crate) struct WrapperGen<'a> {
    pub desc: &'a TraitDesc,
    pub pkg: String,
}

impl<'a> WrapperGen<'a> {
    /// Generate Java Code for rust trait.
    pub(crate) fn gen(&self) -> Result<String> {
        let outer_class_name = format!("Rust{}", &self.desc.name);
        let mut outer_class = Class::new(outer_class_name);
        outer_class.modifiers = vec![Modifier::Public];
        outer_class
            .implements
            .push(java::imported(self.pkg.clone(), self.desc.name.clone()));

        let mut constructor = Constructor::new();
        constructor.modifiers = vec![];
        outer_class.constructors.push(constructor);

        let methods = self.desc.methods.clone();
        for method in methods.into_iter() {
            let mut outer_method = self.fill_method_sig(&method)?;
            let mut outer_method_body: Tokens<Java> = Tokens::new();
            let inner_class_name = format!("Internal{}", &self.desc.name);
            self.fill_outer_method_body(inner_class_name.clone(), &mut outer_method_body, &method)?;
            outer_method.body = outer_method_body;
            outer_class.methods.push(outer_method)
        }

        to_java_file(self.pkg.as_ref(), outer_class.into_tokens())
    }

    fn fill_method_sig(&self, method: &MethodDesc) -> Result<Method> {
        let mut m = java::Method::new(method.name.to_lower_camel_case());
        m.modifiers = vec![Modifier::Public];
        let return_ty = JavaType::new(method.return_type.clone(), self.pkg.clone());
        m.returns = Java::from(return_ty);

        for arg in method.args.clone().into_iter() {
            // Add arguments
            match arg.ty {
                AstType::Void => (),
                _ => {
                    let java = JavaType::new(arg.ty.clone(), self.pkg.clone());
                    let mut argument = Argument::new(java, arg.name.clone());
                    argument.modifiers = vec![];
                    m.arguments.push(argument);
                }
            }
        }
        Ok(m)
    }

    fn fill_outer_method_body(
        &self,
        inner_cls_name: String,
        method_body: &mut Tokens<Java>,
        method: &MethodDesc,
    ) -> Result<()> {
        match method.return_type.clone() {
            AstType::Void => {
                method_body.push(toks!(
                    inner_cls_name,
                    ".",
                    method.name.to_lower_camel_case(),
                    "("
                ));
            }
            _ => {
                method_body.push(toks!(
                    "return ",
                    inner_cls_name,
                    ".",
                    method.name.to_lower_camel_case(),
                    "("
                ));
            }
        }

        for (index, item) in method.args.clone().into_iter().enumerate() {
            if index == method.args.len() - 1 {
                method_body.append(toks!(item.name.clone()));
            } else {
                method_body.append(toks!(item.name.clone(), ", "));
            }
        }
        method_body.append(toks!(");"));
        Ok(())
    }
}
