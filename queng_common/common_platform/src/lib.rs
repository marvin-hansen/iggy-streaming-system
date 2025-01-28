#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum PlatformType {
    /// The unknown platform type.
    #[default]
    UnknownPlatform,
    /// The Linux X86_64 platform type.
    LinuxX86_64,
    /// The Linux aarch64 platform type.
    LinuxAarch64,
    /// The MacOS aarch64 (Apple Silicon) platform type.
    MacOSAarch64,
}

impl PlatformType {
    pub fn as_u8(&self) -> u8 {
        (*self).into()
    }

    pub fn as_u16(&self) -> u16 {
        (*self).into()
    }

    pub fn as_u32(&self) -> u32 {
        (*self).into()
    }
}

impl From<PlatformType> for u8 {
    fn from(platform: PlatformType) -> Self {
        match platform {
            PlatformType::UnknownPlatform => 0,
            PlatformType::LinuxX86_64 => 1,
            PlatformType::LinuxAarch64 => 2,
            PlatformType::MacOSAarch64 => 3,
        }
    }
}

impl From<PlatformType> for u16 {
    fn from(platform: PlatformType) -> Self {
        match platform {
            PlatformType::UnknownPlatform => 0,
            PlatformType::LinuxX86_64 => 1,
            PlatformType::LinuxAarch64 => 2,
            PlatformType::MacOSAarch64 => 3,
        }
    }
}

impl From<PlatformType> for u32 {
    fn from(platform: PlatformType) -> Self {
        match platform {
            PlatformType::UnknownPlatform => 0,
            PlatformType::LinuxX86_64 => 1,
            PlatformType::LinuxAarch64 => 2,
            PlatformType::MacOSAarch64 => 3,
        }
    }
}

impl std::fmt::Display for PlatformType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatformType::UnknownPlatform => write!(f, "UnknownPlatform"),
            PlatformType::LinuxX86_64 => write!(f, "LinuxX86_64"),
            PlatformType::LinuxAarch64 => write!(f, "LinuxAarch64"),
            PlatformType::MacOSAarch64 => write!(f, "MacOSAarch64"),
        }
    }
}
