use std::{
    fmt::Debug,
    ops::{Add, Rem},
};

use arbitrary::{Arbitrary, Unstructured};

#[cfg(feature = "finances")]
use rust_decimal::{prelude::Zero, Decimal};

pub trait NumberExt {
    fn is_zero(&self) -> bool;
    fn is_positive(&self) -> bool;
    fn is_negative(&self) -> bool;
}

macro_rules! impl_number_ext {
    ($t:ty) => {
        impl NumberExt for $t {
            fn is_zero(&self) -> bool {
                *self == (0 as $t)
            }

            fn is_positive(&self) -> bool {
                *self > (0 as $t)
            }

            fn is_negative(&self) -> bool {
                *self < (0 as $t)
            }
        }
    };
}

impl_number_ext!(u8);
impl_number_ext!(u16);
impl_number_ext!(u32);
impl_number_ext!(u64);
impl_number_ext!(u128);
impl_number_ext!(usize);
impl_number_ext!(i8);
impl_number_ext!(i16);
impl_number_ext!(i32);
impl_number_ext!(i64);
impl_number_ext!(i128);
impl_number_ext!(isize);
impl_number_ext!(f32);
impl_number_ext!(f64);

#[cfg(feature = "finances")]
impl NumberExt for Decimal {
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }

    fn is_positive(&self) -> bool {
        self > &Self::zero()
    }

    fn is_negative(&self) -> bool {
        self < &Self::zero()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotZero<T>(pub T);

impl<'a, T> Arbitrary<'a> for NotZero<T>
where
    T: NumberExt + Arbitrary<'a>,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
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
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut res = T::arbitrary(u)?;

        while !res.is_positive() {
            res = T::arbitrary(u)?;
        }

        Ok(Self(res))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InBounds<T, const N: usize, const M: usize>(pub T)
where
    T: Debug;

impl<'a, T, const N: usize, const M: usize> Arbitrary<'a> for InBounds<T, N, M>
where
    T: From<usize>
        + Arbitrary<'a>
        + Rem<Output = T>
        + Add<Output = T>
        + PartialOrd
        + NumberExt
        + Debug,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut result = T::arbitrary(u)?;

        while N.is_positive() && result.is_negative() {
            result = T::arbitrary(u)?;
        }

        let result = (result % T::from(M)) + T::from(N);

        Ok(Self(result))
    }
}
