use std::fs;
use std::io::Write;
use std::path::PathBuf;

use proc_macro2::{Ident, Span};

use crate::ast::contract::desc::{StructDesc, TraitDesc};
use crate::ast::imp::desc::ImpDesc;
use crate::ast::AstResult;
use crate::errors::*;

///
/// Different strategy on generating a bridge mod.
///
pub(crate) trait ModGenStrategy {
    fn mod_name(&self, mod_name: &str) -> String;
    fn sdk_gen(&self, out_dir: &PathBuf, file_name: &str, mod_names: &Vec<String>) -> Result<()>;
    fn file_gen(
        &self,
        out_dir: &PathBuf,
        file_name: &str,
        trait_descs: &Vec<TraitDesc>,
        struct_descs: &Vec<StructDesc>,
        imp_desc: &Vec<ImpDesc>,
    ) -> Result<()>;
}

///
/// The executor for generating a bridge mod
///
pub(crate) struct BridgeModGen<'a, T: ModGenStrategy> {
    pub ast_result: &'a AstResult,
    pub bridge_dir: &'a PathBuf,
    pub mod_gen_strategy: T,
    pub crate_name: String,
}

impl<'a, T: ModGenStrategy> BridgeModGen<'a, T> {
    ///
    /// generate the bridge files
    ///
    pub(crate) fn gen_bridges(&self) -> Result<()> {
        let emtpy_vec = vec![];

        let traits = &self.ast_result.trait_descs;
        let structs = &self.ast_result.struct_descs;
        let imp_info = &self.ast_result.imp_desc;

        let mut bridges: Vec<String> = vec![];
        for each_mod in traits {
            let mod_name = each_mod.0;
            let trait_descs = each_mod.1;
            let struct_descs = if let Some(vec) = structs.get(mod_name) {
                vec
            } else {
                &emtpy_vec
            };

            let out_mod_name = self.mod_gen_strategy.mod_name(&mod_name);
            let out_file_name = format!("{}.rs", &out_mod_name);

            self.mod_gen_strategy
                .file_gen(
                    self.bridge_dir,
                    &out_file_name,
                    trait_descs,
                    struct_descs,
                    imp_info,
                )
                .unwrap();

            bridges.push(out_mod_name)
        }

        self.mod_gen_strategy
            .sdk_gen(self.bridge_dir, "sdk.rs", &bridges)
            .unwrap();
        bridges.push("sdk".to_owned());

        // generate common.rs
        self.gen_common_code(&self.bridge_dir).unwrap();

        // generate bridge/mod.rs
        self.gen_bridge_mod_code(&self.bridge_dir, &bridges)
            .unwrap();

        // generate _gen/mod.rs
        self.gen_mode_code(&self.bridge_dir).unwrap();

        Ok(())
    }

    ///
    /// generate common.rs
    ///
    fn gen_common_code(&self, bridge_dir: &PathBuf) -> Result<()> {
        let crate_name = &self.crate_name.replace("-", "_");
        let free_fun_ident = Ident::new(&format!("{}_free_rust", crate_name), Span::call_site());
        let free_str_fun_ident = Ident::new(&format!("{}_free_str", crate_name), Span::call_site());

        let tokens = quote! {
            use std::panic::*;
            use std::ffi::CString;
            use std::os::raw::c_char;

            #[no_mangle]
            pub extern "C" fn #free_fun_ident(ptr: *mut u8, length: u32) {
                let catch_result = catch_unwind(AssertUnwindSafe(|| {
                    let len: usize = length as usize;
                    unsafe {
                        Vec::from_raw_parts(ptr, len, len);
                    }
                }));

                match catch_result {
                    Ok(_) => {}
                    Err(e) => {
                        println!("catch_unwind of `rsbind free_rust` error: {:?}", e);
                    }
                };
            }

            #[no_mangle]
            pub extern "C" fn #free_str_fun_ident(ptr: *mut c_char) {
                let catch_result = catch_unwind(AssertUnwindSafe(|| {
                    unsafe {
                        CString::from_raw(ptr);
                    }
                }));

                match catch_result {
                    Ok(_) => {}
                    Err(e) => {
                        println!("catch_unwind of `rsbind free_str` error: {:?}", e);
                    }
                };
            }

            #[repr(C)]
            pub struct CInt8Array {
                pub ptr: * const i8,
                pub len: i32
            }

            #[repr(C)]
            pub struct CUInt8Array {
                pub ptr: * const u8,
                pub len: i32
            }
        };

        let file_path = bridge_dir.join("common.rs");
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(&tokens.to_string().into_bytes()).unwrap();

        Ok(())
    }

    ///
    /// generate mod.rs in [c/java]/bridge dir.
    ///
    fn gen_bridge_mod_code(&self, out_dir: &PathBuf, bridges: &Vec<String>) -> Result<()> {
        let bridge_ident = bridges
            .iter()
            .map(|bridge| Ident::new(bridge, Span::call_site()))
            .collect::<Vec<Ident>>();

        let bridge_mod_tokens = quote! {
            # (pub mod #bridge_ident;)*
            pub mod common;
        };

        let bridget_mod_file_path = out_dir.join("mod.rs");
        let mut bridge_mod_file = fs::File::create(&bridget_mod_file_path).unwrap();
        bridge_mod_file
            .write_all(&bridge_mod_tokens.to_string().into_bytes())
            .unwrap();

        Ok(())
    }

    ///
    /// generate the mode.rs in src/[c/java] directory.
    ///
    fn gen_mode_code(&self, out_dir: &PathBuf) -> Result<()> {
        let gen_mod_tokens = quote! {
            pub mod bridge;
        };

        let gen_mod_path = out_dir.parent().unwrap().join("mod.rs");
        let mut gen_mod_file = fs::File::create(&gen_mod_path).unwrap();
        gen_mod_file
            .write_all(&gen_mod_tokens.to_string().into_bytes())
            .unwrap();
        Ok(())
    }
}
