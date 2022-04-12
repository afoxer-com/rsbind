use std::fs;
use std::path::PathBuf;

use crate::ast::AstResult;
use crate::errors::*;
use crate::swift::callback::{CallbackGen, InternalCallbackGen};
use crate::swift::internal::TraitGen;
use crate::swift::manager::ManagerGen;
use crate::swift::protocol::ProtocolGen;
use crate::swift::struct_::StructGen;
use crate::swift::wrapper::WrapperGen;

pub(crate) struct SwiftCodeGen<'a> {
    pub swift_gen_dir: &'a PathBuf,
    pub ast: &'a AstResult,
}

impl<'a> SwiftCodeGen<'a> {
    pub fn gen_swift_code(&self) -> Result<()> {
        // collect all the callbacks.
        let mut callbacks = vec![];
        for desc in self.ast.traits.iter() {
            let descs = desc.1;
            for each in descs.iter() {
                if each.is_callback {
                    callbacks.push(each);
                }
            }
        }

        // generate all the callbacks.
        for each in callbacks.clone().iter() {
            let gen = CallbackGen { desc: each };

            let callback_str = gen.gen()?;
            let file_name = format!("{}.swift", &each.name);
            let callback_path = self.swift_gen_dir.clone().join(file_name);
            fs::write(callback_path, callback_str)?;

            let gen = InternalCallbackGen {
                desc: each,
                callbacks: &callbacks,
            };

            let callback_str = gen.gen()?;
            let file_name = format!("Internal{}.swift", &each.name);
            let callback_path = self.swift_gen_dir.clone().join(file_name);
            fs::write(callback_path, callback_str)?;
        }

        // generate all the traits.
        for desc in self.ast.traits.iter() {
            let descs = desc.1;
            for each in descs.iter() {
                if !each.is_callback {
                    let gen = TraitGen {
                        desc: each,
                        callbacks: &callbacks,
                    };
                    let str = gen.gen()?;
                    let file_name = format!("Internal{}.swift", &each.name);
                    let path = self.swift_gen_dir.clone().join(file_name);
                    fs::write(path, str)?;

                    let gen = ProtocolGen { desc: each };
                    let str = gen.gen()?;
                    let file_name = format!("{}.swift", &each.name);
                    let path = self.swift_gen_dir.clone().join(file_name);
                    fs::write(path, str)?;

                    let gen = WrapperGen { desc: each };
                    let str = gen.gen()?;
                    let file_name = format!("Rust{}.swift", &each.name);
                    let path = self.swift_gen_dir.clone().join(file_name);
                    fs::write(path, str)?;
                }
            }
        }

        // generate all the structs
        for (_key, struct_descs) in self.ast.structs.iter() {
            for struct_desc in struct_descs.iter() {
                let gen = StructGen { desc: struct_desc };

                let struct_str = gen.gen()?;
                let file_name = format!("{}.swift", &struct_desc.name);
                let path = self.swift_gen_dir.join(file_name);
                fs::write(path, struct_str)?
            }
        }

        let manager_gen = ManagerGen { ast: self.ast };
        let manager_result = manager_gen.gen()?;
        let path = self.swift_gen_dir.join("RustLib.swift");
        fs::write(path, manager_result)?;

        Ok(())
    }
}
