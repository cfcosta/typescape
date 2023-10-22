pub trait Currency {
    fn ticker(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn decimals(&self) -> usize;
}

macro_rules! currency {
    ($t:tt,$name:expr,$decimals:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
        pub struct $t;

        impl Currency for $t {
            fn ticker(&self) -> &'static str {
                stringify!($t)
            }

            fn name(&self) -> &'static str {
                $name
            }

            fn decimals(&self) -> usize {
                $decimals
            }
        }
    };
}

// Fiat Currencies
currency!(USD, "United States Dollar", 2);
currency!(EUR, "Euro", 2);
currency!(GBP, "Sterling Pound", 2);

// Cryptocurrencies
currency!(BTC, "Bitcoin", 8);
currency!(ETH, "Ethereum", 18);
