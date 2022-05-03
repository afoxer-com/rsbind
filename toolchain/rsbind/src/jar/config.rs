const NAMESPACE: &str = "com.afoxer.xxx.ffi";
const DYLIB_NAME: &str = "ffi";

///
/// Jar Configuration struct
///
#[derive(Clone, Deserialize, Debug)]
pub struct Jar {
    pub ndk_stand_alone: Option<String>,
    pub rustc_param: Option<String>,
    pub release: Option<bool>,
    pub namespace: Option<String>,
    pub dylib_name: Option<String>,
    pub ext_lib: Option<Vec<String>>,
    pub features_def: Option<Vec<String>>,
}

impl Default for Jar {
    fn default() -> Self {
        Self {
            ndk_stand_alone: None,
            rustc_param: None,
            release: Some(true),
            namespace: Some(NAMESPACE.to_owned()),
            dylib_name: Some(DYLIB_NAME.to_owned()),
            ext_lib: None,
            features_def: None,
        }
    }
}

impl Jar {
    pub fn namespace(&self) -> String {
        match self.namespace {
            Some(ref namespace) => namespace.to_owned(),
            None => NAMESPACE.to_owned(),
        }
    }

    pub fn rustc_param(&self) -> String {
        match self.rustc_param {
            Some(ref rustc) => rustc.to_owned(),
            None => "".to_owned(),
        }
    }

    pub fn is_release(&self) -> bool {
        self.release.unwrap_or(true)
    }

    pub fn release_str(&self) -> String {
        if self.is_release() {
            "--release".to_owned()
        } else {
            "".to_owned()
        }
    }

    pub fn ext_libs(&self) -> String {
        let ext_libs = match self.ext_lib {
            Some(ref ext_lib) => ext_lib.to_owned(),
            None => vec![],
        };

        let mut result = String::new();
        for (index, ext_lib) in ext_libs.iter().enumerate() {
            if index == 0 {
                result = ext_lib.to_owned();
            } else if index < ext_libs.len() {
                result = format!("{},{}", &result, ext_lib)
            }
        }

        result
    }

    pub fn dylib_name(&self) -> String {
        match self.dylib_name {
            Some(ref dylib_name) => dylib_name.to_owned(),
            None => DYLIB_NAME.to_owned(),
        }
    }

    pub fn features(&self) -> Vec<String> {
        match self.features_def {
            Some(ref features) => features.clone(),
            None => vec![],
        }
    }
}
