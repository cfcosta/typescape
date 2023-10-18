mod id;
mod sensitive;
mod text;

#[cfg(feature = "testing")]
pub mod testing;

#[cfg(feature = "internet")]
mod internet;

pub mod prelude {
    pub use super::{id::*, sensitive::*, text::*};

    #[cfg(feature = "internet")]
    pub use super::internet::*;

    #[cfg(feature = "internet")]
    pub use super::testing;

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
}
