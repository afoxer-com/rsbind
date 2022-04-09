use heck::ToLowerCamelCase;
use rstgen::swift::{self, *};
use rstgen::{IntoTokens, Tokens};

use crate::ast::contract::desc::{MethodDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::errors::*;
use crate::swift::mapping::SwiftMapping;
use crate::swift::types::to_swift_file;

pub(crate) struct TraitGen<'a> {
    pub desc: &'a TraitDesc,
    pub callbacks: &'a Vec<&'a TraitDesc>,
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
            self.fill_arg_convert(&mut method_body, method)?;
            self.fill_call_native_method(&mut method_body, method)?;
            self.fill_return_type_convert(&mut method_body, method, self.callbacks)?;

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

    fn fill_arg_convert<'b>(
        &'a self,
        method_body: &'b mut Tokens<'a, Swift<'a>>,
        method: &'a MethodDesc,
    ) -> Result<()> {
        for arg in method.args.iter() {
            // Argument convert
            crate::swift::artifact_s2r::fill_arg_convert(method_body, arg, self.callbacks)?;
        }
        Ok(())
    }

    fn fill_call_native_method(
        &self,
        method_body: &mut Tokens<Swift>,
        method: &MethodDesc,
    ) -> Result<()> {
        let method_name = format!(
            "{}_{}_{}",
            &self.desc.mod_name, &self.desc.name, &method.name
        );
        println!("quote method call for {}", method_name);
        push!(method_body, "let result = ", method_name, "(");

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
        method_body: &mut Tokens<'a, Swift<'a>>,
        method: &'a MethodDesc,
        callbacks: &'a [&'a TraitDesc],
    ) -> Result<()> {
        crate::swift::artifact_s2r::fill_return_type_convert(
            method_body,
            &method.return_type,
            &self.desc.crate_name,
            callbacks,
        )
    }
}
