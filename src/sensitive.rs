use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;

#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;

#[cfg(any(test, feature = "testing"))]
use crate::testing::NegateArbitrary;

const MASK: &str = "******";

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
/// A container for sensitive data, such as passwords or credentials, blocking them from being
/// printed.
pub struct Sensitive<T>(T);

impl<T> Sensitive<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn get(self) -> T {
        self.0
    }
}

impl<T> Display for Sensitive<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", MASK)
    }
}

impl<T> Debug for Sensitive<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Sensitive").field(&MASK).finish()
    }
}

impl<T> From<T> for Sensitive<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T: FromStr> FromStr for Sensitive<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match T::from_str(s) {
            Ok(t) => Ok(Sensitive(t)),
            Err(e) => Err(e),
        }
    }
}

impl<T: PartialEq> PartialEq<Self> for Sensitive<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Eq for Sensitive<T> where T: Eq {}

impl<T: PartialOrd> PartialOrd for Sensitive<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Ord> Ord for Sensitive<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Hash> Hash for Sensitive<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[cfg(any(test, feature = "testing"))]
impl<T: Arbitrary + 'static> Arbitrary for Sensitive<T> {
    type Parameters = T::Parameters;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        T::arbitrary_with(args).prop_map(Self).boxed()
    }
}

#[cfg(any(test, feature = "testing"))]
impl<T: NegateArbitrary + 'static> NegateArbitrary for Sensitive<T> {
    fn negate_arbitrary() -> <Self as Arbitrary>::Strategy {
        T::negate_arbitrary().prop_map(Self).boxed()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn hides_internal_representation(s in any::<Sensitive<String>>()) {
            assert_eq!(s.to_string(), MASK);
            assert_eq!(format!("{}", s), MASK);
            assert_eq!(
                format!("{:?}", s),
                format!("Sensitive(\"{}\")", MASK)
            );
        }

        #[test]
        fn preserves_equality(a in any::<String>(), b in any::<String>()) {
            prop_assert_eq!(
                Sensitive::new(a.clone()) == Sensitive::new(b.clone()),
                a == b
            );
        }

        #[test]
        fn preserves_order(a in any::<usize>(), b in any::<usize>()) {
            prop_assert_eq!(Sensitive::new(a) == Sensitive::new(b), a == b);
            prop_assert_eq!(Sensitive::new(a) >= Sensitive::new(b), a >= b);
            prop_assert_eq!(Sensitive::new(a) <= Sensitive::new(b), a <= b);
            prop_assert_eq!(Sensitive::new(a) > Sensitive::new(b), a > b);
            prop_assert_eq!(Sensitive::new(a) < Sensitive::new(b), a < b);
        }

        #[test]
        fn preserves_into(a in any::<usize>()) {
            assert_eq!(Sensitive::from(a), Sensitive::new(a));
        }
    }
}
