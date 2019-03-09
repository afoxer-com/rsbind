use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use toml;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub android: Option<Android>,
    pub ios: Option<Ios>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Android {
    pub ndk_stand_alone: Option<String>,
    pub rustc_param: Option<String>,
    pub arch: Option<Vec<String>>,
    pub arch_64: Option<Vec<String>>,
    pub arch_x86: Option<Vec<String>>,
    pub release: Option<bool>,
    pub namespace: Option<String>,
    pub so_name: Option<String>,
    pub ext_lib: Option<Vec<String>>,
    pub features_def: Option<Vec<String>>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Ios {
    pub rustc_param: Option<String>,
    pub arch_phone: Option<Vec<String>>,
    pub arch_simu: Option<Vec<String>>,
    pub release: Option<bool>,
    pub features_def: Option<Vec<String>>,
}

pub fn parse(prj_path: &PathBuf) -> Option<Config> {
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
