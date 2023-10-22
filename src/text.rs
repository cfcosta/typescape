use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[cfg(any(test, feature = "testing"))]
use proptest::{
    prelude::*,
    strategy::{BoxedStrategy, Strategy},
};

#[cfg(any(test, feature = "testing"))]
use crate::testing::Rng;

use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
/// A piece of UTF-8 valid text
pub struct Text(String);

impl FromStr for Text {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Deref for Text {
    type Target = <String as Deref>::Target;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(any(test, feature = "testing"))]
impl Arbitrary for Text {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        any::<(Rng, u8)>()
            .prop_map(|(rng, size)| lipsum::lipsum_with_rng(rng.0, size as usize))
            .prop_map(Self)
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn arbitrary_text_is_always_valid(a in any::<Text>()) {
            a.to_string().parse::<Text>().expect("Failed parsing");
        }
    }
}
