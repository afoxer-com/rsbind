use heck::{ToLowerCamelCase, ToUpperCamelCase};
use rstgen::java::{self, *};
use rstgen::IntoTokens;
use rstgen::Tokens;

use crate::ast::contract::desc::MethodDesc;
use crate::ast::contract::desc::TraitDesc;
use crate::ast::types::AstType;
use crate::base::lang::{Convertible, Direction};
use crate::errors::*;
use crate::java::converter::JavaConvert;
use crate::java::types::{to_java_file, JavaType};

pub(crate) struct InnerTraitGen<'a> {
    pub desc: &'a TraitDesc,
    pub pkg: String,
    pub callbacks: Vec<TraitDesc>,
}

impl<'a> InnerTraitGen<'a> {
    /// Generate Java Code for rust trait.
    pub(crate) fn gen(&self) -> Result<String> {
        // We create two class, one is inner for interaction with Rust, another is for user to call.
        let inner_class_name = format!("Internal{}", &self.desc.name);
        let mut inner_class = Class::new(inner_class_name);

        inner_class.modifiers = vec![];
        inner_class
            .implements
            .push(java::imported("java.io", "Serializable"));

        let methods = self.desc.methods.clone();
        for method in methods.into_iter() {
            let mut inner_method = self.fill_method_sig(&method)?;
            let mut inner_method_body: Tokens<Java> = Tokens::new();
            self.fill_arg_convert(&mut inner_method_body, &method)?;
            self.fill_call_native_method(&mut inner_method_body, &method)?;
            self.fill_return_ty_convert(&mut inner_method_body, &method)?;
            inner_method.body = inner_method_body;
            inner_class.methods.push(inner_method);
        }

        let mut sel_callbacks = vec![];
        let methods = self.desc.methods.clone();
        for method in methods.into_iter() {
            for arg in method.args.clone().into_iter() {
                // Select the callbacks in arguments
                if let AstType::Callback(_) = arg.ty {
                    let callback = self
                        .callbacks
                        .iter()
                        .filter(|callback| callback.name == arg.ty.origin())
                        .collect::<Vec<&TraitDesc>>();
                    println!("callback xxxx is {:?}", callback.clone());
                    if !callback.is_empty() && !sel_callbacks.contains(&callback[0]) {
                        sel_callbacks.push(callback[0]);
                    }
                }
            }
        }

        self.build_native_methods(self.desc.methods.clone(), &mut inner_class);
        to_java_file(self.pkg.as_ref(), inner_class.into_tokens())
    }

    fn fill_method_sig(&self, method: &MethodDesc) -> Result<Method> {
        let mut m = java::Method::new(method.name.to_lower_camel_case());
        m.modifiers = vec![Modifier::Static];
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

    fn fill_arg_convert(&self, method_body: &mut Tokens<Java>, method: &MethodDesc) -> Result<()> {
        for arg in method.args.iter() {
            if let AstType::Void = arg.ty.clone() {
                continue;
            }

            let java = JavaType::new(arg.ty.clone()).to_transfer();
            let converted = format!("r_{}", &arg.name);
            let convert = JavaConvert { ty: arg.ty.clone() }
                .native_to_transferable(arg.name.clone(), Direction::Down);
            push!(method_body, java, "  ", converted, " = ", convert, ";");
        }

        Ok(())
    }

    fn fill_call_native_method(
        &self,
        method_body: &mut Tokens<Java>,
        method: &MethodDesc,
    ) -> Result<()> {
        let return_ty = JavaType::new(method.return_type.clone());

        let return_java_ty = return_ty.to_transfer();
        match return_ty.ast_type.clone() {
            AstType::Void => {
                push!(
                    method_body,
                    "native",
                    method.name.to_upper_camel_case(),
                    "("
                );
            }
            _ => {
                push!(
                    method_body,
                    return_java_ty,
                    " ret = native",
                    method.name.to_upper_camel_case(),
                    "("
                );
            }
        }

        for (index, item) in method.args.clone().into_iter().enumerate() {
            let converted = format!("r_{}", &item.name);
            if index == method.args.len() - 1 {
                method_body.append(toks!(converted));
            } else {
                method_body.append(toks!(converted, ", "));
            }
        }
        method_body.append(toks!(");"));
        Ok(())
    }

    fn fill_return_ty_convert(
        &self,
        method_body: &mut Tokens<Java>,
        method: &MethodDesc,
    ) -> Result<()> {
        if let AstType::Void = method.return_type.clone() {
            return Ok(());
        }

        let convert = JavaConvert {
            ty: method.return_type.clone(),
        }
        .transferable_to_native("ret".to_string(), Direction::Down);
        push!(method_body, "return ", convert, ";");
        Ok(())
    }

    ///
    /// build native methods for accessing .so
    ///
    fn build_native_methods(&self, methods: Vec<MethodDesc>, class: &mut Class) {
        for method in methods.iter() {
            let method_name = format!("native{}", method.name.to_upper_camel_case());
            let mut m = java::Method::new(method_name);
            m.modifiers = vec![Modifier::Private, Modifier::Static, Modifier::Native];

            match method.return_type.clone() {
                AstType::Void => (),
                _ => {
                    let java = JavaType::new(method.return_type.clone());
                    m.returns = java.to_transfer();
                }
            }

            let args = method.args.clone();
            for arg in args.iter() {
                match arg.ty.clone() {
                    AstType::Void => (),
                    _ => {
                        let java = JavaType::new(arg.ty.clone());
                        let mut argument = Argument::new(java.to_transfer(), arg.name.clone());
                        argument.modifiers = vec![];
                        m.arguments.push(argument);
                    }
                }
            }

            class.methods.push(m);
        }
    }
}
