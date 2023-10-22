use std::{
    fmt::Debug,
    ops::{Add, Rem},
};

use arbitrary::Arbitrary;
use proptest::strategy::Strategy;

pub mod invalid;
pub mod numeric;
pub mod pairs;
mod proptest_compat;

pub use self::proptest_compat::*;

pub use self::{invalid::NegateArbitrary, numeric::NumberExt};

use self::{
    invalid::Invalid,
    numeric::InBounds,
    pairs::{InequalPair, SortedPair},
};

pub fn inequal_pair<T: PartialOrd + Debug + for<'a> Arbitrary<'a>>() -> impl Strategy<Value = (T, T)>
{
    gen::<InequalPair<T>>().prop_map(|x| (x.0, x.1))
}

pub fn sorted_pair<T: PartialOrd + Debug + for<'a> Arbitrary<'a>>() -> impl Strategy<Value = (T, T)>
{
    gen::<SortedPair<T>>().prop_map(|x| (x.0, x.1))
}

pub fn invalid<T>() -> impl Strategy<Value = T>
where
    T: Debug + for<'a> NegateArbitrary<'a>,
{
    crate::testing::gen::<Invalid<T>>().prop_map(move |x| x.0)
}

pub fn bound<
    T: From<usize>
        + Rem<Output = T>
        + Add<Output = T>
        + PartialOrd
        + NumberExt
        + Debug
        + for<'a> Arbitrary<'a>,
    const M: usize,
    const N: usize,
>() -> impl proptest::prelude::Strategy<Value = T> {
    gen::<InBounds<T, M, N>>().prop_map(|x| x.0)
}
