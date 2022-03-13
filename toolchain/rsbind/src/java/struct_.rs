use rstgen::{IntoTokens, java, Java};
use rstgen::java::{Class, Field, Modifier};
use crate::ast::contract::desc::StructDesc;
use crate::errors::*;
use crate::java::types::{JavaType, to_java_file};

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
