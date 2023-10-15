use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use arbitrary::Arbitrary;
use check_if_email_exists::syntax::check_syntax;
use fake::{faker::internet::en::SafeEmail, Fake};
use rand::{rngs::StdRng, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::{Error, Kind};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Email(String);

impl<'a> Arbitrary<'a> for Email {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut rng = StdRng::from_seed(u.arbitrary()?);
        let domain = SafeEmail().fake_with_rng(&mut rng);
        Ok(Self(domain))
    }
}

impl FromStr for Email {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if check_syntax(s).is_valid_syntax {
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

#[cfg(test)]
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
