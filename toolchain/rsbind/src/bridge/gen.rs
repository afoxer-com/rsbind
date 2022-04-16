use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use proc_macro2::{Ident, TokenStream};

use crate::ast::contract::desc::{StructDesc, TraitDesc};
use crate::ast::imp::desc::ImpDesc;
use crate::ast::AstResult;
use crate::errors::*;
use crate::ident;

///
/// Different strategy on generating a bridge mod.
///
pub(crate) trait ModGenStrategy {
    fn mod_name(&self, mod_name: &str) -> String;
    fn sdk_gen(&self, out_dir: &Path, file_name: &str, mod_names: &[String]) -> Result<()>;
    fn file_gen(
        &self,
        out_dir: &Path,
        file_name: &str,
        trait_descs: &[TraitDesc],
        struct_descs: &[StructDesc],
        imp_desc: &[ImpDesc],
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

        let traits = &self.ast_result.traits;
        let structs = &self.ast_result.structs;
        let imp_info = &self.ast_result.imps;

        let mut bridges: Vec<String> = vec![];
        for each_mod in traits {
            let mod_name = each_mod.0;
            let trait_descs = each_mod.1;
            let struct_descs = if let Some(vec) = structs.get(mod_name) {
                vec
            } else {
                &emtpy_vec
            };

            let out_mod_name = self.mod_gen_strategy.mod_name(mod_name);
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
        self.gen_common_code(self.bridge_dir).unwrap();

        // generate bridge/mod.rs
        self.gen_bridge_mod_code(self.bridge_dir, &bridges).unwrap();

        // generate _gen/mod.rs
        self.gen_mode_code(self.bridge_dir).unwrap();

        Ok(())
    }

    fn quote_free_rust_array(&self, fn_name: String, ty: TokenStream) -> TokenStream {
        let fn_name_ident = ident!(&fn_name);
        quote! {
            #[no_mangle]
            pub extern "C" fn #fn_name_ident(ptr: *mut #ty, length: i32) {
                let catch_result = catch_unwind(AssertUnwindSafe(|| {
                    let len: usize = length as usize;
                    unsafe { Vec::from_raw_parts(ptr, len, len); }
                }));
                match catch_result {
                    Ok(_) => {}
                    Err(e) => { println!("catch_unwind of `rsbind free_rust` error: {:?}", e); }
                };
            }
        }
    }

    ///
    /// generate common.rs
    ///
    fn gen_common_code(&self, bridge_dir: &Path) -> Result<()> {
        let crate_name = &self.crate_name.replace('-', "_");

        let int8_free_fn =
            self.quote_free_rust_array("free_i8_array".to_string(), quote! {i8});
        let int16_free_fn =
            self.quote_free_rust_array("free_i16_array".to_string(), quote! {i16});
        let int32_free_fn =
            self.quote_free_rust_array("free_i32_array".to_string(), quote! {i32});
        let int64_free_fn =
            self.quote_free_rust_array("free_i64_array".to_string(), quote! {i64});


        let tokens = quote! {
            use std::panic::*;
            use std::ffi::CString;
            use std::os::raw::c_char;
            use std::ffi::CStr;

            #[repr(C)]
            #[derive(Clone)]
            pub struct CInt8Array {
                pub ptr: * const i8,
                pub len: i32,
                pub free_ptr: extern "C" fn(*mut i8, i32),
            }

            #[repr(C)]
            #[derive(Clone)]
            pub struct CInt16Array {
                pub ptr: * const i16,
                pub len: i32,
                pub free_ptr: extern "C" fn(*mut i16, i32),
            }

            #[repr(C)]
            #[derive(Clone)]
            pub struct CInt32Array {
                pub ptr: * const i32,
                pub len: i32,
                pub free_ptr: extern "C" fn(*mut i32, i32),
            }

            #[repr(C)]
            #[derive(Clone)]
            pub struct CInt64Array {
                pub ptr: * const i64,
                pub len: i32,
                pub free_ptr: extern "C" fn(*mut i64, i32),
            }

            #int8_free_fn
            #int16_free_fn
            #int32_free_fn
            #int64_free_fn

            #[no_mangle]
            pub extern "C" fn free_str(ptr: *mut i8, length: i32) {
                let catch_result = catch_unwind(AssertUnwindSafe(|| unsafe {
                    let slice = std::slice::from_raw_parts_mut(ptr as (*mut u8), length as usize);
                    let cstr = CStr::from_bytes_with_nul_unchecked(slice);
                    CString::from(cstr);
                }));
                match catch_result {
                    Ok(_) => {}
                    Err(e) => {
                        println!("catch_unwind of `rsbind free_str` error: {:?}", e);
                    }
                };
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
    fn gen_bridge_mod_code(&self, out_dir: &Path, bridges: &[String]) -> Result<()> {
        let bridge_ident = bridges
            .iter()
            .map(|bridge| ident!(bridge))
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
    fn gen_mode_code(&self, out_dir: &Path) -> Result<()> {
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
