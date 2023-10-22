use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[cfg(feature = "testing")]
use fake::{faker::internet::en::SafeEmail, Fake};

#[cfg(feature = "testing")]
use proptest::prelude::*;

use crate::{
    testing::{NegateArbitrary, Rng},
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
impl Arbitrary for Email {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        any::<Rng>()
            .prop_map(|mut rng| SafeEmail().fake_with_rng(&mut rng.0))
            .prop_map(Self)
            .boxed()
    }
}

impl NegateArbitrary for Email {
    fn negate_arbitrary() -> <Self as Arbitrary>::Strategy {
        "[a-zA-Z0-9._%+-]+[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"
            .prop_map(Self)
            .boxed()
    }
}

#[cfg(all(test, feature = "testing"))]
mod tests {
    use proptest::prelude::*;

    use crate::testing::*;

    use super::*;

    proptest! {
        #[test]
        fn arbitrary_email_is_always_valid(a in any::<Email>()) {
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
