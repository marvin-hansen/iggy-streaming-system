use crate::StartDataMessage;
use sbe_bindings::message_type::MessageType as SbeMessageType;
use sbe_bindings::{
    message_header_codec, start_data_msg_codec::StartDataMsgEncoder, Encoder, WriteBuf,
};
use sbe_types::SbeEncodeError;

impl StartDataMessage {
    /// Encodes a `StartDataMessage` to a byte buffer.
    ///
    /// # Arguments
    ///
    /// * `self` - `StartDataMessage` to encode
    ///
    /// # Returns
    ///
    /// (usize, `Vec<u8>`) - Tuple containing encoded size and byte buffer
    ///
    /// # Errors
    ///
    /// Returns Err if encoding fails
    ///
    /// # Process
    ///
    /// - Create 35 byte buffer
    /// - Create default `StartDataMsgEncoder`
    /// - Wrap buffer in `WriteBuf`
    /// - Encode header
    /// - Encode `message_type`
    /// - Encode `client_id`
    /// - Encode `exchange_id`
    /// - Encode `symbol_id`
    /// - Encode `time_resolution`
    /// - Encode `data_type_id`
    /// - Return encoded size and buffer
    ///
    pub fn encode(&self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        let mut buffer = vec![0u8; 35];

        let mut csg = StartDataMsgEncoder::default();

        csg = csg.wrap(
            WriteBuf::new(buffer.as_mut_slice()),
            message_header_codec::ENCODED_LENGTH,
        );

        csg = csg.header(0).parent().expect("Failed to encode header");

        let value = SbeMessageType::from(self.message_type as u16);
        csg.message_type(value);

        csg.client_id(self.client_id);

        csg.exchange_id(self.exchange_id as u8);

        // Convert string symbol id into fixed sized char [u8; 20]
        let mut byte_array = [0u8; 20];
        byte_array[..self.symbol_id().len()].copy_from_slice(self.symbol_id().as_bytes());
        csg.symbol_id(&byte_array);

        csg.time_resolution(self.time_resolution as u8);

        csg.data_type_id(self.data_type_id as u8);

        let limit = csg.get_limit();
        Ok((limit, buffer))
    }
}
