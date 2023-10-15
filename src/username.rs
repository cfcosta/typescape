use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Username(String);

impl FromStr for Username {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use chumsky::{error::Cheap, prelude::*};

        let result = filter::<_, _, Cheap<char>>(|&c: &char| c.is_ascii_alphabetic() || c == '_')
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

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for Username {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        use fake::{faker::internet::en::Username, Fake};
        use rand::{rngs::StdRng, SeedableRng};

        let mut rng = StdRng::from_seed(u.arbitrary()?);
        let user = Username()
            .fake_with_rng::<String, _>(&mut rng)
            .replace('.', "_");

        Ok(Self(user))
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::prelude::*;

    #[cfg(feature = "arbitrary")]
    use proptest_arbitrary_interop::arb;

    proptest! {
        #[test]
        fn parses_correctly(u in "[a-zA-Z_][a-zA-Z0-9_]*") {
            u.to_string().parse::<Username>().expect("Failed parsing");
        }

        #[test]
        fn rejects_all_invalid(u in "[^a-zA-Z_].*|.*[^a-zA-Z0-9_].*|.{26,}") {
            assert!(u.parse::<Username>().is_err())
        }

        #[cfg(feature = "arbitrary")]
        #[test]
        fn arbitrary_email_is_always_valid(a in arb::<Username>()) {
            a.to_string().parse::<Username>().expect("Failed parsing");
        }

        #[cfg(feature = "arbitrary")]
        #[test]
        fn invalid_emails_are_always_invalid(a in arb::<Invalid<Username>>()) {
            assert_eq!(
                a.to_string().parse::<Username>(),
                Err(Error::FailedParsing(Kind::Username, a.to_string()))
            );
        }
    }
}
