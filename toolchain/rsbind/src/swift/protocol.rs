use heck::ToLowerCamelCase;
use rstgen::{IntoTokens};
use rstgen::swift::{self, *};
use crate::ast::contract::desc::{MethodDesc, TraitDesc};
use crate::errors::*;
use crate::swift::mapping::SwiftMapping;
use crate::swift::types::{to_swift_file};

pub(crate) struct ProtocolGen<'a> {
    pub desc: &'a TraitDesc,
    pub callbacks: Vec<TraitDesc>,
}

impl<'a> ProtocolGen<'a> {
    pub fn gen(&self) -> Result<String> {
        let mut class = Protocol::new(self.desc.name.clone());
        class.modifiers = vec![Modifier::Public];

        let mut tokens = toks!();

        // let mut sel_callbacks = vec![];
        for method in self.desc.methods.iter() {
            println!("generate swift protocol method for {}", &method.name);
            // Method signature
            let m = self.fill_method_sig(method)?;
            class.methods.push(m);
        }

        tokens.push(class.into_tokens());

        to_swift_file(tokens)
    }

    fn fill_method_sig(&self, method: &MethodDesc) -> Result<Method> {
        let mut m = Method::new(method.name.to_lower_camel_case());
        m.modifiers = vec![];
        m.returns(SwiftMapping::map_swift_sig_type(&method.return_type));

        let args = method.args.clone();
        for arg in args.iter() {
            let argument =
                swift::Argument::new(SwiftMapping::map_swift_sig_type(&arg.ty), arg.name.clone());
            m.arguments.push(argument);
        }

        Ok(m)
    }
}
