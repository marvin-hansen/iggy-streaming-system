use common_config::{Endpoint, ProtocolType};

#[test]
fn test_new() {
    let name = "name".to_string();
    let version = 1;
    let uri = "/".to_string();
    let port = 8080;
    let protocol = ProtocolType::GRPC;

    let endpoint = Endpoint::new(name, version, uri, port, protocol);

    assert_eq!(endpoint.name(), "name");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(endpoint.uri(), &String::from("/"));
    assert_eq!(endpoint.port(), 8080);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);
}

#[test]
fn test_default() {
    let endpoint = Endpoint::default();

    assert_eq!(endpoint.name(), "");
    assert_eq!(endpoint.version(), 0);
    assert_eq!(endpoint.uri(), "");
    assert_eq!(endpoint.port(), 0);
    assert_eq!(endpoint.protocol(), ProtocolType::UnknownProtocol);
}

#[test]
fn test_host_endpoint() {
    let endpoint = Endpoint::default();

    let host_endpoint = endpoint.host_endpoint();

    assert_eq!(host_endpoint.host_uri(), "");
    assert_eq!(host_endpoint.port(), 0);
}

#[test]
fn test_display() {
    let name = "name".to_string();
    let version = 1;
    let uri = "/".to_string();
    let port = 8080;
    let protocol = ProtocolType::GRPC;

    let endpoint = Endpoint::new(name, version, uri, port, protocol);

    assert_eq!(
        endpoint.to_string(),
        "name: name,  version: 1,  port: 8080,  uri: /,  protocol: GRPC"
    );
}
