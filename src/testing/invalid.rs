use std::fmt::Display;

use crate::testing::NegateArbitrary;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
/// A container for testing types with invalid values
pub struct Invalid<T>(T);

impl<T: Display> Display for Invalid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl<'a, T> arbitrary::Arbitrary<'a> for Invalid<T>
where
    T: NegateArbitrary<'a> + arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self(T::negate_arbitrary(u)?))
    }
}

#[cfg(all(test, feature = "testing", feature = "internet"))]
mod tests {
    use proptest::*;

    use crate::{internet::*, testing::*, Error, Kind, Sensitive};

    proptest! {
        #[test]
        fn generates_invalid_emails(a in arb::<Invalid<Email>>()) {
            assert_eq!(
                a.to_string().parse::<Email>(),
                Err(Error::FailedParsing(Kind::Email, a.to_string()))
            );
        }

        #[test]
        fn generates_invalid_usernames(a in arb::<Invalid<Username>>()) {
            assert_eq!(
                a.to_string().parse::<Username>(),
                Err(Error::FailedParsing(Kind::Username, a.to_string()))
            );
        }

        #[test]
        fn generates_invalid_composite_types(a in arb::<Invalid<Sensitive<Username>>>()) {
            assert_eq!(
                a.to_string().parse::<Username>(),
                Err(Error::FailedParsing(Kind::Username, a.to_string()))
            );
        }
    }
}
