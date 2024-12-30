#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum MessageType {
    UnknownMessageType = 0x0_u16,
    ClientLogin = 0x65_u16,
    ClientLogout = 0x66_u16,
    StartData = 0xc9_u16,
    StopData = 0xca_u16,
    StopAllData = 0xcb_u16,
    DataBar = 0xcc_u16,
    TradeBar = 0xcd_u16,
    OrderCreate = 0x191_u16,
    OrderUpdate = 0x192_u16,
    OrderCancel = 0x193_u16,
    OrderCancelAll = 0x194_u16,
    ClientError = 0x321_u16,
    DataError = 0x322_u16,
    OrderError = 0x323_u16,
    #[default]
    NullVal = 0xffff_u16,
}
impl From<u16> for MessageType {
    #[inline]
    fn from(v: u16) -> Self {
        match v {
            0x0_u16 => Self::UnknownMessageType,
            0x65_u16 => Self::ClientLogin,
            0x66_u16 => Self::ClientLogout,
            0xc9_u16 => Self::StartData,
            0xca_u16 => Self::StopData,
            0xcb_u16 => Self::StopAllData,
            0xcc_u16 => Self::DataBar,
            0xcd_u16 => Self::TradeBar,
            0x191_u16 => Self::OrderCreate,
            0x192_u16 => Self::OrderUpdate,
            0x193_u16 => Self::OrderCancel,
            0x194_u16 => Self::OrderCancelAll,
            0x321_u16 => Self::ClientError,
            0x322_u16 => Self::DataError,
            0x323_u16 => Self::OrderError,
            _ => Self::NullVal,
        }
    }
}
impl From<MessageType> for u16 {
    #[inline]
    fn from(v: MessageType) -> Self {
        match v {
            MessageType::UnknownMessageType => 0x0_u16,
            MessageType::ClientLogin => 0x65_u16,
            MessageType::ClientLogout => 0x66_u16,
            MessageType::StartData => 0xc9_u16,
            MessageType::StopData => 0xca_u16,
            MessageType::StopAllData => 0xcb_u16,
            MessageType::DataBar => 0xcc_u16,
            MessageType::TradeBar => 0xcd_u16,
            MessageType::OrderCreate => 0x191_u16,
            MessageType::OrderUpdate => 0x192_u16,
            MessageType::OrderCancel => 0x193_u16,
            MessageType::OrderCancelAll => 0x194_u16,
            MessageType::ClientError => 0x321_u16,
            MessageType::DataError => 0x322_u16,
            MessageType::OrderError => 0x323_u16,
            MessageType::NullVal => 0xffff_u16,
        }
    }
}
impl core::str::FromStr for MessageType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "UnknownMessageType" => Ok(Self::UnknownMessageType),
            "ClientLogin" => Ok(Self::ClientLogin),
            "ClientLogout" => Ok(Self::ClientLogout),
            "StartData" => Ok(Self::StartData),
            "StopData" => Ok(Self::StopData),
            "StopAllData" => Ok(Self::StopAllData),
            "DataBar" => Ok(Self::DataBar),
            "TradeBar" => Ok(Self::TradeBar),
            "OrderCreate" => Ok(Self::OrderCreate),
            "OrderUpdate" => Ok(Self::OrderUpdate),
            "OrderCancel" => Ok(Self::OrderCancel),
            "OrderCancelAll" => Ok(Self::OrderCancelAll),
            "ClientError" => Ok(Self::ClientError),
            "DataError" => Ok(Self::DataError),
            "OrderError" => Ok(Self::OrderError),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for MessageType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownMessageType => write!(f, "UnknownMessageType"),
            Self::ClientLogin => write!(f, "ClientLogin"),
            Self::ClientLogout => write!(f, "ClientLogout"),
            Self::StartData => write!(f, "StartData"),
            Self::StopData => write!(f, "StopData"),
            Self::StopAllData => write!(f, "StopAllData"),
            Self::DataBar => write!(f, "DataBar"),
            Self::TradeBar => write!(f, "TradeBar"),
            Self::OrderCreate => write!(f, "OrderCreate"),
            Self::OrderUpdate => write!(f, "OrderUpdate"),
            Self::OrderCancel => write!(f, "OrderCancel"),
            Self::OrderCancelAll => write!(f, "OrderCancelAll"),
            Self::ClientError => write!(f, "ClientError"),
            Self::DataError => write!(f, "DataError"),
            Self::OrderError => write!(f, "OrderError"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
