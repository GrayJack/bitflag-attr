#![doc = include_str!("../README.md")]
#![cfg_attr(all(not(test), not(feature = "std")), no_std)]

pub use bitflags_attr_macros::bitflag;

/// Primitive types that can be used with [`bitflag`] attribute implement this trait.
pub trait BitflagPrimitive: private::Sealed {}

mod private {
    pub trait Sealed {}
}

macro_rules! impl_primitive {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl $crate::private::Sealed for $ty {}
            impl $crate::BitflagPrimitive for $ty {}
        )+
    };
}

impl_primitive!(i8, i16, i32, i64, i128, isize);
impl_primitive!(u8, u16, u32, u64, u128, usize);

#[cfg(doc)]
pub mod example_generated;
