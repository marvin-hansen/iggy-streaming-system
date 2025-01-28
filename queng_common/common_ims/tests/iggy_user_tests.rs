use common_ims::IggyUser;

#[test]
fn test_new_iggy_user() {
    let user = IggyUser::new("test_user", "test_pass");
    assert_eq!(user.username(), "test_user");
    assert_eq!(user.password(), "test_pass");
}

#[test]
fn test_default_iggy_user() {
    let user = IggyUser::default();
    assert_eq!(user.username(), "iggy");
    assert_eq!(user.password(), "iggy");
}

#[test]
fn test_display_iggy_user() {
    let user = IggyUser::new("test_user", "test_pass");
    assert_eq!(format!("{}", user), "User { username: test_user }");
}

#[test]
fn test_clone_iggy_user() {
    let user = IggyUser::new("test_user", "test_pass");
    let cloned_user = user.clone();
    assert_eq!(user, cloned_user);
}

#[test]
fn test_partial_eq_iggy_user() {
    let user1 = IggyUser::new("test_user", "test_pass");
    let user2 = IggyUser::new("test_user", "test_pass");
    let user3 = IggyUser::new("different_user", "test_pass");

    assert_eq!(user1, user2);
    assert_ne!(user1, user3);
}
