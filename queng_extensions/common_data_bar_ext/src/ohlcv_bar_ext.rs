use common_data_bar::OHLCVBar;
use sbe_types::{SbeDecodeError, SbeEncodeError};

/// Provides an extension trait for `OHLCVBar` to encode and decode into an SBE message.
pub trait SbeOHLCVBarExtension {
    /// Encodes an `OHLCVBar` into an SBE message buffer.
    ///
    /// # Parameters
    ///
    /// - `bar` - The `OHLCVBar` to encode
    ///
    /// # Returns
    ///
    /// A Result containing:
    ///
    /// - The size of the encoded message
    /// - The encoded message buffer
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError>;

    /// Decodes an SBE message buffer into an `OHLCVBar`.
    ///
    /// # Parameters
    ///
    /// - `buffer` - The SBE encoded message buffer
    ///
    /// # Returns
    ///
    /// A Result containing the decoded `OHLCVBar` or a decoding error.
    fn decode_from_sbe(buffer: &[u8]) -> Result<OHLCVBar, SbeDecodeError>;
}

impl SbeOHLCVBarExtension for OHLCVBar {
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        sbe_messages_data::encode_data_bar_message(self)
    }
    fn decode_from_sbe(buffer: &[u8]) -> Result<OHLCVBar, SbeDecodeError> {
        sbe_messages_data::decode_data_bar_message(buffer)
    }
}
