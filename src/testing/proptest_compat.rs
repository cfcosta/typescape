use std::{fmt::Debug, marker::PhantomData, sync::RwLock};

use arbitrary::{Arbitrary, Unstructured};
use proptest::{
    strategy::{Strategy, ValueTree},
    string::RegexGeneratorStrategy,
    test_runner::{TestRng, TestRunner},
};
use rand::{Rng, RngCore};

pub struct PropTest<S: Strategy> {
    tree: S::Tree,
}

impl<S: Strategy> PropTest<S> {
    pub fn new(strategy: S, seed: [u8; 32]) -> Self {
        let config = proptest::test_runner::Config::default();
        let algo = config.rng_algorithm;

        let mut runner = TestRunner::new_with_rng(config, TestRng::from_seed(algo, &seed));
        let tree = strategy.new_tree(&mut runner).unwrap();

        Self { tree }
    }

    pub fn get(&self) -> S::Value {
        self.tree.current()
    }
}

pub struct ArbTree<T> {
    rng: RwLock<TestRng>,
    _marker: PhantomData<T>,
}

impl<'a, T> ArbTree<T>
where
    T: Debug + Arbitrary<'a>,
{
    pub fn new(rng: &mut TestRng) -> Self {
        let mut seed = [0u8; 32];
        rng.fill_bytes(&mut seed);

        let config = proptest::test_runner::Config::default();
        let algo = config.rng_algorithm;

        Self {
            rng: TestRng::from_seed(algo, &seed).into(),
            _marker: Default::default(),
        }
    }
}

impl<T> ValueTree for ArbTree<T>
where
    T: Debug + for<'a> Arbitrary<'a>,
{
    type Value = T;

    fn current(&self) -> Self::Value {
        let (min, max) = T::size_hint(0);
        let capacity = max.unwrap_or(min * 2) + 16;
        let mut data = vec![0u8; capacity];

        loop {
            let mut rng = self.rng.write().unwrap();
            rng.fill(&mut *data);

            let mut u = Unstructured::new(&data);

            let x = match T::arbitrary(&mut u) {
                Ok(x) => x,
                Err(arbitrary::Error::NotEnoughData) => {
                    // Double the buffer's size. Optionally have a max
                    // buffer size.
                    let new_len = data.len() * 2;
                    data.resize(new_len, 0);

                    continue;
                }
                Err(_) => {
                    // Just try again with new data. Optionally have a
                    // max number of retries.
                    continue;
                }
            };

            return x;
        }
    }

    fn simplify(&mut self) -> bool {
        false
    }

    fn complicate(&mut self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct ArbStrategy<T>(PhantomData<T>);

impl<T> Strategy for ArbStrategy<T>
where
    T: Debug + for<'a> Arbitrary<'a>,
{
    type Tree = ArbTree<T>;
    type Value = T;

    fn new_tree(&self, runner: &mut TestRunner) -> proptest::strategy::NewTree<Self> {
        Ok(ArbTree::new(runner.rng()))
    }
}

pub fn arb<'a, T>() -> ArbStrategy<T>
where
    T: Arbitrary<'a> + Debug,
    T: Debug + Arbitrary<'a>,
{
    ArbStrategy(Default::default())
}

pub fn from_regex<'a>(
    u: &mut Unstructured<'a>,
    regex: &str,
) -> PropTest<RegexGeneratorStrategy<String>> {
    let seed: [u8; 32] = u.arbitrary().unwrap();
    let strategy = proptest::string::string_regex(regex).unwrap();
    PropTest::new(strategy, seed)
}
