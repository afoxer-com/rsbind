pub(crate) enum Arch {
    Unknown,
    Linux32,
    Linux64,
    LinuxArm,
    LinuxArm64,
    Windows32,
    Windows64,
    WindowsArm64,
    Osx32,
    Osx64,
    OsxPpc,
    OsxArm64,
}

impl Arch {
    pub(crate) fn from_env() -> Self {
        if cfg!(target_os = "windows") && cfg!(target_arch = "x86") {
            Arch::Windows32
        } else if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") {
            Arch::Windows64
        } else if cfg!(target_os = "windows") && cfg!(target_arch = "aarch64") {
            Arch::WindowsArm64
        } else if cfg!(target_os = "macos") && cfg!(target_arch = "x86") {
            Arch::Osx32
        } else if cfg!(target_os = "macos") && cfg!(target_arch = "x86_64") {
            Arch::Osx64
        } else if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
            Arch::OsxArm64
        } else if cfg!(target_os = "macos") && cfg!(target_arch = "powerpc") {
            Arch::OsxPpc
        } else if cfg!(target_os = "linux") && cfg!(target_arch = "x86") {
            Arch::Linux32
        } else if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
            Arch::Linux64
        } else if cfg!(target_os = "linux") && cfg!(target_arch = "arm") {
            Arch::LinuxArm
        } else if cfg!(target_os = "linux") && cfg!(target_arch = "aarch64") {
            Arch::LinuxArm64
        } else {
            Arch::Unknown
        }
    }

    pub(crate) fn as_string(&self) -> String {
        match self {
            Arch::Unknown => "unknown",
            Arch::Linux32 => "linux_32",
            Arch::Linux64 => "linux_64",
            Arch::LinuxArm => "linux_arm",
            Arch::LinuxArm64 => "linux_arm64",
            Arch::Windows32 => "windows_32",
            Arch::Windows64 => "windows_64",
            Arch::WindowsArm64 => "windows_arm64",
            Arch::Osx32 => "osx_32",
            Arch::Osx64 => "osx_64",
            Arch::OsxPpc => "osx_ppc",
            Arch::OsxArm64 => "osx_arm64",
        }
        .to_string()
    }
}
