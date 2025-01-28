use common_platform::PlatformType;

#[test]
fn test_as_u8() {
    assert_eq!(PlatformType::UnknownPlatform.as_u8(), 0);
    assert_eq!(PlatformType::LinuxX86_64.as_u8(), 1);
    assert_eq!(PlatformType::LinuxAarch64.as_u8(), 2);
    assert_eq!(PlatformType::MacOSAarch64.as_u8(), 3);
}

#[test]
fn test_as_u16() {
    assert_eq!(PlatformType::UnknownPlatform.as_u16(), 0);
    assert_eq!(PlatformType::LinuxX86_64.as_u16(), 1);
    assert_eq!(PlatformType::LinuxAarch64.as_u16(), 2);
    assert_eq!(PlatformType::MacOSAarch64.as_u16(), 3);
}

#[test]
fn test_as_u32() {
    assert_eq!(PlatformType::UnknownPlatform.as_u32(), 0);
    assert_eq!(PlatformType::LinuxX86_64.as_u32(), 1);
    assert_eq!(PlatformType::LinuxAarch64.as_u32(), 2);
    assert_eq!(PlatformType::MacOSAarch64.as_u32(), 3);
}

#[test]
fn test_from_u8() {
    assert_eq!(u8::from(PlatformType::UnknownPlatform), 0);
    assert_eq!(u8::from(PlatformType::LinuxX86_64), 1);
    assert_eq!(u8::from(PlatformType::LinuxAarch64), 2);
    assert_eq!(u8::from(PlatformType::MacOSAarch64), 3);
}

#[test]
fn test_from_u16() {
    assert_eq!(u16::from(PlatformType::UnknownPlatform), 0);
    assert_eq!(u16::from(PlatformType::LinuxX86_64), 1);
    assert_eq!(u16::from(PlatformType::LinuxAarch64), 2);
    assert_eq!(u16::from(PlatformType::MacOSAarch64), 3);
}

#[test]
fn test_from_u32() {
    assert_eq!(u32::from(PlatformType::UnknownPlatform), 0);
    assert_eq!(u32::from(PlatformType::LinuxX86_64), 1);
    assert_eq!(u32::from(PlatformType::LinuxAarch64), 2);
    assert_eq!(u32::from(PlatformType::MacOSAarch64), 3);
}

#[test]
fn test_display() {
    assert_eq!(
        format!("{}", PlatformType::UnknownPlatform),
        "UnknownPlatform"
    );
    assert_eq!(format!("{}", PlatformType::LinuxX86_64), "LinuxX86_64");
    assert_eq!(format!("{}", PlatformType::LinuxAarch64), "LinuxAarch64");
    assert_eq!(format!("{}", PlatformType::MacOSAarch64), "MacOSAarch64");
}
