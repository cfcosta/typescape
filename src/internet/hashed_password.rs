use std::fmt::Display;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[cfg(any(test, feature = "testing"))]
use proptest::{prelude::*, strategy::BoxedStrategy};

use crate::Error;

#[derive(Debug)]
pub struct HashedPassword(String);

impl Display for HashedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl HashedPassword {
    pub fn generate(password: impl AsRef<str>) -> Result<Self, Error> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        let hash = argon2
            .hash_password(password.as_ref().as_bytes(), &salt)
            .map_err(|e| Error::PasswordHashing(e.to_string()))?
            .to_string();

        Ok(Self(hash))
    }

    pub fn verify_against(&self, other: impl AsRef<str>) -> bool {
        let parsed = PasswordHash::new(&self.0).unwrap();

        Argon2::default()
            .verify_password(other.as_ref().as_bytes(), &parsed)
            .is_ok()
    }
}

#[derive(Debug)]
#[cfg(any(test, feature = "testing"))]
pub struct PasswordPair {
    pub plain: String,
    pub hashed: HashedPassword,
}

#[cfg(any(test, feature = "testing"))]
impl Arbitrary for PasswordPair {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        any::<String>()
            .prop_map(|plain| Self {
                hashed: HashedPassword::generate(&plain).expect("Failed to hash password"),
                plain,
            })
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        // Argon, by design, takes some work to run, so we generate less test cases.
        #![proptest_config(ProptestConfig::with_cases(10))]
        #[test]
        fn arbitrary_pair_is_always_valid(pair in any::<PasswordPair>()) {
            prop_assert!(pair.hashed.verify_against(&pair.plain))
        }
    }
}
