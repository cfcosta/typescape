mod email;
mod id;
mod invalid;
mod sensitive;
mod text;
mod username;

pub mod prelude {
    pub use super::{email::*, id::*, invalid::*, sensitive::*, text::*, username::*};

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
