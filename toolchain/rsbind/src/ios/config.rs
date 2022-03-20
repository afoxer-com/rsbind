const ARCHS: [&str; 4] = [
    "aarch64-apple-ios",
    "armv7-apple-ios",
    "i386-apple-ios",
    "x86_64-apple-ios",
];

///
/// iOS Configuration struct
///
#[derive(Clone, Deserialize, Debug)]
pub struct Ios {
    pub rustc_param: Option<String>,
    pub arch: Option<Vec<String>>,
    pub release: Option<bool>,
    pub features_def: Option<Vec<String>>,
}

impl Default for Ios {
    fn default() -> Self {
        let arch = Some(ARCHS.iter().copied().map(|item| item.to_owned()).collect());

        Self {
            rustc_param: None,
            arch,
            release: Some(true),
            features_def: None,
        }
    }
}

impl Ios {
    pub fn rustc_param(&self) -> String {
        match self.rustc_param {
            Some(ref rustc) => rustc.clone(),
            None => "".to_owned(),
        }
    }

    pub fn release_str(&self) -> String {
        if self.is_release() {
            "--release".to_owned()
        } else {
            "".to_owned()
        }
    }

    pub fn is_release(&self) -> bool {
        self.release.unwrap_or(true)
    }

    pub fn archs(&self) -> Vec<String> {
        let default_phone_archs = ARCHS.iter().copied().map(|a| a.to_owned()).collect();

        match self.arch {
            Some(ref arch) => arch.clone(),
            None => default_phone_archs,
        }
    }

    pub fn features(&self) -> Vec<String> {
        match self.features_def {
            Some(ref features) => features.clone(),
            None => vec![],
        }
    }
}
