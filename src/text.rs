use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::prelude::*;

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

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for Text {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        use rand::SeedableRng;

        let rng = rand::rngs::StdRng::from_seed(u.arbitrary()?);

        Ok(Self(lipsum::lipsum_with_rng(
            rng,
            u8::arbitrary(u)? as usize,
        )))
    }
}

#[cfg(all(test, feature = "arbitrary"))]
mod tests {
    use proptest::prelude::*;
    use proptest_arbitrary_interop::arb;

    use crate::prelude::*;

    proptest! {
        #[test]
        fn arbitrary_text_is_always_valid(a in arb::<Text>()) {
            a.to_string().parse::<Text>().expect("Failed parsing");
        }
    }
}
