use std::{fmt::Display, marker::PhantomData};

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

#[cfg(feature = "arbitrary")]
impl<'a, T: std::str::FromStr> arbitrary::Arbitrary<'a> for Invalid<T> {
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

#[cfg(all(test, feature = "arbitrary"))]
mod tests {
    use proptest::prelude::*;
    use proptest_arbitrary_interop::arb;

    use crate::prelude::*;

    proptest! {
        #[test]
        fn invalid_resources_are_always_invalid(a in arb::<Invalid<Email>>()) {
            assert_eq!(
                a.to_string().parse::<Email>(),
                Err(Error::FailedParsing(crate::Kind::Email, a.to_string()))
            );
        }
    }
}
