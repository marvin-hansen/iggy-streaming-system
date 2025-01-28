use common_config::HostEndpoint;

#[test]
fn test_new() {
    let host_uri = "http://localhost";
    let port = 8080;
    let host_endpoint = HostEndpoint::new(host_uri, port);

    assert_eq!(host_endpoint.host_uri(), "http://localhost");
    assert_eq!(host_endpoint.port(), 8080);
}

#[test]
fn test_host_uri() {
    let host_uri = "http://localhost";
    let port = 8080;
    let host_endpoint = HostEndpoint::new(host_uri, port);

    assert_eq!(host_endpoint.host_uri(), "http://localhost");
}

#[test]
fn test_port() {
    let host_uri = "http://localhost";
    let port = 8080;
    let host_endpoint = HostEndpoint::new(host_uri, port);

    assert_eq!(host_endpoint.port(), 8080);
}

#[test]
fn test_display() {
    let host_uri = "http://localhost";
    let port = 8080;
    let host_endpoint = HostEndpoint::new(host_uri, port);

    assert_eq!(
        host_endpoint.to_string(),
        "host_uri: http://localhost,  port: 8080"
    );
}
