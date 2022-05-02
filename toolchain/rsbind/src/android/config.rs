const NAMESPACE: &str = "com.afoxer.xxx.ffi";

const PHONE_ARCHS: [&str; 4] = [
    "armv7-linux-androideabi",
    "arm-linux-androideabi",
    "aarch64-linux-android",
    "i686-linux-android",
];
const SO_NAME: &str = "ffi";

///
/// Android Configuration struct
///
#[derive(Clone, Deserialize, Debug)]
pub struct Android {
    pub ndk_stand_alone: Option<String>,
    pub rustc_param: Option<String>,
    pub arch: Option<Vec<String>>,
    pub release: Option<bool>,
    pub namespace: Option<String>,
    pub so_name: Option<String>,
    pub ext_lib: Option<Vec<String>>,
    pub features_def: Option<Vec<String>>,
}

impl Default for Android {
    fn default() -> Self {
        let arch = Some(
            PHONE_ARCHS
                .iter()
                .copied()
                .map(|item| item.to_owned())
                .collect(),
        );

        Self {
            ndk_stand_alone: None,
            rustc_param: None,
            arch,
            release: Some(true),
            namespace: Some(NAMESPACE.to_owned()),
            so_name: Some(SO_NAME.to_owned()),
            ext_lib: None,
            features_def: None,
        }
    }
}

impl Android {
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

    pub fn archs(&self) -> Vec<String> {
        let default_phone_archs = PHONE_ARCHS.iter().copied().map(|a| a.to_owned()).collect();

        match self.arch {
            Some(ref arch) => arch.to_owned(),
            None => default_phone_archs,
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

    pub fn so_name(&self) -> String {
        match self.so_name {
            Some(ref so_name) => so_name.to_owned(),
            None => SO_NAME.to_owned(),
        }
    }

    pub fn features(&self) -> Vec<String> {
        match self.features_def {
            Some(ref features) => {
                let mut all_features = features.clone();
                all_features
            }
            None => vec![],
        }
    }
}
