use std::{fmt::Display, marker::PhantomData, str::FromStr};

use proptest::{
    prelude::{any, Arbitrary},
    strategy::{BoxedStrategy, Strategy},
};
use uuid::Uuid;

use crate::{Error, Kind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// An unique id pointing to a resource
pub struct Id<T> {
    inner: Uuid,
    _marker: PhantomData<T>,
}

impl<T> FromStr for Id<T> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<Uuid>()
            .map_err(|_| Error::FailedParsing(Kind::Id, s.to_string()))?
            .into())
    }
}

impl<T> From<Uuid> for Id<T> {
    fn from(inner: Uuid) -> Id<T> {
        Self {
            inner,
            _marker: Default::default(),
        }
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.inner)
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'a, T: serde::Deserialize<'a>> serde::Deserialize<'a> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        Ok(Uuid::deserialize(deserializer)?.into())
    }
}

#[cfg(feature = "testing")]
impl<T> Arbitrary for Id<T>
where
    T: Arbitrary + std::fmt::Debug + Into<Self> + 'static,
{
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        any::<u128>()
            .prop_map(Uuid::from_u128)
            .prop_map(Self::from)
            .boxed()
    }
}
