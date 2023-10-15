use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

const MASK: &str = "******";

#[derive(Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
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

#[cfg(feature = "arbitrary")]
impl<'a, T: arbitrary::Arbitrary<'a>> arbitrary::Arbitrary<'a> for Sensitive<T> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self(u.arbitrary()?))
    }
}

#[cfg(all(test, feature = "arbitrary"))]
mod tests {
    use proptest::prelude::*;
    use proptest_arbitrary_interop::arb;

    use super::MASK;
    use crate::prelude::*;

    proptest! {
        #[test]
        fn hides_internal_representation(s in arb::<Sensitive<String>>()) {
            assert_eq!(s.to_string(), MASK);
            assert_eq!(format!("{}", s), MASK);
            assert_eq!(
                format!("{:?}", s),
                format!("Sensitive(\"{}\")", MASK)
            );
        }

        #[test]
        fn preserves_equality(a in arb::<String>(), b in arb::<String>()) {
            if a == b {
                assert_eq!(Sensitive::new(a), Sensitive::new(b));
            } else {
                assert_ne!(Sensitive::new(a), Sensitive::new(b));
            }
        }

        #[test]
        fn preserves_order(a in arb::<usize>(), b in arb::<usize>()) {
            if a == b {
                assert_eq!(Sensitive::new(a), Sensitive::new(b));
            } else if a > b {
                assert!(Sensitive::new(a) > Sensitive::new(b));
            } else {
                assert!(Sensitive::new(a) < Sensitive::new(b));
            }
        }

        #[test]
        fn preserves_into(a in arb::<usize>()) {
            assert_eq!(Sensitive::from(a), Sensitive::new(a));
        }
    }
}
