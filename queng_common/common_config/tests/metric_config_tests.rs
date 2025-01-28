use common_config::{Endpoint, MetricConfig, ProtocolType};

#[test]
fn test_new() {
    let config = MetricConfig::new("metrics".to_string(), "localhost".to_string(), 8080);

    assert_eq!(config.uri(), "metrics");
    assert_eq!(config.host(), "localhost");
    assert_eq!(config.port(), 8080);
}

#[test]
fn test_from_endpoint() {
    let endpoint = Endpoint::new(
        "name".to_string(),
        0,
        "/uri".to_string(),
        80,
        ProtocolType::HTTP,
    );
    let metric_config = MetricConfig::from_endpoint(&endpoint);
    assert_eq!(metric_config.uri(), "/uri");
    assert_eq!(metric_config.host(), "0.0.0.0");
    assert_eq!(metric_config.port(), 80);
}

#[test]
fn test_default() {
    let config = MetricConfig::default();

    assert_eq!(config.uri(), "metrics");
    assert_eq!(config.host(), "0.0.0.0");
    assert_eq!(config.port(), 8080);
}

#[test]
fn test_display() {
    let config = MetricConfig::default();

    let expected = format!(
        "metric_uri: {},  metric_host: {},  metric_port: {}",
        config.uri(),
        config.host(),
        config.port()
    );

    assert_eq!(format!("{config}"), expected);
}
