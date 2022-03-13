use rstgen::{IntoTokens, swift};
use rstgen::swift::{Field, local, Modifier, Swift};
use crate::ast::contract::desc::StructDesc;
use crate::swift::types::{SwiftType, to_swift_file};
use crate::errors::*;

pub(crate) struct StructGen<'a> {
    pub desc: &'a StructDesc,
}

impl<'a> StructGen<'a> {
    pub fn gen(&self) -> Result<String> {
        let mut struct_ = swift::Struct::new(self.desc.name.clone());
        struct_.modifiers.push(Modifier::Public);
        struct_.implements.push(local("Codable"));

        for field in self.desc.fields.iter() {
            let field_ty = SwiftType::new(field.ty.clone());
            let mut swift_field = Field::new(Swift::from(field_ty), field.name.clone());
            swift_field.modifiers = vec![Modifier::Public];
            struct_.fields.push(swift_field);
        }

        to_swift_file(struct_.into_tokens())
    }
}