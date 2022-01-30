use errors::ErrorKind::*;
use errors::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;

#[derive(Clone, Deserialize, Debug)]
pub struct Manifest {
    pub package: Package,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Package {
    pub name: String,
}

/// Parse the Cargo.toml for a given path
pub fn manifest(manifest_path: &Path) -> Result<Manifest> {
    let mut s = String::new();
    let mut f = File::open(manifest_path)
        .map_err(|e| FileError(format!("open cargo toml error, {:?}", e)))?;
    f.read_to_string(&mut s)
        .map_err(|e| FileError(format!("read cargo toml error, {:?}", e)))?;

    toml::from_str::<Manifest>(&s).map_err(|x| x.into())
}
