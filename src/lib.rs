#![doc = include_str!("../README.md")]
#![cfg_attr(all(not(test), not(feature = "std")), no_std)]

use core::{
    fmt,
    ops::{BitAnd, BitOr, BitXor, Not},
};

pub use bitflags_attr_macros::bitflag;

pub mod iter;
pub mod parser;

/// Primitive types that can be used with [`bitflag`] attribute implement this trait.
pub trait BitsPrimitive:
    private::Sealed
    + Copy
    + PartialEq
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
    + fmt::UpperHex
    + Sized
    + 'static
{
    /// A value with all bits unset.
    const EMPTY: Self;

    /// A value with all bits set.
    const ALL: Self;
}

mod private {
    pub trait Sealed {}
}

macro_rules! impl_primitive {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl $crate::private::Sealed for $ty {}
            impl $crate::BitsPrimitive for $ty {
                const EMPTY: Self = 0;
                const ALL: Self = !0;
            }
            impl $crate::parser::ParseHex for $ty {
                fn parse_hex(input: &str) -> Result<Self, $crate::parser::ParseError>
                where
                    Self: Sized
                {
                    <$ty>::from_str_radix(input, 16).map_err(|_| $crate::parser::ParseError::invalid_hex_flag(input))
                }
            }
        )+
    };
}

impl_primitive!(i8, i16, i32, i64, i128, isize);
impl_primitive!(u8, u16, u32, u64, u128, usize);

/// A set of defined flags using a bits type as storage.
///
/// ## Implementing `Flags`
///
/// This trait is implemented by the [`bitflag`](crate::bitflag) macro:
///
/// ```
/// use bitflag_attr::bitflag;
///
/// #[bitflag(u8)]
/// #[derive(Clone, Copy)]
/// enum MyFlags {
///   A = 1,
///   B = 1 << 1,
/// }
/// ```
///
/// It can also be implemented manually:
///
/// ```
/// use bitflag_attr::{Flags};
///
/// #[derive(Clone, Copy)]
/// struct MyFlags(u8);
///
/// impl Flags for MyFlags {
///     const KNOWN_FLAGS: &'static [(&'static str, Self)] = &[
///         ("A", MyFlags(1)),
///         ("B", MyFlags(1 << 1)),
///     ];
///
///     const EXTRA_VALID_BITS: Self::Bits = 1 | (1 << 1);
///
///     type Bits = u8;
///
///     fn from_bits_retain(bits: Self::Bits) -> Self {
///         MyFlags(bits)
///     }
///
///     fn bits(&self) -> Self::Bits {
///         self.0
///     }
/// }
/// ```
///
/// ## Using `Flags`
///
/// The `Flags` trait can be used generically to work with any flags types. In this example,
/// we can count the number of defined named flags:
///
/// ```
/// # use bitflag_attr::{bitflag, Flags};
/// fn defined_flags<F: Flags>() -> usize {
///     F::KNOWN_FLAGS.iter().count()
/// }
///
/// #[bitflag(u8)]
/// #[non_exhaustive]
/// #[derive(Clone, Copy)]
/// enum MyFlags {
///     A = 1,
///     B = 1 << 1,
///     C = 1 << 2,
/// }
///
/// assert_eq!(3, defined_flags::<MyFlags>());
/// ```
pub trait Flags: Sized + Copy + 'static {
    /// The set of named defined flags.
    const KNOWN_FLAGS: &'static [(&'static str, Self)];

    /// Extra possible bits values for the flags.
    ///
    /// Useful for externally defined flags
    const EXTRA_VALID_BITS: Self::Bits;

    /// The underlying bits type.
    type Bits: BitsPrimitive;

    /// Return the underlying bits of this bitflag.
    ///
    /// The returned value is exactly the bits set in this flags value.
    fn bits(&self) -> Self::Bits;

    /// Convert from `bits` value exactly.
    fn from_bits_retain(bits: Self::Bits) -> Self;

    /// Converts from a `bits` value. Returning [`None`] is any unknown bits are set.
    fn from_bits(bits: Self::Bits) -> Option<Self> {
        let truncated = Self::from_bits_truncate(bits);

        if truncated.bits() == bits {
            Some(truncated)
        } else {
            None
        }
    }

    /// Convert from `bits` value, unsetting any unknown bits.
    fn from_bits_truncate(bits: Self::Bits) -> Self {
        Self::from_bits_retain(bits & Self::all().bits())
    }

    /// Convert from a flag `name`.
    #[inline]
    fn from_flag_name(name: &str) -> Option<Self> {
        // Don't parse empty names as empty flags
        if name.is_empty() {
            return None;
        }

        Self::KNOWN_FLAGS
            .iter()
            .find(|(s, _)| *s == name)
            .map(|(_, v)| Self::from_bits_retain(v.bits()))
    }

    /// Get a flags value with the bits of a flag with the given name set.
    ///
    /// This method will return `None` if `name` is empty or doesn't
    /// correspond to any named flag.
    fn from_name(name: &str) -> Option<Self> {
        // Don't parse empty names as empty flags
        if name.is_empty() {
            return None;
        }

        for (flag_name, flag) in Self::KNOWN_FLAGS {
            if *flag_name == name {
                return Some(Self::from_bits_retain(flag.bits()));
            }
        }

        None
    }

    /// Construct a flag value with all bits unset.
    fn empty() -> Self {
        Self::from_bits_retain(Self::Bits::EMPTY)
    }

    /// Returns `true` if the flag value has all bits unset.
    fn is_empty(&self) -> bool {
        self.bits() == Self::Bits::EMPTY
    }

    /// Returns a flag value that contains all value.
    ///
    /// This will include bits that do not have any flags/meaning.
    /// Use [`all`](Flags::all) if you want only the specified flags set.
    fn all_bits() -> Self {
        Self::from_bits_retain(Self::Bits::ALL)
    }

    /// Returns `true` if the bitflag contains all value bits set.
    ///
    /// This will check for all bits.
    /// Use [`is_all`](Flags::is_all) if you want to check for all specified flags.
    fn is_all_bits(&self) -> bool {
        self.bits() == Self::Bits::ALL
    }

    /// Construct a flag value with all known flags set.
    ///
    /// This will only set the flags specified as associated constant.
    fn all() -> Self {
        let mut truncated = Self::Bits::EMPTY;

        for (_, flag) in Self::KNOWN_FLAGS.iter() {
            truncated = truncated | flag.bits();
        }

        truncated = truncated | Self::EXTRA_VALID_BITS;

        Self::from_bits_retain(truncated)
    }

    /// Whether all known bits in this flags value are set.
    fn is_all(&self) -> bool {
        // NOTE: We check against `Self::all` here, not `Self::Bits::ALL`
        // because the set of all flags may not use all bits
        Self::all().bits() | self.bits() == self.bits()
    }

    /// Returns `true` if there are any unknown bits set in the flag value.
    fn contains_unknown_bits(&self) -> bool {
        Self::all().bits() & self.bits() != self.bits()
    }

    /// Returns a bit flag that only has bits corresponding to the specified flags as associated constant.
    fn truncated(&self) -> Self {
        Self::from_bits_retain(self.bits() & Self::all().bits())
    }

    /// Returns `true` if this flag value intersects with any value in `other`.
    ///
    /// This is equivalent to `(self & other) != Self::empty()`
    fn intersects(&self, other: Self) -> bool
    where
        Self: Sized,
    {
        self.bits() & other.bits() != Self::Bits::EMPTY
    }

    /// Returns `true` if this flag value contains all values of `other`.
    ///
    /// This is equivalent to `(self & other) == other`
    fn contains(&self, other: Self) -> bool
    where
        Self: Sized,
    {
        self.bits() & other.bits() == other.bits()
    }

    /// Remove any unknown bits from the flags.
    fn truncate(&mut self)
    where
        Self: Sized,
    {
        *self = Self::from_bits_truncate(self.bits());
    }

    /// Returns the intersection from this value with `other`.
    #[must_use]
    fn intersection(self, other: Self) -> Self {
        Self::from_bits_retain(self.bits() & other.bits())
    }

    /// Returns the union from this value with `other`.
    #[must_use]
    fn union(self, other: Self) -> Self {
        Self::from_bits_retain(self.bits() | other.bits())
    }

    /// Returns the difference from this value with `other`.
    ///
    /// In other words, returns the intersection of this value with the negation of `other`.
    ///
    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
    /// `difference` won't truncate `other`, but the `!` operator will.
    #[must_use]
    fn difference(self, other: Self) -> Self {
        Self::from_bits_retain(self.bits() & !other.bits())
    }

    /// TReturns the symmetric difference from this value with `other`..
    #[must_use]
    fn symmetric_difference(self, other: Self) -> Self {
        Self::from_bits_retain(self.bits() ^ other.bits())
    }

    /// Returns the complement of the value.
    ///
    /// This is very similar to the [`not`](Self::not), but truncates non used bits.
    #[must_use]
    fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }

    /// Set the flags in `other` in the value.
    fn set(&mut self, other: Self)
    where
        Self: Sized,
    {
        *self = Self::from_bits_retain(self.bits()).union(other);
    }

    /// /// Unset the flags bits in `other` in the value.
    ///
    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
    /// `remove` won't truncate `other`, but the `!` operator will.
    fn unset(&mut self, other: Self)
    where
        Self: Sized,
    {
        *self = Self::from_bits_retain(self.bits()).difference(other);
    }

    /// Toggle the flags in `other` in the value.
    fn toggle(&mut self, other: Self)
    where
        Self: Sized,
    {
        *self = Self::from_bits_retain(self.bits()).symmetric_difference(other);
    }

    /// Yield a set of contained flags values.
    ///
    /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
    /// will be yielded together as a final flags value.
    fn iter(&self) -> iter::Iter<Self> {
        iter::Iter::new(self)
    }

    /// Yield a set of contained named flags values.
    ///
    /// This method is like [`Flags::iter`], except only yields bits in contained named flags.
    /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
    fn iter_names(&self) -> iter::IterNames<Self> {
        iter::IterNames::new(self)
    }
}

#[cfg(doc)]
pub mod example_generated;
