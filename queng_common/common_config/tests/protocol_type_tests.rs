use common_config::ProtocolType;

#[test]
fn test_default() {
    let protocol = ProtocolType::default();
    assert_eq!(protocol, ProtocolType::UnknownProtocol);
}

#[test]
fn test_from() {
    assert_eq!(ProtocolType::from(0x1_i32), ProtocolType::GRPC);
    assert_eq!(ProtocolType::from(0x2_i32), ProtocolType::HTTP);
    assert_eq!(ProtocolType::from(0x3_i32), ProtocolType::UDP);
    assert_eq!(ProtocolType::from(0x4_i32), ProtocolType::UnknownProtocol);
}

#[test]
fn test_protocol_type_from_string() {
    let grpc_type = ProtocolType::from_string("GRPC");
    assert_eq!(grpc_type, Some(ProtocolType::GRPC));

    let http_type = ProtocolType::from_string("HTTP");
    assert_eq!(http_type, Some(ProtocolType::HTTP));

    let udp_type = ProtocolType::from_string("UDP");
    assert_eq!(udp_type, Some(ProtocolType::UDP));

    let invalid_type = ProtocolType::from_string("invalid");
    assert_eq!(invalid_type, None);
}

#[test]
fn test_debug() {
    let e1 = ProtocolType::GRPC;
    assert_eq!(format!("{e1:?}"), "GRPC");

    let e2 = ProtocolType::HTTP;
    assert_eq!(format!("{e2:?}"), "HTTP");

    let e3 = ProtocolType::UDP;
    assert_eq!(format!("{e3:?}"), "UDP");
}

#[test]
fn test_display() {
    let e1 = ProtocolType::GRPC;
    assert_eq!(format!("{e1}"), "GRPC");

    let e2 = ProtocolType::HTTP;
    assert_eq!(format!("{e2}"), "HTTP");

    let e3 = ProtocolType::UDP;
    assert_eq!(format!("{e3}"), "UDP");
}
