use std::fmt::Display;

use thiserror::Error;

mod email;
mod invalid;
mod sensitive;

pub mod prelude {
    pub use super::{email::*, invalid::*, sensitive::*, Error};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Email,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Email => write!(f, "email"),
        }
    }
}

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum Error {
    #[error("Failed to parse resource {0}: {1}")]
    FailedParsing(Kind, String),
}
