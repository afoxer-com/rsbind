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

pub(crate) struct JavaCodeGen<'a> {
    pub java_gen_dir: &'a PathBuf,
    pub ast: &'a AstResult,
    pub namespace: String,
    pub so_name: String,
    pub ext_libs: String,
}

impl<'a> JavaCodeGen<'a> {
    pub(crate) fn gen_java_code(&self) -> Result<()> {
        // get the java_gen dir string
        println!("get java_gen dir string");
        // fs::write(&java_gen_path, );

        // collect all the callbacks.
        let mut callbacks = vec![];
        for desc in self.ast.trait_descs.iter() {
            let descs = desc.1;
            for each in descs.iter() {
                if each.is_callback {
                    callbacks.push(each.clone());
                }
            }
        }

        // generate all the callbacks.
        for each in callbacks.clone().iter() {
            let gen = CallbackGen {
                desc: each,
                pkg: self.namespace.clone(),
            };

            let callback_str = gen.gen()?;
            let file_name = format!("{}.java", &each.name);
            let callback_path = self.java_gen_dir.clone().join(file_name);
            fs::write(callback_path, callback_str)?;
        }

        // generate all the traits.
        for desc in self.ast.trait_descs.iter() {
            let descs = desc.1;
            for each in descs.iter() {
                if !each.is_callback {
                    let gen = TraitGen {
                        desc: each,
                        pkg: self.namespace.clone(),
                        so_name: self.so_name.clone(),
                        ext_libs: self.ext_libs.clone(),
                        callbacks: callbacks.clone(),
                    };
                    let str = gen.gen()?;
                    let file_name = format!("{}.java", &each.name);
                    let path = self.java_gen_dir.clone().join(file_name);
                    fs::write(path, str)?;
                }
            }
        }

        // generate all the structs
        for (_key, struct_descs) in self.ast.struct_descs.iter() {
            for struct_desc in struct_descs.iter() {
                let gen = StructGen {
                    desc: struct_desc,
                    pkg: self.namespace.clone(),
                };

                let struct_str = gen.gen()?;
                let file_name = format!("{}.java", &struct_desc.name);
                let path = self.java_gen_dir.join(file_name);
                fs::write(path, struct_str)?
            }
        }

        Ok(())
    }
}

pub(crate) struct StructGen<'a> {
    pub desc: &'a StructDesc,
    pub pkg: String,
}

impl<'a> StructGen<'a> {
    pub(crate) fn gen(&self) -> Result<String> {
        let mut class = Class::new(self.desc.name.clone());
        class.modifiers.push(Modifier::Public);
        class
            .implements
            .push(java::imported("java.io", "Serializable"));

        for field in self.desc.fields.iter() {
            let field_ty = JavaType::new(field.ty.clone(), self.pkg.clone());
            let mut java_field = Field::new(Java::from(field_ty), field.name.clone());
            java_field.modifiers = vec![Modifier::Public];
            class.fields.push(java_field);
        }

        to_java_file(self.pkg.as_ref(), class.into_tokens())
    }
}

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
            let mut m = Method::new(method.name.clone());
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

pub(crate) struct TraitGen<'a> {
    pub desc: &'a TraitDesc,
    pub pkg: String,
    pub so_name: String,
    pub ext_libs: String,
    pub callbacks: Vec<TraitDesc>,
}

impl<'a> TraitGen<'a> {
    /// Generate Java Code for rust trait.
    pub(crate) fn gen(&self) -> Result<String> {
        let mut class = Class::new(self.desc.name.clone());
        class.modifiers = vec![Modifier::Public];
        class
            .implements
            .push(java::imported("java.io", "Serializable"));

        self.fill_global_block(&mut class)?;
        self.fill_global_fields(&mut class)?;

        let methods = self.desc.methods.clone();
        for method in methods.into_iter() {
            // Method signature
            let mut m = self.fill_method_sig(&method)?;

            let mut method_body: Tokens<Java> = Tokens::new();

            // Argument convert
            self.fill_arg_convert(&mut method_body, &method)?;

            // Call native method
            self.fill_call_native_method(&mut method_body, &method)?;

            // Return type convert
            self.fill_return_ty_convert(&mut method_body, &method)?;

            m.body = method_body;
            class.methods.push(m);
        }

        self.fill_free_freecb_method(&mut class)?;

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
        for callback in sel_callbacks.iter() {
            for cb_method in callback.methods.iter() {
                let mut m = self.fill_cb_method_sig(cb_method, callback)?;

                let mut cb_body = toks!();
                // argument convert
                self.fill_cb_arg_convert(&mut cb_body, cb_method)?;

                // callback invoke
                self.fill_cb_invoke(&mut cb_body, cb_method, callback)?;

                // return type convert
                self.fill_cb_return_convert(&mut cb_body, cb_method)?;

                m.body = cb_body;

                class.methods.push(m);
            }
        }

        self.build_native_methods(self.desc.methods.clone(), &mut class);

        to_java_file(self.pkg.as_ref(), class.into_tokens())
    }

    fn fill_global_block(&'a self, class: &mut Class<'a>) -> Result<()> {
        let mut body = Tokens::new();
        body.push("static {");
        body.nested({
            let mut load_lib_tokens = Tokens::new();
            load_lib_tokens.push(toks!("System.loadLibrary(\"", self.so_name.clone(), "\");"));
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
        let mut m = java::Method::new(method.name.clone());
        m.modifiers = vec![Modifier::Public, Modifier::Static];

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
                method_body.push(toks!("native_", method.name.clone(), "("));
            }
            _ => {
                method_body.push(toks!(
                    return_java_ty,
                    " ret = native_",
                    method.name.clone(),
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

    fn fill_cb_method_sig(&self, cb_method: &MethodDesc, callback: &TraitDesc) -> Result<Method> {
        let method_name = format!("invoke_{}_{}", &callback.name, &cb_method.name);
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
                    cb_method.name.clone(),
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
                    cb_method.name.clone(),
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

    ///
    /// build native methods for accessing .so
    ///
    fn build_native_methods(&self, methods: Vec<MethodDesc>, class: &mut Class) {
        for method in methods.iter() {
            let method_name = format!("native_{}", method.name.clone());
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

#[derive(Clone)]
struct JavaType {
    pub ast_type: AstType,
    pub pkg: String,
}

impl JavaType {
    pub(crate) fn new(ast_type: AstType, pkg: String) -> JavaType {
        JavaType { ast_type, pkg }
    }

    pub(crate) fn to_array(&self) -> Java<'static> {
        let base_name = Java::from(self.clone());
        self.to_java_array(base_name, false)
    }

    pub(crate) fn to_boxed_array(&self) -> Java<'static> {
        let base_name = Java::from(self.clone());
        self.to_java_array(base_name, true)
    }

    pub(crate) fn to_transfer(&self) -> Java<'static> {
        match self.ast_type.clone() {
            AstType::Boolean => java::INTEGER,
            AstType::Vec(AstBaseType::Byte(_)) => Java::from(self.clone()),
            AstType::Vec(_) => java::imported("java.lang", "String"),
            AstType::Struct(_) => java::imported("java.lang", "String"),
            AstType::Callback(_) => java::LONG,
            _ => Java::from(self.clone()),
        }
    }

    /// If JavaType is an Vec(base), we will return base, else we will return itself.
    pub(crate) fn get_base_ty(&self) -> Java<'static> {
        match self.ast_type.clone() {
            AstType::Vec(AstBaseType::Struct(origin)) => java::local(origin),
            AstType::Vec(base) => Java::from(JavaType::new(AstType::from(base), self.pkg.clone())),
            _ => Java::from(self.clone()),
        }
    }

    fn to_java_array(&self, java: Java<'static>, boxed: bool) -> Java<'static> {
        let mut base_str = String::new();
        {
            let mut formatter = Formatter::new(&mut base_str);
            let mut extra = java::Extra::default();
            let level = if boxed { 1 } else { 0 };
            let _ = java.format(&mut formatter, &mut extra, level);
        }
        let _ = base_str.write_str("[]");
        java::local(base_str)
    }
}

impl From<JavaType> for Java<'static> {
    fn from(item: JavaType) -> Self {
        match item.ast_type {
            AstType::Boolean => java::BOOLEAN,
            AstType::Byte(_) => java::BYTE,
            AstType::Int(_) => java::INTEGER,
            AstType::Long(_) => java::LONG,
            AstType::Float(_) => java::FLOAT,
            AstType::Double(_) => java::DOUBLE,
            AstType::String => java::imported("java.lang", "String"),
            AstType::Vec(ref base) => match base {
                AstBaseType::Struct(_sub) => {
                    JavaType::new(AstType::from(base.clone()), item.pkg.clone()).to_array()
                }
                // Byte array is not transferred by json, so we don't use boxed array.
                AstBaseType::Byte(_) => {
                    JavaType::new(AstType::from(base.clone()), item.pkg.clone()).to_array()
                }
                // Why we use boxed array, because we use json to transfer array,
                // and it is translated to list, and then we need to change it to array(boxed).
                _ => JavaType::new(AstType::from(base.clone()), item.pkg.clone()).to_boxed_array(),
            },
            AstType::Void => java::VOID,
            AstType::Callback(origin) | AstType::Struct(origin) => java::local(origin),
        }
    }
}

fn to_java_file(pkg: &str, tokens: Tokens<Java>) -> Result<String> {
    let mut buf = String::new();
    {
        let mut formatter = Formatter::new(&mut buf);
        let mut extra = java::Extra::default();
        extra.package(pkg.as_ref());
        java::Java::write_file(tokens, &mut formatter, &mut extra, 0)?;
    }
    Ok(buf)
}
