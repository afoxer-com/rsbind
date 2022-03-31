use rstgen::swift::{local, Field, Modifier, Swift, Constructor, Argument};
use rstgen::{swift, IntoTokens};

use crate::ast::contract::desc::StructDesc;
use crate::errors::*;
use crate::swift::types::{to_swift_file, SwiftType};

pub(crate) struct StructGen<'a> {
    pub desc: &'a StructDesc,
}

impl<'a> StructGen<'a> {
    pub fn gen(&self) -> Result<String> {
        let mut struct_ = swift::Struct::new(self.desc.name.clone());
        struct_.modifiers.push(Modifier::Public);
        struct_.implements.push(local("Codable"));

        let mut constructor = Constructor::new();
        constructor.modifiers = vec![Modifier::Public];
        for arg in self.desc.fields.iter() {
            let field_ty = SwiftType::new(arg.ty.clone());
            let swift_ty = Swift::from(field_ty);
            let mut swift_field = Field::new(swift_ty.clone(), arg.name.clone());
            swift_field.modifiers = vec![Modifier::Public];
            struct_.fields.push(swift_field);

            constructor.arguments.push(Argument::new(swift_ty.clone(), arg.name.clone()));
            push!(constructor.body, "self.", arg.name.clone(), " = ", arg.name.clone());
        }
        struct_.constructors.push(constructor);
        to_swift_file(struct_.into_tokens())
    }
}
