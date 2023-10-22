#[cfg(feature = "finances")]
use num_bigint::BigUint;

#[cfg(feature = "finances")]
use num_traits::identities::Zero;

pub trait NumberExt {
    fn is_zero(&self) -> bool;
    fn is_positive(&self) -> bool;
    fn is_negative(&self) -> bool;
}

#[cfg(feature = "finances")]
impl NumberExt for BigUint {
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }

    fn is_positive(&self) -> bool {
        self > &Self::zero()
    }

    fn is_negative(&self) -> bool {
        self < &Self::zero()
    }
}

macro_rules! impl_number_ext {
    ($t:ty) => {
        impl NumberExt for $t {
            fn is_zero(&self) -> bool {
                *self == (0 as $t)
            }

            fn is_positive(&self) -> bool {
                *self > (0 as $t)
            }

            fn is_negative(&self) -> bool {
                *self < (0 as $t)
            }
        }
    };
}

impl_number_ext!(u8);
impl_number_ext!(u16);
impl_number_ext!(u32);
impl_number_ext!(u64);
impl_number_ext!(u128);
impl_number_ext!(usize);
impl_number_ext!(i8);
impl_number_ext!(i16);
impl_number_ext!(i32);
impl_number_ext!(i64);
impl_number_ext!(i128);
impl_number_ext!(isize);
impl_number_ext!(f32);
impl_number_ext!(f64);
