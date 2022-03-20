use std::fmt::Write;

use rstgen::{Custom, Formatter, java, Java, Tokens};

use crate::ast::types::{AstBaseType, AstType};
use crate::errors::*;

#[derive(Clone)]
pub(crate) struct JavaType {
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

    pub(crate) fn to_java_array(&self, java: Java<'static>, boxed: bool) -> Java<'static> {
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
            AstType::Short(_) => java::SHORT,
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

pub(crate) fn to_java_file(pkg: &str, tokens: Tokens<Java>) -> Result<String> {
    let mut buf = String::new();
    {
        let mut formatter = Formatter::new(&mut buf);
        let mut extra = java::Extra::default();
        extra.package(pkg.as_ref());
        java::Java::write_file(tokens, &mut formatter, &mut extra, 0)?;
    }
    Ok(buf)
}
