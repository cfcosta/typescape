use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[cfg(feature = "testing")]
use fake::{faker::internet::en as f, Fake};

#[cfg(feature = "testing")]
use crate::testing::{NegateArbitrary, Rng};

#[cfg(feature = "testing")]
use proptest::prelude::*;

use crate::{Error, Kind};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
/// A handle for an user, with only alphanumeric characters and underscores
pub struct Username(String);

impl FromStr for Username {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use chumsky::{error::Cheap, prelude::*};

        let result = filter::<_, _, Cheap<char>>(|&c: &char| c.is_ascii_alphanumeric())
            .map(Some)
            .chain::<char, Vec<_>, _>(
                filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_').repeated(),
            )
            .then_ignore(end())
            .collect()
            .parse(s)
            .map_err(|_| Error::FailedParsing(Kind::Username, s.to_string()))?;

        Ok(Self(result))
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Deref for Username {
    type Target = <String as Deref>::Target;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Username {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "testing")]
impl Arbitrary for Username {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        any::<Rng>()
            .prop_map(|mut rng| f::Username().fake_with_rng(&mut rng.0))
            .prop_map(|u: String| u.replace('.', "_"))
            .prop_map(Self)
            .boxed()
    }
}

impl NegateArbitrary for Username {
    fn negate_arbitrary() -> <Self as Arbitrary>::Strategy {
        "[^a-zA-Z0-9].*|.*[^a-zA-Z0-9_].*".prop_map(Self).boxed()
    }
}

#[cfg(all(test, feature = "testing"))]
mod tests {
    use proptest::prelude::*;

    use crate::testing::*;

    use super::*;

    proptest! {
        #[test]
        fn parses_correctly(u in "[a-zA-Z0-9][a-zA-Z0-9_]*") {
            u.to_string().parse::<Username>().expect("Failed parsing");
        }

        #[test]
        fn rejects_all_invalid(u in "[^a-zA-Z0-9].*|.*[^a-zA-Z0-9_].*") {
            assert!(u.parse::<Username>().is_err())
        }

        #[test]
        fn arbitrary_email_is_always_valid(a in any::<Username>()) {
            a.to_string().parse::<Username>().expect("Failed parsing");
        }

        #[test]
        fn invalid_emails_are_always_invalid(a in invalid::<Username>()) {
            assert_eq!(
                a.to_string().parse::<Username>(),
                Err(Error::FailedParsing(Kind::Username, a.to_string()))
            );
        }
    }
}
