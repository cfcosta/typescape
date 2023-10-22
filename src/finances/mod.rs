use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub};

use rust_decimal::Decimal;
use thiserror::Error;

use crate::testing::NumberExt;

pub mod currencies;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum Error {
    #[error("Negative amount is not allowed")]
    NegativeAmount,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "testing", derive(proptest_derive::Arbitrary))]
pub struct Money<C>(Decimal, C);

impl<C: Default> Money<C> {
    pub fn new(amount: impl Into<Decimal>) -> Self {
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

impl<C> Eq for Money<C> {}

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

#[cfg(feature = "testing")]
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

#[cfg(all(test, feature = "testing"))]
mod tests {
    use proptest::prelude::*;

    use crate::testing::*;

    use super::{currencies::USD, *};

    type M = Money<USD>;

    proptest! {
        #[test]
        fn converts_from_usize(
            a in any::<usize>(),
        ) {
            prop_assert_eq!(Money::<USD>::new(a), Money::<USD>::new(Decimal::from(a)));
        }

        #[test]
        fn maintains_equality(
            a in between::<Decimal, 0, { u32::MAX as usize }>(),
            b in between::<Decimal, 0, { u32::MAX as usize }>(),
        ) {
            prop_assert_eq!(M::new(a) == M::new(b), a == b);
        }

        #[test]
        fn maintains_ordering(
            a in between::<Decimal, 0, { u32::MAX as usize }>(),
            b in between::<Decimal, 0, { u32::MAX as usize }>(),
        ) {
            prop_assert_eq!(M::new(a) == M::new(b), a == b);
            prop_assert_eq!(M::new(a) >= M::new(b), a >= b);
            prop_assert_eq!(M::new(a) <= M::new(b), a <= b);
            prop_assert_eq!(M::new(a) > M::new(b), a > b);
            prop_assert_eq!(M::new(a) < M::new(b), a < b);
        }

        #[test]
        fn allows_addition(
            a in between::<Decimal, 0, { u32::MAX as usize }>(),
            b in between::<Decimal, 0, { u32::MAX as usize }>(),
        ) {
            prop_assert_eq!(M::new(a) + M::new(b), M::new(a + b));
            prop_assert_eq!(M::new(a) + M::new(b), M::new(b + a));
        }

        #[test]
        fn allows_add_assign(
            a in between::<Decimal, 0, { u32::MAX as usize }>(),
            b in between::<Decimal, 0, { u32::MAX as usize }>(),
        ) {
            let mut ma = M::new(a);
            ma += M::new(b);

            prop_assert_eq!(ma, M::new(a + b));
            prop_assert_eq!(ma, M::new(b + a));
        }

        #[test]
        fn allows_subtraction_as_result(
            (b, a) in in_order::<u32>().prop_map(|(a,b)| (a as usize, b as usize)).prop_map(Into::into)
        ) {
            prop_assert_eq!(M::new(a) - M::new(b), Ok(M::new(a - b)));
        }

        #[test]
        fn only_allow_positive_amounts_when_subtracting(
            a in between::<Decimal, 0, { u32::MAX as usize }>(),
            b in between::<Decimal, 0, { u32::MAX as usize }>(),
        ) {
            prop_assert_eq!((M::new(a) - M::new(b)).is_ok(), a > b);
        }

        #[test]
        fn allows_multiplication(
            a in between::<Decimal, 0, { u32::MAX as usize }>(),
            b in between::<Decimal, 0, { u32::MAX as usize }>(),
        ) {
            prop_assert_eq!(M::new(a) * M::new(b), M::new(a * b));
            prop_assert_eq!(M::new(a) * M::new(b), M::new(b * a));
        }

        #[test]
        fn allows_mul_assign(
            a in between::<Decimal, 0, { u32::MAX as usize }>(),
            b in between::<Decimal, 0, { u32::MAX as usize }>(),
        ) {
            let mut ma = M::new(a);
            ma *= M::new(b);

            prop_assert_eq!(ma, M::new(a * b));
            prop_assert_eq!(ma, M::new(b * a));
        }

        #[test]
        fn allows_division(
            a in between::<Decimal, 0, { u32::MAX as usize }>(),
            b in between::<Decimal, 1, { u32::MAX as usize }>(),
        ) {
            prop_assert_eq!(M::new(a) / M::new(b), M::new(a / b));
        }

        #[test]
        fn allows_div_assign(
            a in between::<Decimal, 0, { u32::MAX as usize }>(),
            b in between::<Decimal, 1, { u32::MAX as usize }>(),
        ) {
            let mut ma = M::new(a);
            ma /= M::new(b);

            prop_assert_eq!(ma, M::new(a / b));
        }
    }
}
