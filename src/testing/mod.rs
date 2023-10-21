use std::fmt::Debug;

use arbitrary::{Arbitrary, Unstructured};

mod invalid;
pub use self::invalid::*;

mod proptest_compat;
pub use self::proptest_compat::*;

#[derive(Debug, Clone)]
pub struct InequalPair<T>(pub T, pub T);

impl<'a, T> Arbitrary<'a> for InequalPair<T>
where
    T: Eq + Arbitrary<'a>,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let (a, mut b) = u.arbitrary()?;

        while a == b {
            b = u.arbitrary()?;
        }

        Ok(Self(a, b))
    }
}

/// Generates a "negated" version of an arbitrary type.
pub trait NegateArbitrary<'a>
where
    Self: Arbitrary<'a>,
{
    fn negate_arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self>;
}
