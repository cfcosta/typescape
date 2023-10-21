use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

pub trait Asset {
    type Amount: Add + AddAssign + Sub + SubAssign + Display + PartialEq + Eq;

    /// TODO: Find a better name for this, it is like currency, but for stocks, crypto and other
    /// resources.
    type Exchange: Display + PartialEq;

    fn can_exchange(&self, other: &Self) -> bool;
    fn with_rate(&self, other: Self::Exchange, rate: Self::Amount) -> Self;

    fn amount(&self) -> Self::Amount;
    fn exchange(&self) -> Self::Exchange;

    fn with_amount(&self, other: Self::Amount) -> Self;
    fn with_exchange(&self, other: Self::Exchange) -> Self;
}

#[async_trait::async_trait]
pub trait Exchange {
    type Medium: Asset;
    type Error;

    async fn convert(
        &self,
        from: <Self::Medium as Asset>::Amount,
        to: <Self::Medium as Asset>::Exchange,
    ) -> Result<Self::Medium, Self::Error>;
}
