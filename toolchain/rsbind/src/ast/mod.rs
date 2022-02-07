pub(crate) mod contract;
pub(crate) mod imp;
pub(crate) mod types;

use self::contract::desc::*;
use self::imp::desc::*;
use errors::ErrorKind::*;
use errors::*;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

const CONTRACT_DIR: &str = "src/contract";
const IMP_DIR: &str = "src/imp";

pub(crate) struct AstHandler {
    crate_name: String,
}

/// The ast result after parsing contract and imp directories.
pub(crate) struct AstResult {
    /// All the traits in contract directory, key is mod name, value is all traits.
    pub trait_descs: HashMap<String, Vec<TraitDesc>>,
    /// All the struct in contract directory, key is mod name , value is all structs.
    pub struct_descs: HashMap<String, Vec<StructDesc>>,
    /// All the implementations.
    pub imp_desc: Vec<ImpDesc>,
}

impl AstHandler {
    pub(crate) fn new(crate_name: String) -> AstHandler {
        AstHandler { crate_name }
    }

    pub(crate) fn parse(&self, origin_prj_path: &PathBuf) -> Result<AstResult> {
        let imp_dir_path = origin_prj_path.join(IMP_DIR);
        let imp_desc = imp::parser::parse_dir(&imp_dir_path)?;

        let mut trait_desc_map = HashMap::new();
        let mut struct_desc_map = HashMap::new();
        let contract_dir_path = origin_prj_path.join(CONTRACT_DIR);
        let contract_dir = fs::read_dir(&contract_dir_path)?;
        for file in contract_dir {
            let path = file?.path();

            // skip mod.rs
            if path.ends_with("mod.rs") {
                continue;
            }

            println!("begin parse contract file for {:?}.", &path);
            let mod_name = PathBuf::from(&path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let results = contract::parser::parse(self.crate_name.clone(), &path).unwrap();
            trait_desc_map.insert(mod_name.to_owned(), results.0);
            struct_desc_map.insert(mod_name.to_owned(), results.1);
        }

        Ok(AstResult {
            trait_descs: trait_desc_map,
            struct_descs: struct_desc_map,
            imp_desc,
        })
    }
}

impl AstResult {
    pub(crate) fn flush(self, ast_dir: &PathBuf) -> Result<Self> {
        for each_mod in self.trait_descs.iter() {
            let trait_desc_list = each_mod.1;
            for trait_desc in trait_desc_list {
                let json =
                    serde_json::to_string(trait_desc).map_err(|e| GenerateError(e.to_string()))?;

                let file_name = ast_dir.join(&format!(
                    "{}_{}.json",
                    &trait_desc.mod_name, &trait_desc.name
                ));
                let mut ast_file = fs::File::create(&file_name)?;
                ast_file.write_all(&json.into_bytes())?;
            }
        }

        for each_mod in self.struct_descs.iter() {
            let struct_desc_list = each_mod.1;
            for struct_desc in struct_desc_list {
                let json =
                    serde_json::to_string(struct_desc).map_err(|e| GenerateError(e.to_string()))?;

                let file_name = ast_dir.join(&format!(
                    "{}_{}.json",
                    &struct_desc.mod_name, &struct_desc.name
                ));
                let mut ast_file = fs::File::create(&file_name)?;
                ast_file.write_all(&json.into_bytes())?;
            }
        }

        Ok(self)
    }
}
