use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[cfg(feature = "arrow2")]
use arrow2::{
    array::{MutableUtf8Array, TryPush, Utf8Array},
    datatypes::DataType,
};
#[cfg(feature = "arrow2")]
use arrow2_convert::{deserialize::ArrowDeserialize, field::ArrowField, serialize::ArrowSerialize};

#[cfg(any(test, feature = "testing"))]
use fake::{faker::internet::en::SafeEmail, Fake};

#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;

use crate::{
    testing::{NegateArbitrary, Rng},
    *,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
/// A validated e-mail address
pub struct Email(String);

impl FromStr for Email {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if serde_email::is_valid_email(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(Error::FailedParsing(Kind::Email, s.to_string()))
        }
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Deref for Email {
    type Target = <String as Deref>::Target;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Email {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(any(test, feature = "testing"))]
impl Arbitrary for Email {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        any::<Rng>()
            .prop_map(|mut rng| SafeEmail().fake_with_rng(&mut rng.0))
            .prop_map(Self)
            .boxed()
    }
}

impl NegateArbitrary for Email {
    fn negate_arbitrary() -> <Self as Arbitrary>::Strategy {
        "[a-zA-Z0-9._%+-]+[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"
            .prop_map(Self)
            .boxed()
    }
}

#[cfg(feature = "arrow2")]
impl ArrowField for Email {
    type Type = Self;

    fn data_type() -> DataType {
        DataType::Utf8
    }
}

#[cfg(feature = "arrow2")]
impl ArrowSerialize for Email {
    type MutableArrayType = MutableUtf8Array<i64>;

    fn new_array() -> Self::MutableArrayType {
        MutableUtf8Array::<i64>::default()
    }

    fn arrow_serialize(
        v: &<Self as arrow2_convert::field::ArrowField>::Type,
        array: &mut Self::MutableArrayType,
    ) -> arrow2::error::Result<()> {
        array.try_push(Some(v.0.clone()))
    }
}

#[cfg(feature = "arrow2")]
impl ArrowDeserialize for Email {
    type ArrayType = Utf8Array<i64>;

    fn arrow_deserialize(
        v: <&Self::ArrayType as IntoIterator>::Item,
    ) -> Option<<Self as ArrowField>::Type> {
        v.map(|v| Self(v.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::testing::*;

    use super::*;

    proptest! {
        #[test]
        fn arbitrary_email_is_always_valid(a in any::<Email>()) {
            a.to_string().parse::<Email>().expect("Failed parsing");
        }

        #[test]
        fn invalid_emails_are_always_invalid(a in invalid::<Email>()) {
            assert_eq!(
                a.to_string().parse::<Email>(),
                Err(Error::FailedParsing(Kind::Email, a.to_string()))
            );
        }
    }
}
