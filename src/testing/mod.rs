use arbitrary::{Arbitrary, Unstructured};

use proptest::{
    strategy::{Strategy, ValueTree},
    string::RegexGeneratorStrategy,
    test_runner::{Config, TestRng, TestRunner},
};

mod invalid;
pub use invalid::*;

#[derive(Debug, Clone)]
pub struct InequalPair<T>(pub T, pub T);

impl<'a, T> Arbitrary<'a> for InequalPair<T>
where
    T: PartialEq + Arbitrary<'a>,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let (a, mut b) = u.arbitrary()?;

        while a == b {
            b = u.arbitrary()?;
        }

        Ok(Self(a, b))
    }
}

/// Generates a "negated" version of an arbitrary type.
pub trait NegateArbitrary<'a>
where
    Self: Arbitrary<'a>,
{
    fn negate_arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self>;
}

pub struct PropTest<S: Strategy> {
    tree: S::Tree,
}

impl<S: Strategy> PropTest<S> {
    pub fn new(strategy: S, seed: [u8; 32]) -> Self {
        let config = Config::default();
        let algo = config.rng_algorithm;

        let mut runner = TestRunner::new_with_rng(config, TestRng::from_seed(algo, &seed));
        let tree = strategy.new_tree(&mut runner).unwrap();

        Self { tree }
    }

    pub fn get(&self) -> S::Value {
        self.tree.current()
    }
}

pub fn from_regex<'a>(
    u: &mut Unstructured<'a>,
    regex: &str,
) -> PropTest<RegexGeneratorStrategy<String>> {
    let seed: [u8; 32] = u.arbitrary().unwrap();
    let strategy = proptest::string::string_regex(regex).unwrap();
    PropTest::new(strategy, seed)
}
