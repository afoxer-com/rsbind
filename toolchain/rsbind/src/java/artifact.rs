use std::fs;
use std::path::PathBuf;

use crate::ast::AstResult;
use crate::errors::*;
use crate::java::callback::{CallbackGen, InnerCallbackGen};
use crate::java::interface::InterfaceGen;
use crate::java::internal::InnerTraitGen;
use crate::java::manager::ManagerGen;
use crate::java::struct_::StructGen;
use crate::java::wrapper::WrapperGen;

pub(crate) struct JavaCodeGen<'a> {
    pub java_gen_dir: &'a PathBuf,
    pub ast: &'a AstResult,
    pub namespace: String,
    pub so_name: String,
    pub ext_libs: String,
}

impl<'a> JavaCodeGen<'a> {
    pub(crate) fn gen_files(&self) -> Result<()> {
        // get the java_gen dir string
        println!("get java_gen dir string");
        // fs::write(&java_gen_path, );

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

            let gen = InnerCallbackGen {
                desc: each,
                pkg: self.namespace.clone(),
            };

            let callback_str = gen.gen()?;
            let file_name = format!("Internal{}.java", &each.name);
            let callback_path = self.java_gen_dir.clone().join(file_name);
            fs::write(callback_path, callback_str)?;
        }

        // generate all the traits.
        for desc in self.ast.traits.iter() {
            let descs = desc.1;
            for each in descs.iter() {
                if !each.is_callback {
                    let gen = InnerTraitGen {
                        desc: each,
                        pkg: self.namespace.clone(),
                        callbacks: callbacks.clone(),
                    };
                    let str = gen.gen()?;
                    let file_name = format!("Internal{}.java", &each.name);
                    let path = self.java_gen_dir.clone().join(file_name);
                    fs::write(path, str)?;

                    let gen = WrapperGen {
                        desc: each,
                        pkg: self.namespace.clone(),
                    };
                    let str = gen.gen()?;
                    let file_name = format!("Rust{}.java", &each.name);
                    let path = self.java_gen_dir.clone().join(file_name);
                    fs::write(path, str)?;

                    let gen = InterfaceGen {
                        desc: each,
                        pkg: self.namespace.clone(),
                    };

                    let str = gen.gen()?;
                    let file_name = format!("{}.java", &each.name);
                    let path = self.java_gen_dir.clone().join(file_name);
                    fs::write(path, str)?;
                }
            }
        }

        // generate all the structs
        for (_key, struct_descs) in self.ast.structs.iter() {
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

        let manager_gen = ManagerGen {
            ast: self.ast,
            pkg: self.namespace.clone(),
            so_name: self.so_name.clone(),
            ext_libs: self.ext_libs.clone(),
        };
        let manager_result = manager_gen.gen()?;
        let path = self.java_gen_dir.join("RustLib.java");
        fs::write(path, manager_result)?;

        Ok(())
    }
}
