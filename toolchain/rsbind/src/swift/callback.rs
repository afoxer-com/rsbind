use rstgen::{IntoTokens, swift};
use rstgen::swift::{Method, Modifier, Protocol, Swift};
use crate::ast::contract::desc::TraitDesc;
use crate::errors::*;
use crate::swift::types::{SwiftType, to_swift_file};

pub(crate) struct CallbackGen<'a> {
    pub desc: &'a TraitDesc,
}

impl<'a> CallbackGen<'a> {
    pub fn gen(&self) -> Result<String> {
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