use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum SecurityType {
    UnknownSecurityType,
    #[default]
    Spot,
    Index,
    Future,
    PerpetualFuture,
    Option,
    FutureOption,
}

impl Display for SecurityType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownSecurityType => write!(f, "UnknownSecurityType"),
            Self::Spot => write!(f, "Spot"),
            Self::Index => write!(f, "Index"),
            Self::Future => write!(f, "Future"),
            Self::PerpetualFuture => write!(f, "PerpetualFuture"),
            Self::Option => write!(f, "Option"),
            Self::FutureOption => write!(f, "FutureOption"),
        }
    }
}
