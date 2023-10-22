use std::{
    fmt::Debug,
    ops::{Add, Rem},
};

use proptest::{
    prelude::*,
    test_runner::{Config, TestRng},
};

pub mod numeric;

pub use self::numeric::NumberExt;

/// Generates a "negated" version of an arbitrary type.
pub trait NegateArbitrary
where
    Self: Arbitrary,
{
    fn negate_arbitrary() -> <Self as Arbitrary>::Strategy;
}

#[derive(Debug)]
/// A wrapper for implementing types that require a RNG instance to be generated
pub struct Rng(pub TestRng);

impl Arbitrary for Rng {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        any::<[u8; 32]>()
            .prop_map(|seed| {
                let config = Config::default();
                TestRng::from_seed(config.rng_algorithm, &seed)
            })
            .prop_map(Self)
            .boxed()
    }
}

/// Return "invalid" instances of types
pub fn invalid<T: NegateArbitrary>() -> impl Strategy<Value = T> {
    T::negate_arbitrary()
}

/// Returns a pair (a,b) such that a != b
pub fn different<T>() -> impl Strategy<Value = (T, T)>
where
    T: PartialOrd + Debug + Arbitrary,
{
    any::<(T, T)>().prop_filter("values must not be equal", |(a, b)| a != b)
}

/// Returns a pair (a, b) such that a < b
pub fn in_order<T>() -> impl Strategy<Value = (T, T)>
where
    T: PartialOrd + Debug + Arbitrary,
{
    any::<(T, T)>().prop_filter(
        "values must be different and 0 should be smaller than 1",
        |(a, b)| a < b,
    )
}

/// Returns a pair of type T (that can be made from a usize) that is between M and N
pub fn between<T, const M: usize, const N: usize>() -> impl Strategy<Value = T>
where
    T: From<usize> + Rem<Output = T> + Add<Output = T> + PartialOrd + NumberExt + Debug + Arbitrary,
{
    (M..N)
        .prop_filter("value must be inside bounds", |&t| t >= M && t <= N)
        .prop_map(T::from)
}

/// Returns a positive T
pub fn positive<T: Arbitrary + NumberExt>() -> impl Strategy<Value = T> {
    any::<T>().prop_filter("value must be positive", |t| t.is_positive())
}

/// Returns a negative T
pub fn negative<T: Arbitrary + NumberExt>() -> impl Strategy<Value = T> {
    any::<T>().prop_filter("value must be negative", |t| t.is_negative())
}
