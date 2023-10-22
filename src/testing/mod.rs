use std::fmt::Debug;

use arbitrary::{Arbitrary, Unstructured};
use proptest::strategy::Strategy;

mod invalid;
mod numeric;

pub use self::invalid::*;
pub use self::numeric::*;

mod proptest_compat;
pub use self::proptest_compat::*;

#[derive(Debug, Clone)]
pub struct InequalPair<T>(pub T, pub T);

impl<'a, T> Arbitrary<'a> for InequalPair<T>
where
    T: PartialEq + Arbitrary<'a>,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let (a, mut b) = u.arbitrary()?;

        while a == b {
            b = T::arbitrary(u)?;
        }

        Ok(Self(a, b))
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        let (min, max) = T::size_hint(depth);

        (min * 2, max.map(|max| max * 2))
    }
}

pub fn inequal_pair<T: PartialOrd + Debug + for<'a> Arbitrary<'a>>() -> impl Strategy<Value = (T, T)>
{
    gen::<InequalPair<T>>().prop_map(|x| (x.0, x.1))
}

#[derive(Debug, Clone)]
pub struct SortedPair<T>(pub T, pub T);

impl<'a, T> Arbitrary<'a> for SortedPair<T>
where
    T: PartialOrd + PartialEq + Arbitrary<'a>,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let InequalPair(a, b) = InequalPair::arbitrary(u)?;

        if a > b {
            return Ok(Self(a, b));
        }

        Ok(Self(b, a))
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        let (min, max) = T::size_hint(depth);
        (min * 2, max.map(|max| max * 2))
    }
}

pub fn sorted_pair<T: PartialOrd + Debug + for<'a> Arbitrary<'a>>() -> impl Strategy<Value = (T, T)>
{
    gen::<SortedPair<T>>().prop_map(|x| (x.0, x.1))
}

/// Generates a "negated" version of an arbitrary type.
pub trait NegateArbitrary<'a>
where
    Self: Arbitrary<'a>,
{
    fn negate_arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self>;
}
