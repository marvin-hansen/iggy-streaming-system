use std::fmt;

/// The `ClientChannel` enum represents the different channels a client can use.
///
/// This is represented as a u8 under the hood and includes the following variants:
///
/// - `DataChannel` - The channel for sending data payloads. This has the underlying value 0.
///
/// - `ControlChannel` - The channel for sending control messages. This has the underlying value 1.
///
/// - `ExecutionChannel` - The channel for sending execution messages. This has the underlying value 2.
///
/// - `HeartbeatChannel` - The channel for sending heartbeat messages. This has the underlying value 3.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum MessageClientChannel {
    DataChannel = 0,
    ControlChannel = 1,
    ErrorChannel = 2,
    ExecutionChannel = 3,
    HeartbeatChannel = 4,
}

impl From<u8> for MessageClientChannel {
    /// Implements the From trait to convert a u8 to a `ClientChannel`.
    ///
    /// Matches on the u8 value:
    ///
    /// 0 -> `DataChannel`
    /// 1 -> `ControlChannel`
    /// 2 -> `ErrorChannel`
    /// 3 -> `ExecutionChannel`
    /// 4 -> `HeartbeatChannel`
    ///
    /// Panics on unknown value.
    ///
    /// # Arguments
    ///
    /// * `value` - u8 value to convert
    ///
    /// # Returns
    ///
    /// `ClientChannel` variant
    ///
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::DataChannel,
            1 => Self::ControlChannel,
            2 => Self::ErrorChannel,
            3 => Self::ExecutionChannel,
            4 => Self::HeartbeatChannel,
            _ => panic!("Unknown ClientChannel value: {value}"),
        }
    }
}

impl fmt::Display for MessageClientChannel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::DataChannel => write!(f, "DataChannel"),
            Self::ControlChannel => write!(f, "ControlChannel"),
            Self::ErrorChannel => write!(f, "ErrorChannel"),
            Self::ExecutionChannel => write!(f, "ExecutionChannel"),
            Self::HeartbeatChannel => write!(f, "HeartbeatChannel"),
        }
    }
}
