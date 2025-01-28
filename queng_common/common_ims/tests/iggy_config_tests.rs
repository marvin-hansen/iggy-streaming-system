use common_ims::{IggyConfig, IggyUser};
use iggy::identifier::Identifier;

fn create_test_user() -> IggyUser {
    IggyUser::new("test_user", "test_pass")
}

#[test]
fn test_new_iggy_config() {
    let user = create_test_user();
    let config = IggyConfig::new(user.clone(), "localhost:8090", 1, 2, 3, 100, true);

    assert_eq!(config.stream_id(), Identifier::numeric(1).unwrap());
    assert_eq!(config.stream_name(), "stream_1");
    assert_eq!(config.topic_id(), Identifier::numeric(2).unwrap());
    assert_eq!(config.topic_name(), "topic_2");
    assert_eq!(config.tcp_server_addr(), "localhost:8090");
    assert_eq!(config.partition_id(), 3);
    assert_eq!(config.messages_per_batch(), 100);
    assert!(config.auto_commit());
    assert_eq!(config.user(), &user);
}

#[test]
fn test_from_client_id() {
    let user = create_test_user();
    let config = IggyConfig::from_client_id(user.clone(), 100);

    assert_eq!(config.stream_id(), Identifier::numeric(100).unwrap());
    assert_eq!(config.stream_name(), "stream_100");
    assert_eq!(config.topic_id(), Identifier::numeric(100).unwrap());
    assert_eq!(config.topic_name(), "topic_100");
    assert_eq!(config.tcp_server_addr(), "127.0.0.1:8090");
    assert_eq!(config.partition_id(), 100);
    assert_eq!(config.messages_per_batch(), 10);
    assert!(config.auto_commit());
    assert_eq!(config.user(), &user);
}

#[test]
fn test_clone_iggy_config() {
    let user = create_test_user();
    let config = IggyConfig::new(user, "localhost:8090", 1, 2, 3, 100, true);
    let cloned_config = config.clone();
    assert_eq!(config, cloned_config);
}

#[test]
fn test_display_iggy_config() {
    let user = create_test_user();
    let config = IggyConfig::new(user, "localhost:8090", 1, 2, 3, 100, true);
    let display_str = format!("{}", config);
    assert!(display_str.contains("test_user"));
    assert!(display_str.contains("localhost:8090"));
    assert!(display_str.contains("stream_1"));
    assert!(display_str.contains("topic_2"));
}
