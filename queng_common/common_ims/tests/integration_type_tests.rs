use common_ims::ImsIntegrationType;
use std::fmt::Write;

#[test]
fn test_display_formatting() {
    assert_eq!(ImsIntegrationType::Data.to_string(), "Data");
    assert_eq!(ImsIntegrationType::Execution.to_string(), "Execution");
    assert_eq!(ImsIntegrationType::OMS.to_string(), "OMS");
}

#[test]
fn test_debug_display_consistency() {
    let variants = [
        ImsIntegrationType::Data,
        ImsIntegrationType::Execution,
        ImsIntegrationType::OMS,
    ];

    for variant in variants {
        let mut debug_output = String::new();
        write!(&mut debug_output, "{variant:?}").unwrap();
        assert_eq!(variant.to_string(), debug_output);
    }
}

#[test]
fn test_clone_equality() {
    let original = ImsIntegrationType::Data;
    let cloned = original;

    assert_eq!(original, cloned);
    assert!(matches!(cloned, ImsIntegrationType::Data));
}
