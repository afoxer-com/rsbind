use heck::ToLowerCamelCase;
use rstgen::java::{Argument, Interface, Method, Modifier};
use rstgen::{java, IntoTokens, Java};

use crate::ast::contract::desc::{MethodDesc, TraitDesc};
use crate::ast::types::AstType;
use crate::errors::*;
use crate::java::types::{to_java_file, JavaType};

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
            let outer_method = self.fill_method_sig(&method)?;
            interface.methods.push(outer_method)
        }

        to_java_file(self.pkg.as_ref(), interface.into_tokens())
    }

    fn fill_method_sig(&self, method: &MethodDesc) -> Result<Method> {
        let mut m = java::Method::new(method.name.to_lower_camel_case());
        m.modifiers = vec![];
        let return_ty = JavaType::new(method.return_type.clone());
        m.returns = Java::from(return_ty);

        for arg in method.args.clone().into_iter() {
            // Add arguments
            match arg.ty {
                AstType::Void => (),
                _ => {
                    let java = JavaType::new(arg.ty.clone());
                    let mut argument = Argument::new(java, arg.name.clone());
                    argument.modifiers = vec![];
                    m.arguments.push(argument);
                }
            }
        }
        Ok(m)
    }
}
