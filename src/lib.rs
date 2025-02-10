//! Generate types for C-style flags with ergonomic APIs using attribute macros and enums.
//!
//! # Getting started
//!
//! Add `bitflag_attr` to your `Cargo.toml`:
//!
//! ```sh
//! cargo add bitflag_attr
//! ```
//!
//! or
//!
//! ```toml
//! [dependencies]
//! bitflag-attr = "0.10.1"
//! ```
//!
//! ## Generating flags type
//!
//! Use the [`bitflag`] attribute macro to generate flag types:
//!
//! ```rust
//! use bitflag_attr::bitflag;
//!
//! #[bitflag(u32)]
//! #[derive(Clone, Copy)]
//! enum Flags {
//!     A = 0b00000001,
//!     B = 0b00000010,
//!     C = 0b00000100
//! }
//! ```
//!
//! Deriving [`Clone`] and [`Copy`] for the type is mandatory.
//!
//! The generated type is a **struct** wrapping the chosen primitive type.
//!
//! See the docs for the [`bitflag`] macro for the full syntax.
//!
//! Also see the [`example_generated`] module for an example of what the [`bitflag`] macro generates
//! for a flags type.
//!
//! ### Externally defined flags
//!
//! If you're generating flags types for an external source, such as a C API, you can use the
//! `non_exhaustive` attribute to communicate to the bitflags macro that there may be more valid
//! flags than the known flags.
//!
//! Without extra configuration, it defaults to `!0` (all bits set) as a mask of all bits the
//! external source may ever set, i.e. all bits are considered as possible values.
//!
//! ```rust
//! use bitflag_attr::bitflag;
//!
//! #[bitflag(u32)]
//! #[non_exhaustive] // All bits are considered as possible values.
//! #[derive(Debug, Clone, Copy)]
//! pub enum Flags {
//!     /// The value `A`, at bit position `0`.
//!     A = 0b00000001,
//!     /// The value `B`, at bit position `1`.
//!     B = 0b00000010,
//!     /// The value `C`, at bit position `2`.
//!     C = 0b00000100,
//!
//!     /// The combination of `A`, `B`, and `C`.
//!     ABC = A | B | C,
//! }
//! ```
//!
//! But you can also configure this value by using the helper attribute `extra_valid_bits` with a
//! desired value of valid bits that the external source may ever set.
//!
//! ```rust
//! use bitflag_attr::bitflag;
//!
//! #[bitflag(u32)]
//! #[non_exhaustive] // Communicate there is more potential valid flags than the known flags
//! #[extra_valid_bits = 0b001001111] // Specify the extra bits to take into consideration.
//! #[derive(Debug, Clone, Copy)]
//! pub enum Flags {
//!     /// The value `A`, at bit position `0`.
//!     A = 0b00000001,
//!     /// The value `B`, at bit position `1`.
//!     B = 0b00000010,
//!     /// The value `C`, at bit position `2`.
//!     C = 0b00000100,
//!
//!     /// The combination of `A`, `B`, and `C`.
//!     ABC = A | B | C,
//! }
//! ```
//!
//! Why should you do this? Generated methods like `all` and truncating operators like `!` only
//! consider bits in defined flags. Adding an unnamed flag makes those methods consider additional
//! bits, without generating additional constants for them. It helps compatibility when the external
//! source may start setting additional bits at any time. The
//! [known and unknown bits](#known-and-unknown-bits) section has more details on this behavior.
//!
//! ### Custom derives
//! You can derive some traits on generated flags types if you enable Cargo features. The following
//! libraries are currently supported:
//!
//! - `serde`: Support `#[derive(Serialize, Deserialize)]`, using text for human-readable formats,
//!   and a raw number for binary formats.
//! - `arbitrary`: Support `#[derive(Arbitrary)]`, only generating flags values with known bits.
//! - `bytemuck`: Support `#[derive(Pod, Zeroable)]`, for casting between flags values and their
//!   underlying bits values.
//!
//! ### Adding custom methods
//!
//! The [`bitflag`] macro supports any attributes on generated flags types within the macro itself,
//! while `impl` blocks can be added normally:
//!
//! ```rust
//! # use bitflag_attr::bitflag;
//! #
//! #[bitflag(u32)]
//! // Attributes can be applied to flags types
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//! enum Flags {
//!     A = 0b00000001,
//!     B = 0b00000010,
//!     C = 0b00000100
//! }
//!
//! // Impl blocks can be added to flags types normally
//! impl Flags {
//!     pub fn as_u64(&self) -> u64 {
//!         self.bits() as u64
//!     }
//! }
//! ```
//!
//! ## Working with flags values
//!
//! Use generated constants and standard bitwise operators to interact with flags values:
//!
//! ```rust
//! # use bitflag_attr::bitflag;
//! # #[bitflag(u32)]
//! # #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//! # enum Flags {
//! #     A = 0b00000001,
//! #     B = 0b00000010,
//! #     C = 0b00000100
//! # }
//! #
//! // union
//! let ab = Flags::A | Flags::B;
//!
//! // intersection
//! let a = ab & Flags::A;
//!
//! // difference
//! let b = ab - Flags::A;
//!
//! // complement
//! let c = !ab;
//! ```
//!
//! See the docs for the [`example_generated`] module and the [`Flags`] trait for more details on
//! operators and how they behave.
//!
//! # Formatting and parsing
//!
//! `bitflags` defines a text format that can be used to convert any flags value to and from strings.
//!
//! See the [`parser`] module for more details.
//!
//! # Terminology
//!
//! This crate and its documentation tries to follow the same terminology of the `bitflags` crate
//! (the OG). Here we define some.
//!
//! ## Flags types, flags values, flags
//!
//! Some terminology to refer to things in the bitflags domain:
//!
//! - **Bits type**: A type that defines a fixed number of bits at specific locations.
//! - **Flag**: A set of bits in a bits type that may have a unique name.
//! - **Flags type**: A set of defined flags over a specific bits type.
//! - **Flags value**: An instance of a flags type using its specific bits value for storage.
//!
//! ```rust
//! # use bitflag_attr::bitflag;
//! #
//! #[bitflag(u8)]
//! //        -- Bits type
//! #[derive(Clone, Copy)]
//! enum FlagsType {
//! //   --------- Flags type
//!     A = 1
//! //  ----- Flag
//! }
//!
//! let flag = FlagsType::A;
//! //  ---- Flags value
//! ```
//!
//! ## Known and unknown bits
//!
//! Any bits in a flag you define are called _known bits_. Any other bits are _unknown bits_. In the
//! following flags type:
//!
//! ```rust
//! # use bitflag_attr::bitflag;
//! #[bitflag(u8)]
//! #[derive(Clone, Copy)]
//! enum Flags {
//!     A = 1,
//!     B = 1 << 1,
//!     C = 1 << 2,
//! }
//! ```
//!
//! The known bits are `0b0000_0111` and the unknown bits are `0b1111_1000`.
//!
//! `bitflag_attr` doesn't guarantee that a flags value will only ever have known bits set, but some
//! operators will unset any unknown bits they encounter.
//!
//! If you're using `bitflags` for flags types defined externally, such as from C, you probably want
//! all bits to be considered known, in case that external source changes. You can do this using an
//! unnamed flag, as described in [externally defined flags](#externally-defined-flags).
//!
//! ## Zero-bit flags
//!
//! Flags with no bits set, in general, should be avoided because they interact strangely with
//! [`contains`] and [`intersects`]. A zero-bit flag is always contained, but is never intersected. The
//! names of zero-bit flags can be parsed, but are never formatted.
//!
//! [`contains`]: Flags::contains
//! [`intersects`]: Flags::intersects
//!
//! ## Multi-bit flags
//!
//! Flags that set multiple bits should be avoided unless each bit is also in a single-bit flag.
//! Take the following flags type as an example:
//!
//! ```rust
//! # use bitflag_attr::bitflag;
//! #[bitflag(u8)]
//! #[derive(Clone, Copy)]
//! enum Flags {
//!     A = 1,
//!     B = 1 | (1 << 1),
//! }
//! ```
//!
//! The result of `Flags::A ^ Flags::B` is `0b0000_0010`, which doesn't correspond to either
//! `Flags::A` or `Flags::B` even though it's still a known bit.
//!
//! [`example_generated`]: crate::example_generated::ExampleFlags
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

use core::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
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
    + BitAndAssign
    + BitOrAssign
    + BitXorAssign
    + fmt::Binary
    + fmt::LowerHex
    + fmt::UpperHex
    + fmt::Octal
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
                #[inline]
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
/// This trait is implemented by the [`bitflag`] macro:
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
    #[inline]
    fn from_bits(bits: Self::Bits) -> Option<Self> {
        let truncated = Self::from_bits_truncate(bits);

        if truncated.bits() == bits {
            Some(truncated)
        } else {
            None
        }
    }

    /// Convert from `bits` value, unsetting any unknown bits.
    #[inline]
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
    #[inline]
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
    #[inline]
    fn empty() -> Self {
        Self::from_bits_retain(Self::Bits::EMPTY)
    }

    /// Returns `true` if the flag value has all bits unset.
    #[inline]
    fn is_empty(&self) -> bool {
        self.bits() == Self::Bits::EMPTY
    }

    /// Returns a flag value that contains all value.
    ///
    /// This will include bits that do not have any flags/meaning.
    /// Use [`all`](Flags::all) if you want only the specified flags set.
    #[inline]
    fn all_bits() -> Self {
        Self::from_bits_retain(Self::Bits::ALL)
    }

    /// Returns `true` if the bitflag contains all value bits set.
    ///
    /// This will check for all bits.
    /// Use [`is_all`](Flags::is_all) if you want to check for all specified flags.
    #[inline]
    fn is_all_bits(&self) -> bool {
        self.bits() == Self::Bits::ALL
    }

    /// Construct a flag value with all known flags set.
    ///
    /// This will only set the flags specified as associated constant.
    #[inline]
    fn all() -> Self {
        let mut truncated = Self::Bits::EMPTY;

        for (_, flag) in Self::KNOWN_FLAGS.iter() {
            truncated |= flag.bits();
        }

        truncated |= Self::EXTRA_VALID_BITS;

        Self::from_bits_retain(truncated)
    }

    /// Whether all known bits in this flags value are set.
    #[inline]
    fn is_all(&self) -> bool {
        // NOTE: We check against `Self::all` here, not `Self::Bits::ALL`
        // because the set of all flags may not use all bits
        Self::all().bits() | self.bits() == self.bits()
    }

    /// Returns `true` if there are any unknown bits set in the flag value.
    #[inline]
    fn contains_unknown_bits(&self) -> bool {
        Self::all().bits() & self.bits() != self.bits()
    }

    /// Returns a bit flag that only has bits corresponding to the specified flags as associated constant.
    #[inline]
    fn truncated(&self) -> Self {
        Self::from_bits_retain(self.bits() & Self::all().bits())
    }

    /// Returns `true` if this flag value intersects with any value in `other`.
    ///
    /// This is equivalent to `(self & other) != Self::empty()`
    #[inline]
    fn intersects(&self, other: Self) -> bool
    where
        Self: Sized,
    {
        self.bits() & other.bits() != Self::Bits::EMPTY
    }

    /// Returns `true` if this flag value contains all values of `other`.
    ///
    /// This is equivalent to `(self & other) == other`
    #[inline]
    fn contains(&self, other: Self) -> bool
    where
        Self: Sized,
    {
        self.bits() & other.bits() == other.bits()
    }

    /// Remove any unknown bits from the flags.
    #[inline]
    fn truncate(&mut self)
    where
        Self: Sized,
    {
        *self = Self::from_bits_truncate(self.bits());
    }

    /// Returns the intersection from this value with `other`.
    #[must_use]
    #[inline]
    #[doc(alias = "and")]
    fn intersection(self, other: Self) -> Self {
        Self::from_bits_retain(self.bits() & other.bits())
    }

    /// Returns the union from this value with `other`.
    #[must_use]
    #[inline]
    #[doc(alias = "or")]
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
    #[inline]
    fn difference(self, other: Self) -> Self {
        Self::from_bits_retain(self.bits() & !other.bits())
    }

    /// Returns the symmetric difference from this value with `other`..
    #[must_use]
    #[inline]
    #[doc(alias = "xor")]
    fn symmetric_difference(self, other: Self) -> Self {
        Self::from_bits_retain(self.bits() ^ other.bits())
    }

    /// Returns the complement of the value.
    ///
    /// This is very similar to the `not` operation, but truncates non used bits.
    #[must_use]
    #[inline]
    #[doc(alias = "not")]
    fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }

    /// Set the flags in `other` in the value.
    #[inline]
    #[doc(alias = "insert")]
    fn set(&mut self, other: Self)
    where
        Self: Sized,
    {
        *self = Self::from_bits_retain(self.bits()).union(other);
    }

    /// Unset the flags bits in `other` in the value.
    ///
    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
    /// `remove` won't truncate `other`, but the `!` operator will.
    #[inline]
    #[doc(alias = "remove")]
    fn unset(&mut self, other: Self)
    where
        Self: Sized,
    {
        *self = Self::from_bits_retain(self.bits()).difference(other);
    }

    /// Toggle the flags in `other` in the value.
    #[inline]
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
    #[inline]
    fn iter(&self) -> iter::Iter<Self> {
        iter::Iter::new(self)
    }

    /// Yield a set of contained named flags values.
    ///
    /// This method is like [`Flags::iter`], except only yields bits in contained named flags.
    /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
    #[inline]
    fn iter_names(&self) -> iter::IterNames<Self> {
        iter::IterNames::new(self)
    }
}

///////////////////////////////////////////////////////////////////////////////
// Adapted from bitflags `bitflags_match!`
///////////////////////////////////////////////////////////////////////////////

/// A macro that matches flags values, similar to Rust's `match` statement.
///
/// In a regular `match` statement, the syntax `Flag::A | Flag::B` is interpreted as an or-pattern,
/// instead of the bitwise-or of `Flag::A` and `Flag::B`. This can be surprising when combined with flags types
/// because `Flag::A | Flag::B` won't match the pattern `Flag::A | Flag::B`. This macro is an alternative to
/// `match` for flags values that doesn't have this issue.
///
/// # Syntax
///
/// ```ignore
/// bitflag_match!(expression, {
///     pattern1 => result1,
///     pattern2 => result2,
///     ..
///     _ => default_result,
/// })
/// ```
///
/// The final `_ => default_result` arm is required, otherwise the macro will fail to compile.
///
/// # Examples
///
/// ```rust
/// use bitflag_attr::{bitflag, bitflag_match};
///
/// #[bitflag(u8)]
/// #[derive(Clone, Copy, PartialEq)]
/// enum Flags {
///     A = 1 << 0,
///     B = 1 << 1,
///     C = 1 << 2,
/// }
///
/// let flags = Flags::A | Flags::B;
///
/// bitflag_match!(flags, {
///     Flags::A | Flags::B => println!("A and/or B are set"),
///     _ => println!("neither A nor B are set"),
/// })
/// ```
///
/// # How it works
///
/// The macro expands to a series of `if` statements, checking equality between the input expression
/// and each pattern. This allows for correct matching of bitflag combinations, which is not possible
/// with a regular match expression due to the way bitflags are implemented.
///
/// Patterns are evaluated in order.
#[macro_export]
macro_rules! bitflag_match {
    ($operation:expr, {
        $($t:tt)*
    }) => {
        // Expand to a closure so we can use `return`
        // This makes it possible to apply attributes to the "match arms"
        (|| {
            $crate::__bitflag_match!($operation, { $($t)* })
        })()
    };
}

/// Expand the `bitflags_match` macro
#[macro_export]
#[doc(hidden)]
macro_rules! __bitflag_match {
    // Eat an optional `,` following a block match arm
    ($operation:expr, { $pattern:expr => { $($body:tt)* } , $($t:tt)+ }) => {
        $crate::__bitflag_match!($operation, { $pattern => { $($body)* } $($t)+ })
    };
    // Expand a block match arm `A => { .. }`
    ($operation:expr, { $pattern:expr => { $($body:tt)* } $($t:tt)+ }) => {
        {
            if $operation == $pattern {
                return {
                    $($body)*
                };
            }

            $crate::__bitflag_match!($operation, { $($t)+ })
        }
    };
    // Expand an expression match arm `A => x,`
    ($operation:expr, { $pattern:expr => $body:expr , $($t:tt)+ }) => {
        {
            if $operation == $pattern {
                return $body;
            }

            $crate::__bitflag_match!($operation, { $($t)+ })
        }
    };
    // Expand the default case
    ($operation:expr, { _ => $default:expr $(,)? }) => {
        $default
    }
}

#[cfg(doc)]
pub mod example_generated;
