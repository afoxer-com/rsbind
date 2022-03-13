use rstgen::{IntoTokens, java, Java, Tokens};
use rstgen::java::{Class, Modifier};
use syn::Item::Mod;
use crate::ast::contract::desc::TraitDesc;
use crate::AstResult;
use crate::errors::*;
use crate::java::types::to_java_file;

pub(crate) struct ManagerGen<'a> {
    pub ast: &'a AstResult,
    pub pkg: String,
}

impl<'a> ManagerGen<'a> {
    pub(crate) fn gen(&self) -> Result<String> {
        let mut class = Class::new("RustLib");
        class.modifiers = vec![Modifier::Public];
        for desc in self.ast.trait_descs.iter() {
            let descs = desc.1;
            for each in descs.iter() {
                if !each.is_callback {
                    let method_name = format!("new{}", &each.name);
                    let mut method = java::Method::new(method_name.clone());
                    method.modifiers = vec![Modifier::Public, Modifier::Static];
                    method.returns = java::local(each.name.clone());
                    let mut method_body: Tokens<Java> = Tokens::new();
                    method_body.push(toks!(
                        "return new Rust", each.name.to_string(), "();"
                    ));
                    method.body = method_body;
                    class.methods.push(method)
                }
            }
        }

        to_java_file(self.pkg.as_ref(), class.into_tokens())
    }
}