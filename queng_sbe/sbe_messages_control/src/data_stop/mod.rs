use common_exchange::ExchangeID;
use sbe_types::{DataType, MessageType};

mod display;
mod getter;
mod sbe_decode;
mod sbe_encode;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct StopDataMessage {
    message_type: MessageType,
    client_id: u16,
    exchange_id: ExchangeID,
    symbol_id: String,
    data_type_id: DataType,
}

impl StopDataMessage {
    /// Creates a new `StopDataMessage` instance.
    ///
    /// Sets the `message_type` to `StopData`.
    ///
    /// # Arguments
    ///
    /// * `client_id` - u16 client ID
    /// * `exchange_id` - `ExchangeID` exchange ID
    /// * `symbol_id` - u16 symbol ID
    /// * `data_type_id` - `DataType` data type ID
    ///
    /// # Returns
    ///
    /// `StopDataMessage` instance
    ///
    #[must_use]
    pub const fn new(
        client_id: u16,
        exchange_id: ExchangeID,
        symbol_id: String,
        data_type_id: DataType,
    ) -> Self {
        let message_type = MessageType::StopData;
        Self {
            message_type,
            client_id,
            exchange_id,
            symbol_id,
            data_type_id,
        }
    }
}

impl From<&[u8]> for StopDataMessage {
    /// Implements the From trait to decode a `StopDataMessage` from a byte slice.
    ///
    /// Calls the `sbe_decode::decode_stop_data_message` function to decode the message.
    ///
    /// # Arguments
    ///
    /// * `buffer` - Byte slice to decode
    ///
    /// # Returns
    ///
    /// Decoded `StopDataMessage`
    ///
    /// # Errors
    ///
    /// Panics if decode fails
    ///
    #[inline]
    fn from(buffer: &[u8]) -> Self {
        sbe_decode::decode_stop_data_message(buffer).expect("Failed to decode start data message")
    }
}
