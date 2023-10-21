use std::ops::{Add, Sub};

use rust_decimal::Decimal;
use strum::{Display, EnumCount, EnumIter, EnumString, IntoEnumIterator};

mod asset;

pub use asset::Asset;

use crate::FinanceError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString, EnumCount, Display)]
pub enum Currency {
    CHF,
    EUR,
    USD,
    BRL,
}

impl<'a> arbitrary::Arbitrary<'a> for Currency {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let variants = Self::iter().collect::<Vec<_>>();
        let index = usize::arbitrary(u)? % variants.len();

        Ok(variants[index])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "testing", derive(arbitrary::Arbitrary))]
pub struct Money {
    pub currency: Currency,
    pub amount: Decimal,
}

impl Add for Money {
    type Output = Result<Self, FinanceError<Self>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.can_exchange(&rhs) {
            Ok(self.with_amount(self.amount() + rhs.amount()))
        } else {
            Err(FinanceError::MismatchedExchange {
                expected: self.exchange(),
                got: rhs.exchange(),
            })
        }
    }
}

impl Sub for Money {
    type Output = Result<Self, FinanceError<Self>>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.can_exchange(&rhs) {
            if self.amount() < rhs.amount() {
                return Err(FinanceError::NegativeAmount);
            }

            return Ok(self.with_amount(self.amount() - rhs.amount()));
        }

        Err(FinanceError::MismatchedExchange {
            expected: self.exchange(),
            got: rhs.exchange(),
        })
    }
}

impl Asset for Money {
    type Amount = Decimal;
    type Exchange = Currency;

    fn can_exchange(&self, other: &Self) -> bool {
        self.currency == other.currency
    }

    fn with_rate(&self, other: Self::Exchange, rate: Self::Amount) -> Self {
        Self {
            currency: other,
            amount: self.amount * rate,
        }
    }

    fn amount(&self) -> Self::Amount {
        self.amount
    }

    fn exchange(&self) -> Self::Exchange {
        self.currency
    }

    fn with_amount(&self, other: Self::Amount) -> Self {
        Self {
            amount: other,
            ..*self
        }
    }

    fn with_exchange(&self, other: Self::Exchange) -> Self {
        Self {
            currency: other,
            ..*self
        }
    }
}

impl Money {
    pub fn new(amount: Decimal, currency: Currency) -> Self {
        Self { currency, amount }
    }
}

#[cfg(all(test, feature = "testing"))]
mod tests {
    use proptest::*;
    use rust_decimal::FromPrimitive;

    use crate::{testing::*, *};

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct UpTo<const N: usize>(Decimal);

    impl<'a, const N: usize> arbitrary::Arbitrary<'a> for UpTo<N> {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            let decimal: Decimal = u.arbitrary()?;
            Ok(Self(decimal % Decimal::from_usize(N).unwrap()))
        }
    }

    proptest! {
        #[test]
        fn sum_commutates(
            UpTo(a) in arb::<UpTo<{ usize::MAX }>>(),
            UpTo(b) in arb::<UpTo<{ usize::MAX }>>(),
            currency in arb::<Currency>()
        ) {
            let a = Money::new(a, currency);
            let b = Money::new(b, currency);

            assert_eq!(a + b, b + a);
        }

        #[test]
        fn sums_correctly_if_same_currency(
            UpTo(a) in arb::<UpTo<{ usize::MAX }>>(),
            UpTo(b) in arb::<UpTo<{ usize::MAX }>>(),
            currency in arb::<Currency>()
        ) {
            let a = Money::new(a, currency);
            let b = Money::new(b, currency);

            assert_eq!((a + b).unwrap().amount, a.amount + b.amount);
            assert_eq!((b + a).unwrap().amount, b.amount + a.amount);
        }

        #[test]
        fn sum_returns_error_if_different_currency(
            a in arb::<UpTo<{ usize::MAX }>>(),
            b in arb::<UpTo<{ usize::MAX }>>(),
            InequalPair(ca, cb) in arb::<InequalPair<Currency>>(),
        ) {
            let a = Money::new(a.0, ca);
            let b = Money::new(b.0, cb);

            assert_eq!(
                a + b,
                Err(
                    FinanceError::MismatchedExchange {
                        expected: ca,
                        got: cb
                })
            );
        }

        #[test]
        fn subs_correctly_if_same_currency(
            a in arb::<UpTo<{ usize::MAX }>>(),
            b in arb::<UpTo<{ usize::MAX }>>(),
            currency in arb::<Currency>()
        ) {
            let (a,b) = if a > b { (a,b) } else { (b,a) };

            let a = Money::new(a.0, currency);
            let b = Money::new(b.0, currency);

            assert_eq!((a - b).unwrap().amount, a.amount - b.amount);
        }

        #[test]
        fn sub_returns_error_if_different_currency(
            UpTo(value) in arb::<UpTo<{ usize::MAX }>>(),
            InequalPair(ca, cb) in arb::<InequalPair<Currency>>(),
        ) {
            let a = Money::new(value, ca);

            assert_eq!(
                a - a,
                Err(
                    FinanceError::MismatchedExchange {
                        expected: ca,
                        got: cb
                })
            );
        }

        #[test]
        fn sub_returns_error_if_it_would_be_negative(
            UpTo(a) in arb::<UpTo<{ usize::MAX }>>(),
            UpTo(b) in arb::<UpTo<{ usize::MAX }>>(),
            currency in arb::<Currency>(),
        ) {
            let (a, b) = match (a, b) {
                (a, b) if a > b => (a, b),
                (a, b) if a == b => (a + Decimal::from(1), b),
                _ => (b, a)
            };

            let a = Money::new(a, currency);
            let b = Money::new(b, currency);

            assert_eq!(
                b - a,
                Err(FinanceError::NegativeAmount)
            );
        }
    }
}
