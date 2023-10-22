use arbitrary::Arbitrary;

#[cfg(feature = "finances")]
use rust_decimal::{prelude::Zero, Decimal};

pub trait NumberExt {
    fn is_zero(&self) -> bool;
    fn is_positive(&self) -> bool;
}

macro_rules! implement_is_zero {
    ($t:ty) => {
        impl NumberExt for $t {
            fn is_zero(&self) -> bool {
                *self == (0 as $t)
            }

            fn is_positive(&self) -> bool {
                *self > (0 as $t)
            }
        }
    };
}

implement_is_zero!(u8);
implement_is_zero!(u16);
implement_is_zero!(u32);
implement_is_zero!(u64);
implement_is_zero!(u128);
implement_is_zero!(i8);
implement_is_zero!(i16);
implement_is_zero!(i32);
implement_is_zero!(i64);
implement_is_zero!(i128);
implement_is_zero!(f32);
implement_is_zero!(f64);

#[cfg(feature = "finances")]
impl NumberExt for Decimal {
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }

    fn is_positive(&self) -> bool {
        self > &Self::zero()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotZero<T>(pub T);

impl<'a, T> Arbitrary<'a> for NotZero<T>
where
    T: NumberExt + Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut res = T::arbitrary(u)?;

        while res.is_zero() {
            res = T::arbitrary(u)?;
        }

        Ok(Self(res))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Positive<T>(pub T);

impl<'a, T> Arbitrary<'a> for Positive<T>
where
    T: NumberExt + Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut res = T::arbitrary(u)?;

        while !res.is_positive() {
            res = T::arbitrary(u)?;
        }

        Ok(Self(res))
    }
}
