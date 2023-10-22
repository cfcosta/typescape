use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::{
    testing::{from_regex, NegateArbitrary},
    *,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
/// A validated e-mail address
pub struct Email(String);

impl FromStr for Email {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if serde_email::is_valid_email(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(Error::FailedParsing(Kind::Email, s.to_string()))
        }
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Deref for Email {
    type Target = <String as Deref>::Target;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Email {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "testing")]
impl<'a> arbitrary::Arbitrary<'a> for Email {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        use fake::{faker::internet::en::SafeEmail, Fake};
        use rand::{rngs::StdRng, SeedableRng};

        let mut rng = StdRng::from_seed(u.arbitrary()?);
        let domain = SafeEmail().fake_with_rng(&mut rng);
        Ok(Self(domain))
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        String::size_hint(depth)
    }
}

#[cfg(feature = "testing")]
impl<'a> NegateArbitrary<'a> for Email {
    fn negate_arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let gen = from_regex(u, "[a-zA-Z0-9._%+-]+[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}");

        Ok(Self(gen.get()))
    }
}

#[cfg(all(test, feature = "testing"))]
mod tests {
    use proptest::*;

    use crate::testing::*;

    use super::*;

    proptest! {
        #[test]
        fn arbitrary_email_is_always_valid(a in gen::<Email>()) {
            a.to_string().parse::<Email>().expect("Failed parsing");
        }

        #[test]
        fn invalid_emails_are_always_invalid(a in invalid::<Email>()) {
            assert_eq!(
                a.to_string().parse::<Email>(),
                Err(Error::FailedParsing(Kind::Email, a.to_string()))
            );
        }
    }
}
