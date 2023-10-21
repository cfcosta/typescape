mod id;
mod sensitive;
mod text;

#[cfg(feature = "testing")]
pub mod testing;

#[cfg(feature = "internet")]
pub mod internet;

#[cfg(feature = "finances")]
pub mod finances;

#[cfg(feature = "finances")]
pub use rust_decimal::{Decimal, FromPrimitive};

pub use id::Id;
pub use sensitive::Sensitive;
pub use text::Text;

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// An enum representing all possible types of resources on this crate
pub enum Kind {
    Email,
    Username,
    Id,
    Text,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Email => write!(f, "email"),
            Kind::Username => write!(f, "username"),
            Kind::Id => write!(f, "id"),
            Kind::Text => write!(f, "text"),
        }
    }
}

#[derive(Debug, Clone, Error, PartialEq, Eq)]
/// All errors that might happening when using the resources
pub enum Error {
    #[error("Failed to parse `{0}` resource: {1}")]
    FailedParsing(Kind, String),
}

#[cfg(feature = "finances")]
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum FinanceError<T: Asset> {
    #[error("Mismatched currency, expected `{expected}`, got `{got}`")]
    MismatchedExchange {
        expected: T::Exchange,
        got: T::Exchange,
    },
    #[error("Non-positive amounts are not allowed")]
    NegativeAmount,
}
