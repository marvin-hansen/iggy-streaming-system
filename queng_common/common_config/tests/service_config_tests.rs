use common_config::{Endpoint, ServiceConfig, ServiceID};

#[test]
fn test_new() {
    let id = ServiceID::SMDB;
    let name = "name".to_string();
    let version = 1;
    let online = true;
    let description = "description".to_string();
    let health_check_uri = "health_check_uri".to_string();
    let base_uri = "base_uri".to_string();
    let dependencies = vec![ServiceID::DBGW];
    let endpoints = Vec::from([Endpoint::default(), Endpoint::default()]);

    let service_config = ServiceConfig::new(
        id,
        name,
        version,
        online,
        description,
        health_check_uri,
        base_uri,
        dependencies,
        endpoints,
    );

    assert_eq!(service_config.svc_id(), &ServiceID::SMDB);
    assert_eq!(service_config.name(), String::from("name"));
    assert_eq!(service_config.version(), 1);
    assert!(service_config.online());
    assert_eq!(service_config.description(), String::from("description"));
    assert_eq!(
        service_config.health_check_uri(),
        String::from("health_check_uri")
    );
    assert_eq!(service_config.cluster_uri(), String::from("base_uri"));
    assert_eq!(
        service_config.dependencies().len(),
        vec![ServiceID::DBGW].len()
    );
    assert_eq!(service_config.service_endpoint(), Endpoint::default());
}

#[test]
fn test_default() {
    let service_config = ServiceConfig::default();

    assert_eq!(service_config.svc_id(), &ServiceID::Default);
    assert_eq!(service_config.name(), &String::new());
    assert_eq!(service_config.version(), 0);
    assert!(!service_config.online());
    assert_eq!(service_config.description(), &String::new());
    assert_eq!(service_config.health_check_uri(), &String::new());
    assert_eq!(service_config.cluster_uri(), &String::new());
    assert_eq!(service_config.dependencies(), &Vec::new());
}
