use std::fs;
use std::path::Path;

use crate::errors::ErrorKind::*;
use crate::errors::*;
use crate::unzip;

const MAGIC_NUM: &str = "*521%";

///
/// Unpack the bridge project.
///
pub(crate) struct Unpack<'a> {
    pub path: &'a Path,
    pub host_crate: &'a str,
    pub buf: &'a [u8],
    pub features: &'a Vec<String>,
}

impl<'a> Unpack<'a> {
    pub(crate) fn unpack(&self) -> Result<()> {
        if self.path.exists() {
            std::fs::remove_dir_all(&self.path)?;
        }
        fs::create_dir_all(&self.path)?;

        unzip::unzip_to(self.buf, self.path)?;

        let manifest_path = self.path.join("Cargo.toml");
        let manifest_text = fs::read_to_string(&manifest_path).map_err(|e| {
            FileError(format!(
                "read rust project Cargo.toml error: {:?} path = {:?}",
                e, manifest_path
            ))
        })?;

        // replace the crate name in manifest.
        let replaced =
            manifest_text.replace(&format!("$({}-host_crate)", MAGIC_NUM), self.host_crate);
        let replaced = replaced.replace(
            &format!("$({}-host_crate_underscore)", MAGIC_NUM),
            &self.host_crate.replace('-', "_"),
        );

        // add some features defination.
        let mut feature_defs = String::new();
        for feature in self.features.iter() {
            feature_defs = format!("{}\n{} = []", feature_defs, feature);
        }

        let replaced = replaced.replace(&format!("$({}-features)", MAGIC_NUM), &feature_defs);
        fs::write(manifest_path, replaced)
            .map_err(|e| FileError(format!("write rust project Cargo.toml error {:?}", e)))?;

        // replace the crate name in lib.rs.
        // let lib_file = self.path.join("src").join("lib.rs");
        // let lib_text = fs::read_to_string(&lib_file)
        //     .map_err(|e| FileError(format!("read lib.rs error, {:?}", e)))?;
        //
        // let lib_replaced =
        //     lib_text.replace(&format!("$({}-host_crate)", MAGIC_NUM), self.host_crate);
        // let lib_replaced = lib_replaced.replace(
        //     &format!("$({}-host_crate_underscore)", MAGIC_NUM),
        //     &self.host_crate.replace('-', "_"),
        // );
        //
        // fs::write(lib_file, lib_replaced)
        //     .map_err(|e| FileError(format!("write lib.rs error, {}", e)))?;

        Ok(())
    }
}
