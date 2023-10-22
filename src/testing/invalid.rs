use std::{fmt::Debug, str::FromStr};

use arbitrary::{Arbitrary, Unstructured};
use proptest::prelude::Strategy;

use crate::testing::NegateArbitrary;

#[derive(Debug, Clone)]
pub struct Invalid<T>(T);

impl<'a, T> Arbitrary<'a> for Invalid<T>
where
    T: NegateArbitrary<'a>,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self(T::negate_arbitrary(u)?))
    }
}

impl<'a, T> NegateArbitrary<'a> for Invalid<T>
where
    T: NegateArbitrary<'a> + Arbitrary<'a>,
{
    fn negate_arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self(T::arbitrary(u)?))
    }
}

impl<T: FromStr> FromStr for Invalid<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(T::from_str(s)?))
    }
}

impl<T: ToString> ToString for Invalid<T> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub fn invalid<T>() -> impl Strategy<Value = T>
where
    T: Debug + for<'a> NegateArbitrary<'a>,
{
    crate::testing::gen::<Invalid<T>>().prop_map(move |x| x.0)
}

#[cfg(all(test, feature = "testing", feature = "internet"))]
mod tests {
    use proptest::*;

    use crate::{internet::*, testing::*, Error, Kind, Sensitive};

    proptest! {
        #[test]
        fn generates_invalid_emails(a in invalid::<Email>()) {
            assert_eq!(
                a.to_string().parse::<Email>(),
                Err(Error::FailedParsing(Kind::Email, a.to_string()))
            );
        }

        #[test]
        fn generates_invalid_usernames(a in invalid::<Username>()) {
            assert_eq!(
                a.to_string().parse::<Username>(),
                Err(Error::FailedParsing(Kind::Username, a.to_string()))
            );
        }

        #[test]
        fn invalid_of_invalid_is_valid(a in invalid::<Invalid<Username>>()) {
            assert!(a.to_string().parse::<Username>().is_ok());
        }

        #[test]
        fn generates_invalid_composite_types(a in invalid::<Sensitive<Username>>()) {
            assert_eq!(
                a.to_string().parse::<Username>(),
                Err(Error::FailedParsing(Kind::Username, a.to_string()))
            );
        }
    }
}
