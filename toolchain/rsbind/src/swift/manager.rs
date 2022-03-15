use crate::ast::contract::desc::TraitDesc;
use crate::errors::*;
use crate::swift::types::to_swift_file;
use crate::AstResult;
use rstgen::swift::{Class, Method, Modifier, Swift};
use rstgen::{java, swift, IntoTokens, Java, Tokens};
use syn::Item::Mod;

pub(crate) struct ManagerGen<'a> {
    pub ast: &'a AstResult,
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
                    let mut method = Method::new(method_name.clone());
                    method.modifiers = vec![Modifier::Public, Modifier::Static];
                    method.returns = Some(swift::local(each.name.clone()));
                    let mut method_body: Tokens<Swift> = Tokens::new();
                    method_body.push(toks!("return Rust", each.name.to_string(), "()"));
                    method.body = method_body;
                    class.methods.push(method)
                }
            }
        }

        to_swift_file(class.into_tokens())
    }
}
