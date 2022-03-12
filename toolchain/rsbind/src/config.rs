use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::android::config::Android;
use crate::ios::config::Ios;
use crate::jar::config::Jar;
use crate::mac::config::Mac;

///
/// Configuration struct mapping from Rsbind.toml
///
#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub android: Option<Android>,
    pub ios: Option<Ios>,
    pub mac: Option<Mac>,
    pub jar: Option<Jar>,
}

///
/// Parsing Rsbind.toml to Config struct.
///
pub fn parse(prj_path: &Path) -> Option<Config> {
    let mut s = String::new();
    let path = prj_path.join("Rsbind.toml");
    if !path.exists() {
        println!("Rsbind.toml didn't found, skip parsing.");
        return None;
    }

    let mut f = File::open(&path).expect("open Rsbind.toml failed.");
    f.read_to_string(&mut s).expect("read Rsbind.toml failed.");
    toml::from_str::<Config>(&s).ok()
}
