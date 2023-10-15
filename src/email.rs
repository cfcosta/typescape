use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use serde_email::is_valid_email;

use crate::{Error, Kind};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
pub struct Email(String);

impl FromStr for Email {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_valid_email(s) {
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

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for Email {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        use fake::{faker::internet::en::SafeEmail, Fake};
        use rand::{rngs::StdRng, SeedableRng};

        let mut rng = StdRng::from_seed(u.arbitrary()?);
        let domain = SafeEmail().fake_with_rng(&mut rng);
        Ok(Self(domain))
    }
}

#[cfg(all(test, feature = "arbitrary"))]
mod tests {
    use proptest::prelude::*;
    use proptest_arbitrary_interop::arb;

    use crate::prelude::*;

    proptest! {
        #[test]
        fn arbitrary_email_is_always_valid(a in arb::<Email>()) {
            a.to_string().parse::<Email>().expect("Failed parsing");
        }

        #[test]
        fn invalid_emails_are_always_invalid(a in arb::<Invalid<Email>>()) {
            assert_eq!(
                a.to_string().parse::<Email>(),
                Err(Error::FailedParsing(crate::Kind::Email, a.to_string()))
            );
        }
    }
}
