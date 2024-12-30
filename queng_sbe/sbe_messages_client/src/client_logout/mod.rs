use sbe_types::MessageType;

mod display;
mod getters;
mod sbe_decode;
mod sbe_encode;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ClientLogoutMessage {
    message_type: MessageType,
    client_id: u16,
}

impl ClientLogoutMessage {
    /// Creates a new `ClientLogoutMessage` instance.
    ///
    /// Sets the `message_type` to `ClientLogout`.
    ///
    /// # Arguments
    ///
    /// * `client_id` - u16 client ID
    ///
    /// # Returns
    ///
    /// `ClientLogoutMessage` instance
    ///
    #[must_use]
    pub const fn new(client_id: u16) -> Self {
        let message_type = MessageType::ClientLogout;
        Self {
            message_type,
            client_id,
        }
    }
}

impl From<&[u8]> for ClientLogoutMessage {
    /// Implements the From trait to decode a `ClientLogoutMessage` from a byte slice.
    ///
    /// Calls the `sbe_decode::decode_client_logout_message` function to decode the message.
    ///
    /// # Arguments
    ///
    /// * `value` - Byte slice to decode
    ///
    /// # Returns
    ///
    /// Decoded `ClientLogoutMessage`
    ///
    /// # Errors
    ///
    /// Panics if decode fails
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_client_logout_message(value)
            .expect("Failed to decode ClientLoginMessage")
    }
}
