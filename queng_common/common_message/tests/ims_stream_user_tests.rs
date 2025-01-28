use common_message::StreamUser;

#[test]
fn test_message_user_new() {
    let user = StreamUser::new("test_user", "test_pass");
    assert_eq!(user.username(), "test_user");
    assert_eq!(user.password(), "test_pass");
}

#[test]
fn test_message_user_default() {
    let user = StreamUser::default();
    assert_eq!(user.username(), "default_user");
    assert_eq!(user.password(), "default_password");
}

#[test]
fn test_message_user_display() {
    let user = StreamUser::new("test_user", "test_pass");
    let display_str = format!("{}", user);
    assert!(display_str.contains("test_user"));
    // Password should not be included in the display string for security
    assert!(!display_str.contains("test_pass"));
}

#[test]
fn test_message_user_clone() {
    let user = StreamUser::new("test_user", "test_pass");
    let cloned_user = user.clone();
    assert_eq!(user.username(), cloned_user.username());
    assert_eq!(user.password(), cloned_user.password());
}

#[test]
fn test_message_user_debug() {
    let user = StreamUser::new("test_user", "test_pass");
    let debug_str = format!("{:?}", user);
    assert!(debug_str.contains("test_user"));
    assert!(debug_str.contains("test_pass"));
}
