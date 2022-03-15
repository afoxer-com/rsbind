use crate::ast::contract::desc::{MethodDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::errors::*;
use crate::java::types::JavaType;
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use rstgen::java::{Argument, Class, Method, Modifier};
use rstgen::{java, Java, Tokens};

/// For generating java functions for callback to invoke.
pub(crate) struct CBMethodGen<'a> {
    pub(crate) callbacks: Vec<&'a TraitDesc>,
    pub(crate) pkg: String,
}

impl<'a, 'b> CBMethodGen<'a> {
    pub(crate) fn gen(&'b self, inner_class: &mut Class<'a>) -> Result<()> {
        // invoke callback functions
        for callback in self.callbacks.iter() {
            for cb_method in callback.methods.iter() {
                let mut m = self.fill_cb_method_sig(cb_method, callback)?;
                let mut cb_body = toks!();
                self.fill_cb_arg_convert(&mut cb_body, cb_method)?;
                self.fill_cb_invoke(&mut cb_body, cb_method, callback)?;
                self.fill_cb_return_convert(&mut cb_body, cb_method)?;
                m.body = cb_body;
                inner_class.methods.push(m);
            }
        }

        Ok(())
    }

    fn fill_cb_method_sig(
        &'b self,
        cb_method: &'a MethodDesc,
        callback: &'a TraitDesc,
    ) -> Result<Method<'a>> {
        let method_name = format!(
            "invoke{}{}",
            &callback.name.to_upper_camel_case(),
            &cb_method.name.to_upper_camel_case()
        );
        let mut m = java::Method::new(method_name);
        m.modifiers = vec![Modifier::Public, Modifier::Static];

        if cb_method.return_type != AstType::Void {
            m.returns =
                JavaType::new(cb_method.return_type.clone(), self.pkg.clone()).to_transfer();
        }

        let mut argument = Argument::new(java::LONG, "index");
        argument.modifiers = vec![];
        m.arguments.push(argument);
        for arg in cb_method.args.iter() {
            let arg_type = JavaType::new(arg.ty.clone(), self.pkg.clone()).to_transfer();
            let mut argument = Argument::new(arg_type, arg.name.clone());
            argument.modifiers = vec![];
            m.arguments.push(argument);
        }

        Ok(m)
    }

    fn fill_cb_arg_convert(
        &self,
        cb_body: &mut Tokens<Java>,
        cb_method: &MethodDesc,
    ) -> Result<()> {
        for arg in cb_method.args.iter() {
            match arg.ty.clone() {
                AstType::Boolean => {
                    cb_body.push(toks!(
                        "boolean ",
                        "j_",
                        arg.name.clone(),
                        " = ",
                        arg.name.clone(),
                        " > 0 ? true : false;"
                    ));
                }
                AstType::Struct(sub) => {
                    let json = java::imported("com.google.gson", "Gson");
                    cb_body.push(toks!(
                        sub.clone(),
                        " j_",
                        arg.name.clone(),
                        " = new ",
                        json,
                        "().fromJson(",
                        arg.name.clone(),
                        ", ",
                        sub.clone(),
                        ".class);"
                    ));
                }
                AstType::Vec(AstBaseType::Byte(_)) => {
                    let java = JavaType::new(arg.ty.clone(), self.pkg.clone());
                    cb_body.push(toks!(
                        java.get_base_ty(),
                        "[] ",
                        "j_",
                        arg.name.clone(),
                        " = ",
                        arg.name.clone(),
                        ";"
                    ));
                }
                AstType::Vec(_) => {
                    let json = java::imported("com.google.gson", "Gson");
                    let java = JavaType::new(arg.ty.clone(), self.pkg.clone());
                    cb_body.push(toks!(
                        java.get_base_ty().as_boxed(),
                        "[] ",
                        "j_",
                        arg.name.clone(),
                        " = new ",
                        json,
                        "().fromJson(",
                        arg.name.clone(),
                        ", ",
                        java.get_base_ty().as_boxed(),
                        "[].class);"
                    ));
                }
                _ => {
                    let java = JavaType::new(arg.ty.clone(), self.pkg.clone());
                    cb_body.push(toks!(
                        Java::from(java),
                        " j_",
                        arg.name.clone(),
                        " = ",
                        arg.name.clone(),
                        ";"
                    ));
                }
            }
        }

        Ok(())
    }

    fn fill_cb_invoke(
        &self,
        cb_body: &mut Tokens<Java>,
        cb_method: &MethodDesc,
        callback: &TraitDesc,
    ) -> Result<()> {
        let mut arg_calls = String::new();

        for (index, arg) in cb_method.args.iter().enumerate() {
            if index == cb_method.args.len() - 1 {
                arg_calls = format!("{}j_{}", arg_calls, &arg.name);
            } else {
                arg_calls = format!("{}j_{}, ", arg_calls, &arg.name);
            }
        }

        cb_body.push(toks!(
            callback.name.clone(),
            " callback = (",
            callback.name.clone(),
            ") globalCallbacks.get(index);"
        ));
        match cb_method.return_type.clone() {
            AstType::Void => {
                cb_body.push(toks!(
                    "callback.",
                    cb_method.name.to_lower_camel_case(),
                    "(",
                    arg_calls,
                    ");"
                ));
            }
            _ => {
                let java = JavaType::new(cb_method.return_type.clone(), self.pkg.clone());
                cb_body.push(toks!(
                    Java::from(java),
                    " result = callback.",
                    cb_method.name.to_lower_camel_case(),
                    "(",
                    arg_calls,
                    ");"
                ));
            }
        }

        Ok(())
    }

    fn fill_cb_return_convert(
        &self,
        cb_body: &mut Tokens<Java>,
        cb_method: &MethodDesc,
    ) -> Result<()> {
        match cb_method.return_type.clone() {
            AstType::Boolean => {
                cb_body.push(toks!("return result ? 1 : 0;"));
            }
            AstType::Void => (),
            _ => {
                cb_body.push(toks!("return result;"));
            }
        }
        Ok(())
    }
}
