use iggy::identifier::Identifier;
use message_shared::{IggyConfig, IggyUser};

fn create_test_user() -> IggyUser {
    IggyUser::new("test_user", "test_pass")
}
fn create_test_iggy_config() -> IggyConfig {
    IggyConfig::new(
        create_test_user(),
        1,
        "stream_1".to_string(),
        1,
        2,
        "topic_2".to_string(),
        Some("localhost:8090".to_string()),
        None,
        3,
        "consumer_1".to_string(),
        100,
        true,
    )
}

#[test]
fn test_new_iggy_config() {
    let user = create_test_user();
    let config = create_test_iggy_config();

    assert_eq!(config.stream_id(), Identifier::numeric(1).unwrap());
    assert_eq!(config.stream_name(), "stream_1");
    assert_eq!(config.topic_id(), Identifier::numeric(2).unwrap());
    assert_eq!(config.topic_name(), "topic_2");
    assert!(config.tcp_server_addr().is_some());
    assert!(config.tcp_tls_config().is_none());
    assert_eq!(config.partition_id(), 3);
    assert_eq!(config.messages_per_batch(), 100);
    assert!(config.auto_commit());
    assert_eq!(config.user(), &user);
}

#[test]
fn test_from_client_id() {
    let user = create_test_user();
    let config = IggyConfig::from_client_id(&user, 100);

    assert_eq!(config.stream_id(), Identifier::numeric(100).unwrap());
    assert_eq!(config.stream_name(), "stream_100");
    assert_eq!(config.topic_id(), Identifier::numeric(100).unwrap());
    assert_eq!(config.topic_name(), "topic_100");
    assert!(config.tcp_server_addr().is_some());
    assert!(config.tcp_tls_config().is_none());
    assert_eq!(config.partition_id(), 100);
    assert_eq!(config.messages_per_batch(), 1);
    assert!(config.auto_commit());
    assert_eq!(config.user(), &user);
}

#[test]
fn test_clone_iggy_config() {
    let config = create_test_iggy_config();
    let cloned_config = config.clone();
    assert_eq!(config, cloned_config);
}

#[test]
fn test_display_iggy_config() {
    let config = create_test_iggy_config();
    let display_str = format!("{}", config);
    assert!(display_str.contains("test_user"));
    assert!(display_str.contains("localhost:8090"));
    assert!(display_str.contains("stream_1"));
    assert!(display_str.contains("topic_2"));
}
