use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;

use arbitrary::{Arbitrary, Unstructured};

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

#[cfg(feature = "testing")]
impl<'a, T: Arbitrary<'a>> Arbitrary<'a> for Sensitive<T> {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self(u.arbitrary()?))
    }
}

#[cfg(feature = "testing")]
impl<'a, T: NegateArbitrary<'a> + Arbitrary<'a>> NegateArbitrary<'a> for Sensitive<T> {
    fn negate_arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self(T::negate_arbitrary(u)?))
    }
}

#[cfg(all(test, feature = "testing"))]
mod tests {
    use proptest::*;

    use crate::testing::*;

    use super::*;

    proptest! {
        #[test]
        fn hides_internal_representation(s in gen::<Sensitive<String>>()) {
            assert_eq!(s.to_string(), MASK);
            assert_eq!(format!("{}", s), MASK);
            assert_eq!(
                format!("{:?}", s),
                format!("Sensitive(\"{}\")", MASK)
            );
        }

        #[test]
        fn preserves_equality(a in gen::<String>(), b in gen::<String>()) {
            prop_assert_eq!(
                Sensitive::new(a.clone()) == Sensitive::new(b.clone()),
                a == b
            );
        }

        #[test]
        fn preserves_order(a in gen::<usize>(), b in gen::<usize>()) {
            prop_assert_eq!(Sensitive::new(a) == Sensitive::new(b), a == b);
            prop_assert_eq!(Sensitive::new(a) >= Sensitive::new(b), a >= b);
            prop_assert_eq!(Sensitive::new(a) <= Sensitive::new(b), a <= b);
            prop_assert_eq!(Sensitive::new(a) > Sensitive::new(b), a > b);
            prop_assert_eq!(Sensitive::new(a) < Sensitive::new(b), a < b);
        }

        #[test]
        fn preserves_into(a in gen::<usize>()) {
            assert_eq!(Sensitive::from(a), Sensitive::new(a));
        }
    }
}
