use rstgen::swift::{local, Argument, Constructor, Field, Method, Modifier, Swift};
use rstgen::{swift, IntoTokens};

use crate::ast::contract::desc::StructDesc;
use crate::base::lang::{Convertible, Direction};
use crate::errors::*;
use crate::swift::converter::SwiftConvert;
use crate::swift::types::{to_swift_file, SwiftType};

pub(crate) struct StructGen<'a> {
    pub desc: &'a StructDesc,
}

impl<'a> StructGen<'a> {
    pub fn gen(&self) -> Result<String> {
        let mut struct_ = swift::Struct::new(self.desc.name.clone());
        struct_.modifiers.push(Modifier::Public);
        struct_.implements.push(local("Codable"));

        let mut constructor1 = Constructor::new();
        constructor1.modifiers = vec![Modifier::Public];
        for arg in self.desc.fields.iter() {
            let field_ty = SwiftType::new(arg.ty.clone());
            let swift_ty = Swift::from(field_ty);
            let mut swift_field = Field::new(swift_ty.clone(), arg.name.clone());
            swift_field.modifiers = vec![Modifier::Public];
            struct_.fields.push(swift_field);

            constructor1
                .arguments
                .push(Argument::new(swift_ty.clone(), arg.name.clone()));
            push_f!(constructor1.body, "self.{} = {}", arg.name, arg.name);
        }
        struct_.constructors.push(constructor1);
        struct_.constructors.push(self.create_proxy_constructor());
        struct_.methods.push(self.create_into_proxy_fn());

        to_swift_file(struct_.into_tokens())
    }

    fn create_into_proxy_fn(&self) -> Method {
        let mut method = swift::Method::new("intoProxy");
        method.modifiers = vec![Modifier::Internal];
        method.returns(swift::local(format!("Proxy{}", &self.desc.name)));
        push_f!(method.body, "return Proxy{} (", self.desc.name);
        for (index, field) in self.desc.fields.iter().enumerate() {
            let convert = SwiftConvert {
                ty: field.ty.clone(),
            }
            .native_to_transferable(format!("self.{}", &field.name), Direction::Down);
            nested_f!(method.body, "{} : {}", field.name, convert);
            if index != self.desc.fields.len() - 1 {
                method.body.append(",")
            }
        }
        push!(method.body, ")");

        method
    }

    fn create_proxy_constructor(&self) -> Constructor {
        let mut constructor2 = Constructor::new();
        constructor2.modifiers = vec![Modifier::Internal];
        constructor2.arguments.push(Argument::new(
            swift::local(format!("Proxy{}", &self.desc.name)),
            "proxy",
        ));

        for field in self.desc.fields.iter() {
            let convert = SwiftConvert {
                ty: field.ty.clone(),
            }
            .transferable_to_native(format!("proxy.{}", &field.name), Direction::Down);
            push_f!(constructor2.body, "self.{} = {}", field.name, convert);
        }

        constructor2
    }
}
