use std::fmt::Display;

use thiserror::Error;

mod email;
mod id;
mod invalid;
mod sensitive;
mod username;

pub mod prelude {
    pub use super::{email::*, id::*, invalid::*, sensitive::*, username::*, Error};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Email,
    Username,
    Id,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Email => write!(f, "email"),
            Kind::Username => write!(f, "username"),
            Kind::Id => write!(f, "id"),
        }
    }
}

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum Error {
    #[error("Failed to parse `{0}` resource: {1}")]
    FailedParsing(Kind, String),
}
