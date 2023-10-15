use std::{fmt::Display, marker::PhantomData, str::FromStr};

use arbitrary::Arbitrary;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Invalid<T>(String, PhantomData<T>);

impl<T> Invalid<T> {
    pub fn get(&self) -> String {
        self.0.clone()
    }
}

impl<'a, T: FromStr> Arbitrary<'a> for Invalid<T> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        loop {
            let s = u.arbitrary::<String>()?;

            if s.parse::<T>().is_ok() {
                continue;
            }

            return Ok(Self(s, Default::default()));
        }
    }
}

impl<T> Display for Invalid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
