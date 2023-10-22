use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub},
};

use num_bigint::BigUint;
use thiserror::Error;

use crate::testing::NumberExt;

pub mod currencies;
use currencies::Currency;

#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum Error {
    #[error("Negative amount is not allowed")]
    NegativeAmount,
}

#[derive(Debug, Clone)]
pub struct Money<C>(BigUint, C);

#[cfg(any(test, feature = "testing"))]
impl<C: Debug + Default> Arbitrary for Money<C> {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        any::<u128>()
            .prop_filter("must have positive amount", |d| {
                d.is_zero() || d.is_positive()
            })
            .prop_map(BigUint::from)
            .prop_map(|i| Self(i, Default::default()))
            .boxed()
    }
}

impl<C: Default + Currency> Money<C> {
    pub fn new(amount: impl Into<BigUint>) -> Self {
        Self(amount.into(), C::default())
    }
}

impl<C> PartialOrd for Money<C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<C> PartialEq for Money<C> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<C: Default> Add for Money<C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, C::default())
    }
}

impl<C> AddAssign for Money<C> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<C: Default> Sub for Money<C> {
    type Output = Result<Self, Error>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self > rhs {
            Ok(Self(self.0 - rhs.0, C::default()))
        } else {
            Err(Error::NegativeAmount)
        }
    }
}

impl<C: Default> Mul for Money<C> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, C::default())
    }
}

impl<C> MulAssign for Money<C> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl<C: Default> Div for Money<C> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0, C::default())
    }
}

impl<C> DivAssign for Money<C> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

#[cfg(any(test, feature = "testing"))]
impl<C> NumberExt for Money<C> {
    fn is_zero(&self) -> bool {
        NumberExt::is_zero(&self.0)
    }

    fn is_positive(&self) -> bool {
        NumberExt::is_positive(&self.0)
    }

    fn is_negative(&self) -> bool {
        NumberExt::is_negative(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::testing::*;

    use super::{currencies::USD, *};

    type M = Money<USD>;

    proptest! {
        #[test]
        fn has_factory(a in any::<M>()) {
            prop_assert!(a.0.is_zero() || a.0.is_positive())
        }

        #[test]
        fn converts_from_usize(a in any::<usize>()) {
            prop_assert_eq!(M::new(a), M::new(BigUint::from(a)));
        }

        #[test]
        fn maintains_equality(a in 0..u128::MAX, b in 0..u128::MAX) {
            prop_assert_eq!(M::new(a) == M::new(b), a == b);
        }

        #[test]
        fn maintains_ordering(a in 0..u128::MAX, b in 0..u128::MAX) {
            prop_assert_eq!(M::new(a) == M::new(b), a == b);
            prop_assert_eq!(M::new(a) >= M::new(b), a >= b);
            prop_assert_eq!(M::new(a) <= M::new(b), a <= b);
            prop_assert_eq!(M::new(a) > M::new(b), a > b);
            prop_assert_eq!(M::new(a) < M::new(b), a < b);
        }

        #[test]
        fn allows_addition(
            a in (0..u32::MAX).prop_map(|x| x as u128),
            b in (0..u32::MAX).prop_map(|x| x as u128),
        ) {
            prop_assert_eq!(M::new(a) + M::new(b), M::new(a + b));
            prop_assert_eq!(M::new(a) + M::new(b), M::new(b + a));
        }

        #[test]
        fn allows_add_assign(
            a in (0..u32::MAX).prop_map(|x| x as u128),
            b in (0..u32::MAX).prop_map(|x| x as u128),
        ) {
            let mut ma = M::new(a);
            ma += M::new(b);

            prop_assert_eq!(ma.clone(), M::new(a + b));
            prop_assert_eq!(ma, M::new(b + a));
        }

        #[test]
        fn allows_subtraction_as_result((b, a) in in_order::<u128>()) {
            prop_assert_eq!(M::new(a) - M::new(b), Ok(M::new(a - b)));
        }

        #[test]
        fn only_allow_positive_amounts_when_subtracting(
            a in 0..u128::MAX,
            b in 0..u128::MAX,
        ) {
            prop_assert_eq!((M::new(a) - M::new(b)).is_ok(), a > b);
        }

        #[test]
        fn allows_multiplication(
            a in (0..u32::MAX).prop_map(|x| x as u128),
            b in (0..u32::MAX).prop_map(|x| x as u128),
        ) {
            prop_assert_eq!(M::new(a) * M::new(b), M::new(a * b));
            prop_assert_eq!(M::new(a) * M::new(b), M::new(b * a));
        }

        #[test]
        fn allows_mul_assign(
            a in (0..u32::MAX).prop_map(|x| x as u128),
            b in (0..u32::MAX).prop_map(|x| x as u128),
        ) {
            let mut ma = M::new(a);
            ma *= M::new(b);

            prop_assert_eq!(ma.clone(), M::new(a * b));
            prop_assert_eq!(ma, M::new(b * a));
        }

        #[test]
        fn allows_division(a in 0..u128::MAX, b in 1..u128::MAX) {
            prop_assert_eq!(M::new(a) / M::new(b), M::new(a / b));
        }

        #[test]
        fn allows_div_assign(a in 0..u128::MAX, b in 1..u128::MAX) {
            let mut ma = M::new(a);
            ma /= M::new(b);

            prop_assert_eq!(ma, M::new(a / b));
        }
    }
}
