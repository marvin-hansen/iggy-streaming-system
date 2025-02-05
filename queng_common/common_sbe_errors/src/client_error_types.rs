use std::fmt::{Display, Formatter};

/// `ClientErrorType` enum representing different client error types.
///
/// Can take following values:
///
/// - `UnknownClientError` = `0_u8`
/// - `ClientAlreadyLoggedIn` = `1_u8`
/// - `ClientLogInError` = `2_u8`
/// - `ClientNotLoggedIn` = `3_u8`
/// - `ClientLogOutError` = `4_u8`
/// - `ClientNotAuthorized` = `5_u8`
/// - `ClientShutdownError` = `6_u8`
///
///
/// # Remarks
///
/// Derives common Rust traits for convenience:
/// - Serialize, Deserialize - serialization
/// - Clone, Copy, Debug, Default - generics
/// - `PartialEq`, Eq - equality
/// - `PartialOrd`, Ord - ordering
/// - Hash - hashability
///
/// Represented as u8 for compactness.
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ClientErrorType {
    #[default]
    UnknownClientError = 0_u8,
    ClientAlreadyLoggedIn = 1_u8,
    ClientLogInError = 2_u8,
    ClientNotLoggedIn = 3_u8,
    ClientLogOutError = 4_u8,
    ClientNotAuthorized = 5_u8,
    ClientShutdownError = 6_u8,
}

impl From<ClientErrorType> for u8 {
    #[inline]
    fn from(value: ClientErrorType) -> Self {
        value as u8
    }
}

impl From<u8> for ClientErrorType {
    /// Implements `From<u8>` trait to convert u8 to `ClientErrorType`.
    ///
    /// # Arguments
    ///
    /// * `value` - u8 value to convert
    ///
    /// # Returns
    ///
    /// `ClientErrorType` variant corresponding to u8 value:
    ///
    /// - 0 -> `UnknownClientError`
    /// - 1 -> `ClientAlreadyLoggedIn`
    /// - 2 -> `ClientLogInError`
    /// - 3 -> `ClientNotLoggedIn`
    /// - 4 -> `ClientLogOutError`
    /// - 5 -> `ClientNotAuthorized`
    /// - 6 -> `ClientShutdownError`
    /// - Other -> `UnknownClientError`
    ///
    /// # Remarks
    ///
    /// Allows converting from raw u8 value to `ClientErrorType` enum.
    /// Useful when decoding from binary format.
    ///
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0_u8 => Self::UnknownClientError,
            1_u8 => Self::ClientAlreadyLoggedIn,
            2_u8 => Self::ClientLogInError,
            3_u8 => Self::ClientNotLoggedIn,
            4_u8 => Self::ClientLogOutError,
            5_u8 => Self::ClientNotAuthorized,
            6_u8 => Self::ClientShutdownError,
            _ => Self::UnknownClientError,
        }
    }
}

impl Display for ClientErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
