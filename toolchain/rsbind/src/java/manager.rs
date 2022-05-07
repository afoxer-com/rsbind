use rstgen::java::{Class, Modifier};
use rstgen::{java, IntoTokens, Java, Tokens};

use crate::errors::*;
use crate::java::types::to_java_file;
use crate::AstResult;

pub(crate) struct ManagerGen<'a> {
    pub ast: &'a AstResult,
    pub pkg: String,
    pub so_name: String,
    pub ext_libs: String,
}

impl<'a> ManagerGen<'a> {
    pub(crate) fn gen(&self) -> Result<String> {
        let mut class = Class::new("RustLib");
        class.modifiers = vec![Modifier::Public];
        for desc in self.ast.traits.iter() {
            let descs = desc.1;
            for each in descs.iter() {
                if !each.is_callback {
                    let method_name = format!("new{}", &each.name);
                    let mut method = java::Method::new(method_name.clone());
                    method.modifiers = vec![Modifier::Public, Modifier::Static];
                    method.returns = java::local(each.name.clone());
                    let mut method_body: Tokens<Java> = Tokens::new();
                    push_f!(method_body, "return new Rust{}();", each.name.to_string());
                    method.body = method_body;
                    class.methods.push(method)
                }
            }
        }

        self.fill_global_block(&mut class)?;

        to_java_file(self.pkg.as_ref(), class.into_tokens())
    }

    fn fill_global_block(&'a self, class: &mut Class<'a>) -> Result<()> {
        let mut body = Tokens::new();
        body.push("static {");
        body.nested({
            let mut load_lib_tokens = Tokens::new();
            push_f!(
                load_lib_tokens,
                "com.afoxer.rsbind.Common.loadLibrary(\"{}\");",
                self.so_name,
            );
            let ext_libs = self.ext_libs.split(',').collect::<Vec<&str>>();
            for ext_lib in ext_libs.iter() {
                if !ext_lib.to_owned().is_empty() {
                    push_f!(
                        load_lib_tokens,
                        "com.afoxer.rsbind.Common.loadLibrary(\"{}\");",
                        ext_lib.to_owned(),
                    );
                }
            }
            load_lib_tokens
        });
        body.push("}");

        class.body = body;
        Ok(())
    }
}
