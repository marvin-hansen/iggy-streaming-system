use common_message::MessageClientChannel;

#[test]
fn test_valid_u8_to_channel_conversion() {
    assert_eq!(
        MessageClientChannel::from(0),
        MessageClientChannel::DataChannel
    );
    assert_eq!(
        MessageClientChannel::from(1),
        MessageClientChannel::ControlChannel
    );
    assert_eq!(
        MessageClientChannel::from(2),
        MessageClientChannel::ErrorChannel
    );
    assert_eq!(
        MessageClientChannel::from(3),
        MessageClientChannel::ExecutionChannel
    );
    assert_eq!(
        MessageClientChannel::from(4),
        MessageClientChannel::HeartbeatChannel
    );
}

#[test]
#[should_panic(expected = "Unknown ClientChannel value: 5")]
fn test_invalid_u8_should_panic() {
    let _ = MessageClientChannel::from(5);
}

#[test]
fn test_channel_display_formatting() {
    assert_eq!(MessageClientChannel::DataChannel.to_string(), "DataChannel");
    assert_eq!(
        MessageClientChannel::ControlChannel.to_string(),
        "ControlChannel"
    );
    assert_eq!(
        MessageClientChannel::ErrorChannel.to_string(),
        "ErrorChannel"
    );
    assert_eq!(
        MessageClientChannel::ExecutionChannel.to_string(),
        "ExecutionChannel"
    );
    assert_eq!(
        MessageClientChannel::HeartbeatChannel.to_string(),
        "HeartbeatChannel"
    );
}
