use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::Config;
use syn::__private::str;

use crate::errors::ErrorKind::*;
use crate::errors::*;

use self::contract::desc::*;
use self::imp::desc::*;

pub(crate) mod contract;
pub(crate) mod imp;
pub(crate) mod types;

pub(crate) struct AstHandler {
    crate_name: String,
}

/// The ast result.
#[derive(Clone)]
pub(crate) struct AstResult {
    /// All the traits, key is mod name, value is all traits.
    pub traits: HashMap<String, Vec<TraitDesc>>,
    /// All the structs, key is mod name , value is all structs.
    pub structs: HashMap<String, Vec<StructDesc>>,
    /// All the implementations.
    pub imps: Vec<ImpDesc>,
}

impl Default for AstResult {
    fn default() -> Self {
        AstResult {
            traits: HashMap::default(),
            structs: HashMap::default(),
            imps: vec![],
        }
    }
}

struct IndexedContract {
    /// All the traits, key is mod name, value is all traits.
    pub traits: HashMap<String, Vec<TraitDesc>>,
    /// All the structs, key is mod name , value is all structs.
    pub structs: HashMap<String, Vec<StructDesc>>,
}

impl AstHandler {
    pub(crate) fn new(crate_name: String) -> AstHandler {
        AstHandler { crate_name }
    }

    pub(crate) fn parse(
        &self,
        origin_prj_path: &Path,
        config: &Option<Config>,
    ) -> Result<AstResult> {
        let mut contract_dir: String = "src/contract".to_string();
        let mut imp_dir: String = "src/imp".to_string();
        let mut contract_file: String = "src/contract.rs".to_string();
        let mut imp_file: String = "src/imp.rs".to_string();
        let rsbind_file: String = "src/rsbind.rs".to_string();

        let mut contract_str = "contract".to_string();
        let mut imp_str = "imp".to_string();
        if let &Some(ref cfg) = config {
            if let Some(ref common) = cfg.common {
                if let Some(ref contract_name_str) = common.contract_name {
                    if !contract_name_str.is_empty() {
                        contract_dir = format!("src/{}", contract_name_str);
                        contract_file = format!("src/{}.rs", contract_name_str);
                        contract_str = contract_name_str.clone();
                    }
                }

                if let Some(ref imp_name_str) = common.imp_name {
                    if !imp_name_str.is_empty() {
                        imp_dir = format!("src/{}", imp_name_str);
                        imp_file = format!("src/{}.rs", imp_name_str);
                        imp_str = imp_name_str.clone();
                    }
                }
            }
        }

        let imp_dir_path = origin_prj_path.join(imp_dir);
        let contract_dir_path = origin_prj_path.join(contract_dir);
        let contract_file = origin_prj_path.join(contract_file);
        let imp_file = origin_prj_path.join(imp_file);
        let rsbind_file = origin_prj_path.join(rsbind_file);

        let IndexedContract { traits, structs } = if contract_dir_path.is_dir()
            && contract_dir_path.exists()
        {
            self.parse_contract_from_dir(&contract_dir_path, &contract_str)?
        }
        // contract.rs
        else if contract_file.is_file() && contract_file.exists() {
            self.parse_from_file(&contract_file, &contract_str)?
        }
        // rsbind.rs
        else if rsbind_file.is_file() && rsbind_file.exists() {
            self.parse_from_file(&rsbind_file, "rsbind")?
        } else {
            println!("Err: No contract file in Rust found, should be contract.rs or contract dir or rsbind.rs");
            return Ok(AstResult::default());
        };

        let imps =
        // imp dir
        if imp_dir_path.is_dir() && imp_dir_path.exists() {
            imp::parser::parse_dir(&imp_dir_path, &imp_str)?
        } // imp.rs
        else if imp_file.is_file() && imp_file.exists()  {
            imp::parser::parse_from_file(&imp_file.to_string_lossy(), &imp_str)?
        } //rsbind.rs
        else if rsbind_file.is_file() && rsbind_file.exists() {
            imp::parser::parse_from_file(&rsbind_file.to_string_lossy(), "rsbind")?
        } else {
            println!("Err: No implementation file in Rust found, should be imp.rs or imp dir or rsbind.rs");
            return Ok(AstResult::default())
        };

        Ok(AstResult {
            traits,
            structs,
            imps,
        })
    }

    fn parse_contract_from_dir(
        &self,
        contract_dir_path: &Path,
        contract_name: &str,
    ) -> Result<IndexedContract> {
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
            let mod_path = format!("{}::{}", contract_name, &mod_name);
            let results =
                contract::parser::parse(self.crate_name.clone(), &path, &mod_path).unwrap();
            traits.insert(mod_name.to_owned(), results.traits);
            structs.insert(mod_name.to_owned(), results.structs);
        }

        Ok(IndexedContract { traits, structs })
    }

    fn parse_from_file(&self, contract_file: &Path, mod_mame: &str) -> Result<IndexedContract> {
        let mut traits = HashMap::new();
        let mut structs = HashMap::new();

        let results = contract::parser::parse(self.crate_name.clone(), &contract_file, mod_mame)?;
        traits.insert(mod_mame.to_owned(), results.traits);
        structs.insert(mod_mame.to_owned(), results.structs);

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
