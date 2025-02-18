use sbe_messages_client::ClientLoginMessage;
use sbe_types::MessageType;

const fn get_client_login_message(client_id: u16) -> ClientLoginMessage {
    ClientLoginMessage::new(client_id)
}

#[test]
fn test_new() {
    let client_id = 100;
    let message = get_client_login_message(client_id);

    assert_eq!(message.message_type(), &MessageType::ClientLogin);
    assert_eq!(message.client_id(), client_id);
}

#[test]
fn test_encode() {
    let client_id = 42;
    let message = get_client_login_message(client_id);

    assert_eq!(message.message_type(), &MessageType::ClientLogin);
    assert_eq!(message.client_id(), client_id);

    let enc = message.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 12);

    let expected: Vec<u8> = vec![4, 0, 101, 0, 1, 0, 1, 0, 101, 0, 42, 0];
    let actual = buffer;
    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let encoded: Vec<u8> = vec![4, 0, 101, 0, 1, 0, 1, 0, 101, 0, 23, 0];
    let buffer = encoded.as_slice();

    let message = ClientLoginMessage::from(buffer);
    assert_eq!(message.message_type(), &MessageType::ClientLogin);
    assert_eq!(message.client_id(), 23);
}

#[test]
fn test_message_type() {
    let client_id = 100;
    let message = get_client_login_message(client_id);

    assert_eq!(message.message_type(), &MessageType::ClientLogin);
}

#[test]
fn test_message_client_id() {
    let client_id = 100;
    let message = get_client_login_message(client_id);

    assert_eq!(message.client_id(), client_id);
}

#[test]
fn test_display() {
    let client_id = 100;

    let actual = ClientLoginMessage::new(client_id);
    let expected = "ClientLoginMessage { client_id: 100 }";

    assert_eq!(format!("{actual}"), expected);
}
