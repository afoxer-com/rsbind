use heck::ToLowerCamelCase;
use rstgen::{IntoTokens, Java, java};
use rstgen::java::{Interface, Method, Modifier};
use crate::ast::contract::desc::TraitDesc;
use crate::errors::*;
use crate::java::types::{JavaType, to_java_file};

pub(crate) struct CallbackGen<'a> {
    pub desc: &'a TraitDesc,
    pub pkg: String,
}

impl<'a> CallbackGen<'a> {
    pub(crate) fn gen(&self) -> Result<String> {
        let mut interface = Interface::new(self.desc.name.clone());
        interface.modifiers.push(Modifier::Public);
        interface.extends = toks!(java::imported("java.io", "Serializable"));

        for method in self.desc.methods.iter() {
            let mut m = Method::new(method.name.to_lower_camel_case());
            m.modifiers = vec![];
            m.returns = Java::from(JavaType::new(method.return_type.clone(), self.pkg.clone()));
            for arg in method.args.iter() {
                let arg_ty = Java::from(JavaType::new(arg.ty.clone(), self.pkg.clone()));
                let mut argument = java::Argument::new(arg_ty, arg.name.as_ref());
                argument.modifiers = vec![];

                m.arguments.push(argument);
            }
            interface.methods.push(m);
        }

        to_java_file(self.pkg.as_ref(), interface.into_tokens())
    }
}
