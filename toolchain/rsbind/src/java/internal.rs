use heck::{ToLowerCamelCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;

use rstgen::java::{self, *};
use rstgen::Custom;
use rstgen::Formatter;
use rstgen::IntoTokens;
use rstgen::Tokens;

use crate::ast::contract::desc::MethodDesc;
use crate::ast::contract::desc::StructDesc;
use crate::ast::contract::desc::TraitDesc;
use crate::ast::types::AstBaseType;
use crate::ast::types::AstType;
use crate::ast::AstResult;
use crate::errors::*;
use crate::java::callback::CallbackGen;
use crate::java::method_cb::CBMethodGen;
use crate::java::struct_::StructGen;
use crate::java::types::{to_java_file, JavaType};

pub(crate) struct InnerTraitGen<'a> {
    pub desc: &'a TraitDesc,
    pub pkg: String,
    pub so_name: String,
    pub ext_libs: String,
    pub callbacks: Vec<TraitDesc>,
}

impl<'a> InnerTraitGen<'a> {
    /// Generate Java Code for rust trait.
    pub(crate) fn gen(&self) -> Result<String> {
        // We create two class, one is inner for interaction with Rust, another is for user to call.
        let inner_class_name = format!("Internal{}", &self.desc.name);
        let mut inner_class = Class::new(inner_class_name.clone());

        inner_class.modifiers = vec![];
        inner_class
            .implements
            .push(java::imported("java.io", "Serializable"));

        self.fill_global_block(&mut inner_class)?;
        self.fill_global_fields(&mut inner_class)?;

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

        self.fill_free_freecb_method(&mut inner_class)?;

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

        // invoke callback functions
        CBMethodGen {
            callbacks: sel_callbacks,
            pkg: self.pkg.clone(),
        }
        .gen(&mut inner_class)?;

        self.build_native_methods(self.desc.methods.clone(), &mut inner_class);
        to_java_file(self.pkg.as_ref(), inner_class.into_tokens())
    }

    fn fill_global_block(&'a self, class: &mut Class<'a>) -> Result<()> {
        let mut body = Tokens::new();
        body.push("static {");
        body.nested({
            let mut load_lib_tokens = Tokens::new();
            load_lib_tokens.push(toks!(
                "com.afoxer.rsbind.Common.loadLibrary(\"",
                self.so_name.clone(),
                "\");"
            ));
            let ext_libs = self.ext_libs.split(',').collect::<Vec<&str>>();
            for ext_lib in ext_libs.iter() {
                if !ext_lib.to_owned().is_empty() {
                    load_lib_tokens.push(toks!(
                        "System.loadLibrary(\"",
                        ext_lib.to_owned(),
                        "\");"
                    ));
                }
            }
            load_lib_tokens
        });
        body.push("}");

        class.body = body;
        Ok(())
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

        Ok(())
    }

    fn fill_free_freecb_method(&self, class: &mut Class<'a>) -> Result<()> {
        let mut free_method = Method::new("free_callback");
        free_method.modifiers = vec![Modifier::Public, Modifier::Static];
        free_method.arguments = vec![java::Argument::new(java::LONG, "index")];
        free_method.body = toks!("globalCallbacks.remove(index);");
        class.methods.push(free_method);

        Ok(())
    }

    fn fill_method_sig(&self, method: &MethodDesc) -> Result<Method> {
        let mut m = java::Method::new(method.name.to_lower_camel_case());
        m.modifiers = vec![Modifier::Static];
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

    fn fill_arg_convert(&self, method_body: &mut Tokens<Java>, method: &MethodDesc) -> Result<()> {
        for arg in method.args.clone().into_iter() {
            let converted = format!("r_{}", &arg.name);
            match arg.ty {
                AstType::Void => (),
                AstType::Callback(_) => {
                    let index_name = format!("{}_callback_index", &arg.name);
                    method_body.push(toks!(
                        "long ",
                        index_name.clone(),
                        " = globalIndex.incrementAndGet()",
                        ";"
                    ));
                    method_body.push(toks!(
                        "globalCallbacks.put(",
                        index_name.clone(),
                        ",",
                        arg.name.clone(),
                        ");"
                    ));
                    method_body.push(toks!(
                        "long ",
                        converted.clone(),
                        " = ",
                        index_name.clone(),
                        ";"
                    ));
                }
                AstType::Boolean => {
                    method_body.push(toks!(
                        "int ",
                        converted.clone(),
                        " = ",
                        arg.name.clone(),
                        " ? 1 : 0;"
                    ));
                }
                AstType::Vec(AstBaseType::Byte(_)) => {
                    let java = JavaType::new(arg.ty.clone(), self.pkg.clone());
                    let java = Java::from(java);
                    method_body.push(toks!(
                        java,
                        " ",
                        converted.clone(),
                        " = ",
                        arg.name.clone(),
                        ";"
                    ));
                }
                AstType::Vec(_) => {
                    let json_cls = java::imported("com.google.gson", "Gson");
                    method_body.push(toks!(
                        "String ",
                        converted.clone(),
                        " = new ",
                        json_cls,
                        "().toJson(",
                        arg.name.clone(),
                        ");"
                    ));
                }
                AstType::Struct(origin) => {
                    let json_cls = java::imported("com.google.gson", "Gson");
                    method_body.push(toks!(
                        "String ",
                        converted.clone(),
                        " = new ",
                        json_cls,
                        "().toJson(",
                        arg.name.clone(),
                        ");"
                    ));
                }
                _ => {
                    let java = JavaType::new(arg.ty.clone(), self.pkg.clone());
                    let java = Java::from(java);
                    method_body.push(toks!(java, " ", converted, " = ", arg.name.clone(), ";"));
                }
            }
        }

        Ok(())
    }

    fn fill_call_native_method(
        &self,
        method_body: &mut Tokens<Java>,
        method: &MethodDesc,
    ) -> Result<()> {
        let return_ty = JavaType::new(method.return_type.clone(), self.pkg.clone());

        let return_java_ty = return_ty.to_transfer();
        match return_ty.ast_type.clone() {
            AstType::Void => {
                method_body.push(toks!("native", method.name.to_upper_camel_case(), "("));
            }
            _ => {
                method_body.push(toks!(
                    return_java_ty,
                    " ret = native",
                    method.name.to_upper_camel_case(),
                    "("
                ));
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
        let return_ty = JavaType::new(method.return_type.clone(), self.pkg.clone());

        match return_ty.ast_type.clone() {
            AstType::Void => (),
            AstType::Vec(AstBaseType::Byte(_)) => {
                method_body.push(toks!("return ret;"));
            }
            AstType::Vec(_) => {
                let sub_ty = return_ty.get_base_ty();
                let json = java::imported("com.google.gson", "Gson");
                method_body.push(toks!(
                    "return new ",
                    json,
                    "().fromJson(ret, ",
                    sub_ty.clone().as_boxed(),
                    "[].class);"
                ));
            }
            AstType::Boolean => {
                method_body.push(toks!("return ret > 0 ? true : false;"));
            }
            AstType::Struct(origin) => {
                let json = java::imported("com.google.gson", "Gson");
                method_body.push(toks!(
                    "return new ",
                    json,
                    "().fromJson(ret,",
                    origin,
                    ".class);"
                ));
            }
            _ => {
                method_body.push(toks!("return ret;"));
            }
        }

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
                    let java = JavaType::new(method.return_type.clone(), self.pkg.clone());
                    m.returns = java.to_transfer();
                }
            }

            let args = method.args.clone();
            for arg in args.iter() {
                match arg.ty.clone() {
                    AstType::Void => (),
                    _ => {
                        let java = JavaType::new(arg.ty.clone(), self.pkg.clone());
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
