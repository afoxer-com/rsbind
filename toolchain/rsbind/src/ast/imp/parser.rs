//!
//! This module is used for parsing implementations of a ffi trait.
//!
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use syn;

use crate::errors::ErrorKind::*;
use crate::errors::*;

use super::desc::*;

///
/// Parse all the files in a directory.
///
pub(crate) fn parse_dir(dir: &Path) -> Result<Vec<ImpDesc>> {
    println!("begin parsing dir {:?}", dir);
    let mut result: Vec<ImpDesc> = vec![];

    let imp_dir = fs::read_dir(&dir).map_err(|e| ParseError(e.to_string()))?;

    for file in imp_dir {
        let real_file = file.map_err(|e| ParseError(e.to_string()))?;
        let file_path = real_file.path();
        let path_str = file_path.to_str().ok_or_else(|| {
            ParseError("can't get path from PathBuf when parsing imps.".to_string())
        })?;

        if file_path.ends_with("mod.rs") {
            continue;
        }

        println!("begin parsing file => {} ", path_str);
        let one_file_result = parse(path_str)?;
        for each in one_file_result {
            result.push(each)
        }
    }
    Ok(result)
}

///
/// parse a implementation file to description info.
///
pub(crate) fn parse(file: &str) -> Result<Vec<ImpDesc>> {
    // open file.
    let mut real_file = fs::File::open(file).map_err(|e| ParseError(e.to_string()))?;

    // read all content in file.
    let mut content = String::new();
    let _ = real_file
        .read_to_string(&mut content)
        .map_err(|e| ParseError(e.to_string()))?;

    // parse file to ast.
    let syn_file = syn::parse_file(&content).map_err(|e| ParseError(e.to_string()))?;

    parse_content(&syn_file, file)
}

fn parse_content(file: &syn::File, file_name: &str) -> Result<Vec<ImpDesc>> {
    let mut imp_descs: Vec<ImpDesc> = vec![];

    for item in file.items.iter() {
        let mut trait_ident = None;
        let mut impl_ident = None;
        match *item {
            syn::Item::Impl(ref imp_inner) => {
                if let Some((_, path, _)) = &imp_inner.trait_ {
                    trait_ident = Some(path.segments[path.segments.len() - 1].ident.to_string());
                }

                if let syn::Type::Path(ref path_inner) = *imp_inner.self_ty {
                    let segments = &path_inner.path.segments;
                    impl_ident = Some(segments[segments.len() - 1].ident.to_string());
                }
            }
            _ => continue,
        }

        let mod_name = PathBuf::from(file_name.to_string())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        match (trait_ident, impl_ident) {
            (Some(trait_name), Some(impl_name)) => {
                let imp_desc = ImpDesc {
                    name: impl_name,
                    contract: trait_name,
                    mod_name,
                };
                imp_descs.push(imp_desc)
            }
            _ => continue,
        }
    }

    println!("final imps => {:#?}", imp_descs);
    Ok(imp_descs)
}
