use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use syn::__private::str;

use crate::errors::ErrorKind::*;
use crate::errors::*;

use self::contract::desc::*;
use self::imp::desc::*;

pub(crate) mod contract;
pub(crate) mod imp;
pub(crate) mod types;

const CONTRACT_DIR: &str = "src/contract";
const IMP_DIR: &str = "src/imp";
const CONTRACT_FILE: &str = "src/contract.rs";
const IMP_FILE: &str = "src/imp.rs";

pub(crate) struct AstHandler {
    crate_name: String,
}

/// The ast result after parsing contract and imp directories.
pub(crate) struct AstResult {
    /// All the traits in contract directory, key is mod name, value is all traits.
    pub traits: HashMap<String, Vec<TraitDesc>>,
    /// All the struct in contract directory, key is mod name , value is all structs.
    pub structs: HashMap<String, Vec<StructDesc>>,
    /// All the implementations.
    pub imps: Vec<ImpDesc>,
}

struct IndexedContract {
    /// All the traits in contract directory, key is mod name, value is all traits.
    pub traits: HashMap<String, Vec<TraitDesc>>,
    /// All the struct in contract directory, key is mod name , value is all structs.
    pub structs: HashMap<String, Vec<StructDesc>>,
}

impl AstHandler {
    pub(crate) fn new(crate_name: String) -> AstHandler {
        AstHandler { crate_name }
    }

    pub(crate) fn parse(&self, origin_prj_path: &Path) -> Result<AstResult> {
        let imp_dir_path = origin_prj_path.join(IMP_DIR);
        let contract_dir_path = origin_prj_path.join(CONTRACT_DIR);

        let IndexedContract { traits, structs } =
            if contract_dir_path.is_dir() && contract_dir_path.exists() {
                self.parse_contract_from_dir(&contract_dir_path)?
            } else {
                self.parse_from_file(origin_prj_path)?
            };

        let imps = if imp_dir_path.is_dir() && imp_dir_path.exists() {
            imp::parser::parse_dir(&imp_dir_path, "imp")?
        } else {
            let imp_file = origin_prj_path.join(IMP_FILE);
            imp::parser::parse(&imp_file.to_string_lossy(), "imp")?
        };

        Ok(AstResult {
            traits,
            structs,
            imps,
        })
    }

    fn parse_contract_from_dir(&self, contract_dir_path: &Path) -> Result<IndexedContract> {
        let mut traits = HashMap::new();
        let mut structs = HashMap::new();

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
            let mod_path = format!("contract::{}", &mod_name);
            let results =
                contract::parser::parse(self.crate_name.clone(), &path, &mod_path).unwrap();
            traits.insert(mod_name.to_owned(), results.traits);
            structs.insert(mod_name.to_owned(), results.structs);
        }

        Ok(IndexedContract { traits, structs })
    }

    fn parse_from_file(&self, origin_prj_path: &Path) -> Result<IndexedContract> {
        let contract_file = origin_prj_path.join(CONTRACT_FILE);

        let mut traits = HashMap::new();
        let mut structs = HashMap::new();

        let results = contract::parser::parse(self.crate_name.clone(), &contract_file, "contract")?;
        traits.insert("contract".to_owned(), results.traits);
        structs.insert("contract".to_owned(), results.structs);

        Ok(IndexedContract { traits, structs })
    }
}

impl AstResult {
    pub(crate) fn flush(self, ast_dir: &Path) -> Result<Self> {
        for each_mod in self.traits.iter() {
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

        for each_mod in self.structs.iter() {
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
