use message_shared::IggyTcpTLSConfig;

#[test]
fn test_tcp_tls_config_new() {
    let config = IggyTcpTLSConfig::new(
        "127.0.0.1:8090".to_string(),
        true,
        "example.com".to_string(),
        Some("ca.pem".to_string()),
    );

    assert_eq!(config.tcp_server_address(), "127.0.0.1:8090");
    assert!(config.tcp_tls_enabled());
    assert_eq!(config.tcp_tls_domain(), "example.com");
    assert_eq!(config.tcp_tls_ca_file(), &Some("ca.pem".to_string()));
}

#[test]
fn test_tcp_tls_config_default() {
    let config = IggyTcpTLSConfig::default();
    assert_eq!(config.tcp_server_address(), "127.0.0.1:8090");
    assert!(!config.tcp_tls_enabled());
    assert_eq!(config.tcp_tls_domain(), "localhost");
    assert_eq!(config.tcp_tls_ca_file(), &None);
}

#[test]
fn test_tcp_tls_config_display() {
    let config = IggyTcpTLSConfig::new(
        "127.0.0.1:8090".to_string(),
        true,
        "example.com".to_string(),
        Some("ca.pem".to_string()),
    );
    let display_str = format!("{}", config);
    assert!(display_str.contains("127.0.0.1:8090"));
    assert!(display_str.contains("true"));
    assert!(display_str.contains("example.com"));
}

#[test]
fn test_tcp_tls_config_without_ca_file() {
    let config = IggyTcpTLSConfig::new(
        "127.0.0.1:8090".to_string(),
        true,
        "example.com".to_string(),
        None,
    );
    assert_eq!(config.tcp_tls_ca_file(), &None);
}

#[test]
fn test_tcp_tls_config_equality() {
    let config1 = IggyTcpTLSConfig::new(
        "127.0.0.1:8090".to_string(),
        true,
        "example.com".to_string(),
        Some("ca.pem".to_string()),
    );
    let config2 = IggyTcpTLSConfig::new(
        "127.0.0.1:8090".to_string(),
        true,
        "example.com".to_string(),
        Some("ca.pem".to_string()),
    );
    assert_eq!(config1, config2);
}
