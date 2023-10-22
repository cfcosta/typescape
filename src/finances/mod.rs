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
#[cfg_attr(feature = "testing", derive(arbitrary::Arbitrary))]
pub struct Money<C>(Decimal, C);

impl<C: Default> Money<C> {
    pub fn new(amount: Decimal) -> Self {
        Self(amount, C::default())
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
        self.0.is_zero()
    }

    fn is_positive(&self) -> bool {
        self.0.is_sign_positive()
    }
}

#[cfg(all(test, feature = "testing"))]
mod tests {
    use crate::testing::*;
    use proptest::prelude::*;

    use super::{currencies::USD, *};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct BoundedDecimal<const N: usize>(Decimal, usize);

    impl<'a, const N: usize> arbitrary::Arbitrary<'a> for BoundedDecimal<N> {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            let max = Decimal::from(N);
            Ok(Self(Decimal::arbitrary(u)? % max, N))
        }
    }

    proptest! {
        #[test]
        fn maintains_equality(
            a in arb::<Decimal>(),
            b in arb::<Decimal>(),
        ) {
            prop_assert_eq!(Money::<USD>::new(a) == Money::new(b), a == b);
        }

        #[test]
        fn maintains_ordering(
            a in arb::<Decimal>(),
            b in arb::<Decimal>(),
        ) {
            prop_assert_eq!(Money::<USD>::new(a) == Money::new(b), a == b);
            prop_assert_eq!(Money::<USD>::new(a) >= Money::new(b), a >= b);
            prop_assert_eq!(Money::<USD>::new(a) <= Money::new(b), a <= b);
            prop_assert_eq!(Money::<USD>::new(a) > Money::new(b), a > b);
            prop_assert_eq!(Money::<USD>::new(a) < Money::new(b), a < b);
        }

        #[test]
        fn allows_addition(
            BoundedDecimal(a, _) in arb::<BoundedDecimal<{ u32::MAX as usize }>>(),
            BoundedDecimal(b, _) in arb::<BoundedDecimal<{ u32::MAX as usize }>>(),
        ) {
            prop_assert_eq!(Money::<USD>::new(a) + Money::new(b), Money::new(a + b));
            prop_assert_eq!(Money::<USD>::new(a) + Money::new(b), Money::new(b + a));
        }

        #[test]
        fn allows_add_assign(
            BoundedDecimal(a, _) in arb::<BoundedDecimal<{ u32::MAX as usize }>>(),
            BoundedDecimal(b, _) in arb::<BoundedDecimal<{ u32::MAX as usize }>>(),
        ) {
            let mut ma = Money::<USD>::new(a);
            ma += Money::new(b);

            prop_assert_eq!(ma, Money::new(a + b));
            prop_assert_eq!(ma, Money::new(b + a));
        }

        #[test]
        fn allows_subtraction_as_result(
            SortedPair(a, b) in arb::<SortedPair<Decimal>>(),
        ) {
            prop_assert_eq!(Money::<USD>::new(a) - Money::new(b), Ok(Money::new(a - b)));
        }

        #[test]
        fn only_allow_positive_amounts_when_subtracting(
            a in arb::<Decimal>(),
            b in arb::<Decimal>(),
        ) {
            prop_assert_eq!((Money::<USD>::new(a) - Money::<USD>::new(b)).is_ok(), a > b);
        }

        #[test]
        fn allows_multiplication(
            BoundedDecimal(a, _) in arb::<BoundedDecimal<{ u32::MAX as usize }>>(),
            BoundedDecimal(b, _) in arb::<BoundedDecimal<{ u32::MAX as usize }>>(),
        ) {
            prop_assert_eq!(Money::<USD>::new(a) * Money::new(b), Money::new(a * b));
            prop_assert_eq!(Money::<USD>::new(a) * Money::new(b), Money::new(b * a));
        }

        #[test]
        fn allows_mul_assign(
            BoundedDecimal(a, _) in arb::<BoundedDecimal<{ u32::MAX as usize }>>(),
            BoundedDecimal(b, _) in arb::<BoundedDecimal<{ u32::MAX as usize }>>(),
        ) {
            let mut ma = Money::<USD>::new(a);
            ma *= Money::new(b);

            prop_assert_eq!(ma, Money::new(a * b));
            prop_assert_eq!(ma, Money::new(b * a));
        }

        #[test]
        fn allows_division(
            a in arb::<Decimal>(),
            b in arb::<Decimal>(),
        ) {
            prop_assert_eq!(Money::<USD>::new(a) / Money::new(b), Money::new(a / b));
        }

        #[test]
        fn allows_div_assign(
            a in arb::<Decimal>(),
            b in arb::<Decimal>(),
        ) {
            let mut ma = Money::<USD>::new(a);
            ma /= Money::new(b);

            prop_assert_eq!(ma, Money::new(a / b));
        }
    }
}
