///
/// Mac Configuration struct
///
#[derive(Clone, Deserialize, Debug)]
pub struct Mac {
    pub rustc_param: Option<String>,
    pub release: Option<bool>,
    pub features_def: Option<Vec<String>>,
    pub contract_name: Option<String>,
    pub imp_name: Option<String>,
}

impl Default for Mac {
    fn default() -> Self {
        Self {
            rustc_param: None,
            release: Some(true),
            features_def: None,
            contract_name: None,
            imp_name: None
        }
    }
}

impl Mac {
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

    pub fn features(&self) -> Vec<String> {
        match self.features_def {
            Some(ref features) => features.clone(),
            None => vec![],
        }
    }
}
