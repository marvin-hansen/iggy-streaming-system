use common_data_bar::TimeResolution;
use common_exchange::ExchangeID;
use sbe_types::{DataType, MessageType};

mod display;
mod getter;
mod sbe_decode;
mod sbe_encode;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct StartDataMessage {
    message_type: MessageType,
    client_id: u16,
    exchange_id: ExchangeID,
    symbol_id: String,
    time_resolution: TimeResolution,
    data_type_id: DataType,
}

impl StartDataMessage {
    /// Creates a new `StartDataMessage` instance.
    ///
    /// Sets the `message_type` to `StartData`.
    ///
    /// # Arguments
    ///
    /// * `client_id` - u16 client ID
    /// * `exchange_id` - `ExchangeID` exchange ID
    /// * `symbol_id` - u16 symbol ID
    /// * `time_resolution` - `TimeResolution` time resolution
    /// * `data_type_id` - `DataType` data type ID
    ///
    /// # Returns
    ///
    /// `StartDataMessage` instance
    ///
    #[must_use]
    pub const fn new(
        client_id: u16,
        exchange_id: ExchangeID,
        symbol_id: String,
        time_resolution: TimeResolution,
        data_type_id: DataType,
    ) -> Self {
        let message_type = MessageType::StartData;

        Self {
            message_type,
            client_id,
            exchange_id,
            symbol_id,
            time_resolution,
            data_type_id,
        }
    }
}

impl From<&[u8]> for StartDataMessage {
    /// Implements the From trait to decode a `StartDataMessage` from a byte slice.
    ///
    /// Calls the `sbe_decode::decode_start_data_message` function to decode the message.
    ///
    /// # Arguments
    ///
    /// * `value` - Byte slice to decode
    ///
    /// # Returns
    ///
    /// Decoded `StartDataMessage`
    ///
    /// # Errors
    ///
    /// Panics if decode fails
    #[inline]
    fn from(value: &[u8]) -> Self {
        sbe_decode::decode_start_data_message(value).expect("Failed to decode start data message")
    }
}
