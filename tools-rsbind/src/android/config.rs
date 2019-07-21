const NAMESPACE: &str = "com.afoxer.xxx.ffi";

const PHONE_ARCHS: [&str; 2] = ["armv7-linux-androideabi", "arm-linux-androideabi"];
const PHONE64_ARCHS: [&str; 1] = ["aarch64-linux-android"];
const X86_ARCHS: [&str; 1] = ["i686-linux-android"];
const SO_NAME: &str = "ffi";

///
/// Android Configuration struct
/// 
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

impl Default for Android {
   fn default() -> Self {
       let arch = Some(PHONE_ARCHS
            .to_vec()
            .into_iter()
            .map(|item| item.to_owned())
            .collect());
        
        let arch_64 = Some(PHONE64_ARCHS
            .to_vec()
            .into_iter()
            .map(|item| item.to_owned())
            .collect());

        let arch_x86 = Some(X86_ARCHS
            .to_vec()
            .into_iter()
            .map(|item| item.to_owned())
            .collect());
        
       Self {
          ndk_stand_alone: None,
          rustc_param: Some("--features rsbind".to_owned()),
          arch,
          arch_64,
          arch_x86,
          release: Some(true),
          namespace: Some(NAMESPACE.to_owned()),
          so_name: Some(SO_NAME.to_owned()),
          ext_lib: None,
          features_def: None
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
        let init = "--features rsbind";
        match self.rustc_param {
            Some(ref rustc) => format!("{} {}", rustc, init),
            None => init.to_owned(),
        }
    }

    pub fn is_release(&self) -> bool {
        match self.release {
            Some(is_release) => is_release,
            None => true,
        }
    }

    pub fn release_str(&self) -> String {
        if self.is_release() {
            "--release".to_owned()
        } else {
            "".to_owned()
        }
    }

    pub fn phone_archs(&self) -> Vec<String> {
        let default_phone_archs = PHONE_ARCHS
            .to_vec()
            .into_iter()
            .map(|a| a.to_owned())
            .collect();

        match self.arch {
            Some(ref arch) => arch.to_owned(),
            None => default_phone_archs,
        }
    }

    pub fn phone64_archs(&self) -> Vec<String> {
        let default_phone64_archs = PHONE64_ARCHS
            .to_vec()
            .into_iter()
            .map(|a| a.to_owned())
            .collect();
            
            match self.arch_64 {
                Some(ref arch) => arch.to_owned() ,
                None => default_phone64_archs,
            }
    }

    pub fn x86_archs(&self) -> Vec<String> {
        let default_x86_archs = X86_ARCHS
            .to_vec()
            .into_iter()
            .map(|a| a.to_owned())
            .collect();

        match self.arch_x86 {
            Some(ref arch) => arch.to_owned(),
            None => default_x86_archs,
        }
    }

    pub fn ext_libs(&self) -> String {
        let ext_libs = match self.ext_lib {
            Some(ref ext_lib) => ext_lib.to_owned(),
            None => vec![],
        };

        let mut result = String::new();
        let mut index = 0;
        for ext_lib in ext_libs.iter() {
            if index == 0 {
                result = ext_lib.to_owned();
            } else if index < ext_libs.len() {
                result = format!("{},{}", &result, ext_lib)
            }
            index = index + 1;
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
            Some(ref features) => features.to_owned(),
            None => vec![],
        }
    }
}
