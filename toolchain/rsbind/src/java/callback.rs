use heck::{ToLowerCamelCase, ToUpperCamelCase};
use rstgen::{IntoTokens, java, Java, Tokens};
use rstgen::java::{Argument, Class, Field, Interface, Method, Modifier};

use crate::ast::contract::desc::{MethodDesc, TraitDesc};
use crate::ast::types::AstType;
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

pub(crate) struct InnerCallbackGen<'a> {
    pub desc: &'a TraitDesc,
    pub pkg: String,
}

impl<'a> InnerCallbackGen<'a> {
    /// Generate Java Code for rust trait.
    pub(crate) fn gen(&self) -> Result<String> {
        // We create two class, one is inner for interaction with Rust, another is for user to call.
        let inner_class_name = format!("Internal{}", &self.desc.name);
        let mut inner_class = Class::new(inner_class_name.clone());

        inner_class.modifiers = vec![];
        inner_class
            .implements
            .push(java::imported("java.io", "Serializable"));

        self.fill_global_fields(&mut inner_class)?;

        let mut method = java::Method::new("pushGlobalCallback");
        method.modifiers = vec![Modifier::Static];
        method.arguments.push(Argument::new(
            java::local(self.desc.name.clone()),
            "callback",
        ));
        method.body.push(toks!(
            "long callback_index = globalIndex.incrementAndGet();"
        ));
        method
            .body
            .push(toks!("globalCallbacks.put(callback_index, callback);"));
        method.body.push(toks!("return callback_index;"));
        method.returns = java::LONG;
        inner_class.methods.push(method);

        let methods = self.desc.methods.clone();
        for cb_method in methods.iter() {
            let mut m = self.fill_r2j_cb_method_sig(cb_method, self.desc)?;
            let mut cb_body = toks!();
            self.fill_r2j_cb_arg_convert(&mut cb_body, cb_method)?;
            self.fill_r2j_cb_invoke(&mut cb_body, cb_method, self.desc)?;
            self.fill_r2j_cb_return_convert(&mut cb_body, cb_method)?;
            m.body = cb_body;
            inner_class.methods.push(m);
        }

        self.fill_free_freecb_method(&mut inner_class)?;

        for cb_method in self.desc.methods.iter() {
            let m = self.fill_j2r_cb_method_sig(cb_method, self.desc)?;
            inner_class.methods.push(m);
        }
        let mut free_cb_method = java::Method::new("j2rFreeCallback");
        free_cb_method.modifiers = vec![
            java::Modifier::Native,
            java::Modifier::Private,
            java::Modifier::Static,
        ];
        free_cb_method
            .arguments
            .push(Argument::new(java::LONG, "index"));
        inner_class.methods.push(free_cb_method);

        let class_name = format!("J2R{}Wrapper", &self.desc.name);
        let mut class = java::Class::new(class_name);
        class.modifiers = vec![java::Modifier::Static];
        class.implements.push(java::local(self.desc.name.clone()));

        let mut filed = java::Field::new(java::LONG, "index");
        filed.modifiers = vec![Modifier::Private];
        class.fields.push(filed);

        let mut constructor = java::Constructor::new();
        constructor.modifiers = vec![Modifier::Public];
        constructor
            .arguments
            .push(Argument::new(java::LONG, "index"));
        constructor.body.push(toks!("this.index = index;"));
        // constructor.body.push(toks!(
        //     "cleaner.register(this, () -> ",
        //     inner_class_name.clone(),
        //     ".j2rFreeCallback(index));"
        // ));
        class.constructors.push(constructor);

        for method in self.desc.methods.iter() {
            let mut m = java::Method::new(method.name.to_lower_camel_case());
            m.modifiers = vec![java::Modifier::Public];
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

            let mut method_body = Tokens::new();
            self.fill_j2r_cb_arg_convert(&mut method_body, method)?;
            self.fill_j2r_cb_invoke(&mut method_body, method, self.desc)?;
            crate::java::artifact_j2r::fill_return_convert(&mut method_body, method, &self.pkg)?;
            m.body = method_body;
            class.methods.push(m);
        }

        let mut finalize_method = Method::new("finalize");
        finalize_method.modifiers = vec![Modifier::Protected];
        finalize_method.annotation(toks!("@Override"));
        let _ = finalize_method.throws.insert(toks!("Throwable"));
        finalize_method.body.push(toks!("super.finalize();"));
        finalize_method
            .body
            .push(toks!(inner_class_name, ".j2rFreeCallback(index);"));
        class.methods.push(finalize_method);

        inner_class.body.push(class.into_tokens());

        to_java_file(self.pkg.as_ref(), inner_class.into_tokens())
    }

    fn fill_global_fields(&self, class: &mut Class<'a>) -> Result<()> {
        let mut index_field = Field::new(
            java::imported("java.util.concurrent.atomic", "AtomicLong"),
            "globalIndex",
        );
        index_field.initializer("new AtomicLong(0)");
        index_field.modifiers = vec![Modifier::Private, Modifier::Static];
        class.fields.push(index_field);

        let callbacks_ty = java::imported("java.util.concurrent", "ConcurrentHashMap")
            .with_arguments(vec![java::LONG, java::imported("java.lang", "Object")]);
        let mut callbacks_field = Field::new(callbacks_ty, "globalCallbacks");
        callbacks_field.initializer("new ConcurrentHashMap<>()");
        callbacks_field.modifiers = vec![Modifier::Private, Modifier::Static];
        class.fields.push(callbacks_field);

        // let mut cleaner_field = Field::new(java::imported("java.lang.ref", "Cleaner"), "cleaner");
        // cleaner_field.modifiers = vec![Modifier::Static, Modifier::Private];
        // cleaner_field.initializer(toks!("Cleaner.create();"));
        // class.fields.push(cleaner_field);
        Ok(())
    }

    fn fill_free_freecb_method(&self, class: &mut Class<'a>) -> Result<()> {
        let mut free_method = Method::new("r2jFreeCallback");
        free_method.modifiers = vec![Modifier::Static];
        free_method.arguments = vec![java::Argument::new(java::LONG, "index")];
        free_method.body = toks!("globalCallbacks.remove(index);");
        class.methods.push(free_method);

        Ok(())
    }

    fn fill_r2j_cb_method_sig(
        &self,
        cb_method: &'a MethodDesc,
        _callback: &'a TraitDesc,
    ) -> Result<Method<'a>> {
        let method_name = format!("r2j{}", &cb_method.name.to_upper_camel_case());
        let mut m = java::Method::new(method_name);
        m.modifiers = vec![Modifier::Static];

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

    fn fill_r2j_cb_arg_convert(
        &self,
        cb_body: &mut Tokens<Java>,
        cb_method: &MethodDesc,
    ) -> Result<()> {
        for arg in cb_method.args.iter() {
            crate::java::artifact_r2j::fill_arg_convert(arg, cb_body, &self.pkg)?;
        }

        Ok(())
    }

    fn fill_r2j_cb_invoke(
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

    fn fill_r2j_cb_return_convert(
        &self,
        cb_body: &mut Tokens<Java>,
        cb_method: &MethodDesc,
    ) -> Result<()> {
        crate::java::artifact_r2j::fill_return_convert(cb_body, cb_method)
    }

    fn fill_j2r_cb_method_sig(
        &self,
        cb_method: &'a MethodDesc,
        _callback: &'a TraitDesc,
    ) -> Result<Method<'a>> {
        let method_name = format!("j2r{}", &cb_method.name.to_upper_camel_case());
        let mut m = java::Method::new(method_name);
        m.modifiers = vec![Modifier::Private, Modifier::Static, Modifier::Native];

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

    fn fill_j2r_cb_arg_convert(
        &self,
        cb_body: &mut Tokens<Java>,
        cb_method: &MethodDesc,
    ) -> Result<()> {
        for arg in cb_method.args.iter() {
            crate::java::artifact_j2r::fill_arg_convert(cb_body, arg, &self.pkg)?;
        }

        Ok(())
    }

    fn fill_j2r_cb_invoke(
        &self,
        cb_body: &mut Tokens<Java>,
        cb_method: &MethodDesc,
        _callback: &TraitDesc,
    ) -> Result<()> {
        let mut arg_calls = "index".to_string();
        if !cb_method.args.is_empty() {
            arg_calls = format!("{}, ", &arg_calls);
        }
        for (index, arg) in cb_method.args.iter().enumerate() {
            if index == cb_method.args.len() - 1 {
                arg_calls = format!("{}r_{}", &arg_calls, &arg.name);
            } else {
                arg_calls = format!("{}r_{}, ", &arg_calls, &arg.name);
            }
        }

        match cb_method.return_type.clone() {
            AstType::Void => {
                cb_body.push(toks!(
                    "j2r",
                    cb_method.name.to_upper_camel_case(),
                    "(",
                    arg_calls,
                    ");"
                ));
            }
            _ => {
                let java =
                    JavaType::new(cb_method.return_type.clone(), self.pkg.clone()).to_transfer();
                cb_body.push(toks!(
                    java,
                    " ret = j2r",
                    cb_method.name.to_upper_camel_case(),
                    "(",
                    arg_calls,
                    ");"
                ));
            }
        }

        Ok(())
    }
}
