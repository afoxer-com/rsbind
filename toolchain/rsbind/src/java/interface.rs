use rstgen::{IntoTokens, java, Java, Tokens};
use rstgen::java::{Argument, Class, Constructor, Interface, Method, Modifier};
use crate::ast::contract::desc::{MethodDesc, TraitDesc};
use crate::ast::types::AstType;
use crate::errors::*;
use crate::java::types::{JavaType, to_java_file};
use heck::ToLowerCamelCase;

pub(crate) struct InterfaceGen<'a> {
    pub desc: &'a TraitDesc,
    pub pkg: String,
}

impl<'a> InterfaceGen<'a> {
    /// Generate Java Code for rust trait.
    pub(crate) fn gen(&self) -> Result<String> {
        let mut interface = Interface::new(self.desc.name.clone());
        interface.modifiers = vec![Modifier::Public];

        let methods = self.desc.methods.clone();
        for method in methods.into_iter() {
            let mut outer_method = self.fill_method_sig(&method)?;
            interface.methods.push(outer_method)
        }

        to_java_file(self.pkg.as_ref(), interface.into_tokens())
    }

    fn fill_method_sig(&self, method: &MethodDesc) -> Result<Method> {
        let mut m = java::Method::new(method.name.to_lower_camel_case());
        m.modifiers = vec![];
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
}