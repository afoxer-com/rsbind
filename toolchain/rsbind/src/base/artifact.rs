use crate::ast::contract::desc::{StructDesc, TraitDesc};
use crate::errors::*;
use crate::AstResult;
use heck::{ToKebabCase, ToSnakeCase, ToUpperCamelCase};
use std::fs;
use std::path::PathBuf;

macro_rules! filename {
    ($style: expr, $ext: expr, $fmt: expr, $($values: expr),*) => {
        match $style {
            FileNameStyle::CamelCase => {
                let name = format!($fmt, $($values),*).to_upper_camel_case();
                format!("{}.{}", name, $ext)
            }
            FileNameStyle::SnakeCase => {
                let name = format!($fmt, $($values),*).to_snake_case();
                format!("{}.{}", name, $ext)
            }
            FileNameStyle::KebabCase => {
                let name = format!($fmt, $($values),*).to_kebab_case();
                format!("{}.{}", name, $ext)
            }
        }
    };
}

pub(crate) struct NativeCodeGen<'a, Extra>
where
    Extra: Clone,
{
    pub gen_dir: &'a PathBuf,
    pub file_ext: String,
    pub file_name_style: FileNameStyle,
    pub ast: &'a AstResult,
    pub extra: Extra,
    pub strategy: NativeGenStrategy<'a, Extra>,
}

pub(crate) struct GenContext<'a, Extra>
where
    Extra: Clone,
{
    pub ast: &'a AstResult,
    pub extra: Extra,
    pub callbacks: Vec<TraitDesc>,
}

pub(crate) struct NativeGenStrategy<'a, Extra>
where
    Extra: Clone,
{
    pub gen_bridge_callback: Box<dyn Fn(&GenContext<'a, Extra>, &TraitDesc) -> Result<String>>,
    pub gen_callback: Box<dyn Fn(&GenContext<'a, Extra>, &TraitDesc) -> Result<String>>,
    pub gen_bridge_trait: Box<dyn Fn(&GenContext<'a, Extra>, &TraitDesc) -> Result<String>>,
    pub gen_wrapper_trait: Box<dyn Fn(&GenContext<'a, Extra>, &TraitDesc) -> Result<String>>,
    pub gen_trait: Box<dyn Fn(&GenContext<'a, Extra>, &TraitDesc) -> Result<String>>,
    pub gen_struct: Box<dyn Fn(&GenContext<'a, Extra>, &StructDesc) -> Result<String>>,
    pub gen_manager: Box<dyn Fn(&GenContext<'a, Extra>) -> Result<String>>,
}

impl<'a, Extra> NativeCodeGen<'a, Extra>
where
    Extra: Clone,
{
    pub(crate) fn gen_files(&self) -> Result<()> {
        // collect all the callbacks.
        let mut callbacks = vec![];
        for desc in self.ast.traits.iter() {
            let descs = desc.1;
            for each in descs.iter() {
                if each.is_callback {
                    callbacks.push(each.clone());
                }
            }
        }

        let ctx = GenContext {
            ast: self.ast,
            extra: self.extra.clone(),
            callbacks: callbacks.clone(),
        };

        // generate all the callbacks.
        for each in callbacks.clone().iter() {
            let callback_str = (*self.strategy.gen_callback)(&ctx, each)?;
            let file_name = filename!(
                self.file_name_style.clone(),
                &self.file_ext,
                "{}",
                &each.name
            );
            if !callback_str.is_empty() {
                let callback_path = self.gen_dir.clone().join(file_name);
                fs::write(callback_path, callback_str)?;
            }

            let callback_str = (*self.strategy.gen_bridge_callback)(&ctx, each)?;
            let file_name = filename!(
                self.file_name_style.clone(),
                &self.file_ext,
                "Internal{}",
                &each.name
            );
            if !callback_str.is_empty() {
                let callback_path = self.gen_dir.clone().join(file_name);
                fs::write(callback_path, callback_str)?;
            }
        }

        // generate all the traits.
        for desc in self.ast.traits.iter() {
            let descs = desc.1;
            for each in descs.iter() {
                if !each.is_callback {
                    let str = (*self.strategy.gen_bridge_trait)(&ctx, each)?;
                    let file_name = filename!(
                        self.file_name_style.clone(),
                        &self.file_ext,
                        "Internal{}",
                        &each.name
                    );
                    if !str.is_empty() {
                        let path = self.gen_dir.clone().join(file_name);
                        fs::write(path, str)?;
                    }

                    let str = (*self.strategy.gen_wrapper_trait)(&ctx, each)?;
                    let file_name = filename!(
                        self.file_name_style.clone(),
                        &self.file_ext,
                        "Rust{}",
                        &each.name
                    );
                    if !str.is_empty() {
                        let path = self.gen_dir.clone().join(file_name);
                        fs::write(path, str)?;
                    }

                    let str = (*self.strategy.gen_trait)(&ctx, each)?;
                    let file_name = filename!(
                        self.file_name_style.clone(),
                        &self.file_ext,
                        "{}",
                        &each.name
                    );
                    if !str.is_empty() {
                        let path = self.gen_dir.clone().join(file_name);
                        fs::write(path, str)?;
                    }
                }
            }
        }

        // generate all the structs
        for (_key, struct_descs) in self.ast.structs.iter() {
            for struct_desc in struct_descs.iter() {
                let struct_str = (*self.strategy.gen_struct)(&ctx, struct_desc)?;
                let file_name = filename!(
                    self.file_name_style.clone(),
                    &self.file_ext,
                    "{}",
                    &struct_desc.name
                );
                if !struct_str.is_empty() {
                    let path = self.gen_dir.join(file_name);
                    fs::write(path, struct_str)?
                }
            }
        }

        let manager_result = (*self.strategy.gen_manager)(&ctx)?;
        if !manager_result.is_empty() {
            let path = self.gen_dir.join(filename!(
                self.file_name_style.clone(),
                &self.file_ext,
                "RustLib",
            ));
            fs::write(path, manager_result)?;
        }

        Ok(())
    }
}

#[derive(Clone)]
pub(crate) enum FileNameStyle {
    CamelCase,
    SnakeCase,
    KebabCase,
}
