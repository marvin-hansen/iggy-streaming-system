use common_config::{ServiceID, SvcEnvConfig};

fn get_svc_env_config(service_id: ServiceID) -> SvcEnvConfig {
    let cluster_host = "127.0.0.1".to_string();
    let ci_host = "127.0.0.1".to_string();
    let local_host = "127.0.0.1".to_string();
    let docker_host = "0.0.0.0".to_string();
    let service_port = "7070".to_string();

    let metrics_host = "127.0.0.1".to_string();
    let metrics_uri = "metrics".to_string();
    let metrics_port = 8080;

    SvcEnvConfig::new(
        service_id,
        cluster_host,
        ci_host,
        local_host,
        docker_host,
        service_port,
        metrics_host,
        metrics_uri,
        metrics_port,
    )
}

#[test]
fn test_svc_env_config_new() {
    let config = get_svc_env_config(ServiceID::CMDB);

    assert_eq!(config.service_id(), ServiceID::CMDB);
}

#[test]
fn test_svc_env_config_cluster_host() {
    let config = get_svc_env_config(ServiceID::CMDB);

    assert_eq!(config.service_id(), ServiceID::CMDB);
    assert_eq!(config.cluster_host(), "127.0.0.1".to_string());
}

#[test]
fn test_svc_env_config_ci_host() {
    let config = get_svc_env_config(ServiceID::CMDB);

    assert_eq!(config.service_id(), ServiceID::CMDB);
    assert_eq!(config.cluster_host(), "127.0.0.1".to_string());
    assert_eq!(config.ci_host(), "127.0.0.1".to_string());
}

#[test]
fn test_svc_env_config_local_host() {
    let config = get_svc_env_config(ServiceID::CMDB);

    assert_eq!(config.service_id(), ServiceID::CMDB);
    assert_eq!(config.cluster_host(), "127.0.0.1".to_string());
    assert_eq!(config.ci_host(), "127.0.0.1".to_string());
    assert_eq!(config.local_host(), "127.0.0.1".to_string());
}

#[test]
fn test_svc_env_config_docker_host() {
    let config = get_svc_env_config(ServiceID::CMDB);

    assert_eq!(config.service_id(), ServiceID::CMDB);
    assert_eq!(config.cluster_host(), "127.0.0.1".to_string());
    assert_eq!(config.ci_host(), "127.0.0.1".to_string());
    assert_eq!(config.local_host(), "127.0.0.1".to_string());
    assert_eq!(config.docker_host(), "0.0.0.0".to_string());
}

#[test]
fn test_svc_env_service_port() {
    let config = get_svc_env_config(ServiceID::CMDB);

    assert_eq!(config.service_id(), ServiceID::CMDB);
    assert_eq!(config.cluster_host(), "127.0.0.1".to_string());
    assert_eq!(config.ci_host(), "127.0.0.1".to_string());
    assert_eq!(config.local_host(), "127.0.0.1".to_string());
    assert_eq!(config.service_port(), "7070".to_string());
}

#[test]
fn test_svc_env_metrics_host() {
    let config = get_svc_env_config(ServiceID::CMDB);

    assert_eq!(config.service_id(), ServiceID::CMDB);
    assert_eq!(config.cluster_host(), "127.0.0.1".to_string());
    assert_eq!(config.ci_host(), "127.0.0.1".to_string());
    assert_eq!(config.local_host(), "127.0.0.1".to_string());
    assert_eq!(config.service_port(), "7070".to_string());
    assert_eq!(config.metrics_host(), "127.0.0.1".to_string());
}

#[test]
fn test_svc_env_metrics_uri() {
    let config = get_svc_env_config(ServiceID::CMDB);

    assert_eq!(config.service_id(), ServiceID::CMDB);
    assert_eq!(config.cluster_host(), "127.0.0.1".to_string());
    assert_eq!(config.ci_host(), "127.0.0.1".to_string());
    assert_eq!(config.local_host(), "127.0.0.1".to_string());
    assert_eq!(config.service_port(), "7070".to_string());
    assert_eq!(config.metrics_host(), "127.0.0.1".to_string());
    assert_eq!(config.metrics_uri(), "metrics".to_string());
}

#[test]
fn test_svc_env_metrics_port() {
    let config = get_svc_env_config(ServiceID::CMDB);

    assert_eq!(config.service_id(), ServiceID::CMDB);
    assert_eq!(config.cluster_host(), "127.0.0.1".to_string());
    assert_eq!(config.ci_host(), "127.0.0.1".to_string());
    assert_eq!(config.local_host(), "127.0.0.1".to_string());
    assert_eq!(config.service_port(), "7070".to_string());
    assert_eq!(config.metrics_host(), "127.0.0.1".to_string());
    assert_eq!(config.metrics_uri(), "metrics".to_string());
    assert_eq!(config.metrics_port(), &8080);
}

#[test]
fn test_svc_env_config_display() {
    let config = get_svc_env_config(ServiceID::CMDB);

    let actual = config.to_string();
    let expected = "SvcEnvConfig { service_id: CMDB, cluster_host: \"127.0.0.1\", ci_host: \"127.0.0.1\", local_host: \"127.0.0.1\", docker_host: \"0.0.0.0\", service_port: \"7070\", metrics_host: \"127.0.0.1\", metrics_uri: \"metrics\", metrics_port: 8080 }";
    assert_eq!(actual, expected);
}
