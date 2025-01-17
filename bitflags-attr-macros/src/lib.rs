use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Error, Result};
use typed::{Args, Bitflag};

mod typed;

/// An attribute macro that transforms an C-like enum into a bitflag struct type implementing an API
/// similar to the `bitflags` crate, and implementing many helpful traits (listed in more details
/// below).
///
/// The attribute requires that the [`Clone`] and [`Copy`] traits are derived for the type.
///
/// ## Examples
///
/// Generate a flags type using `u8` as the bits type:
/// ```rust
/// # use bitflag_attr::bitflag;
///
/// #[bitflag(u8)]
/// #[derive(Clone, Copy)]
/// enum Flags {
///     A = 1,
///     B = 1 << 1,
///     C = 0b0000_0100,
/// }
/// ```
///
/// Flags may refer to other flags using their names:
///
/// ```rust
/// # use bitflag_attr::bitflag;
///
/// #[bitflag(u8)]
/// #[derive(Clone, Copy)]
/// enum Flags {
///     A = 1,
///     B = 1 << 1,
///     C = 0b0000_0100,
///     AB = A | B,
/// }
/// ```
///
/// Flags may also refer to other flags using their `bits` method value, like `bitflags` crate:
///
/// ```rust
/// # use bitflag_attr::bitflag;
///
/// #[bitflag(u8)]
/// #[derive(Clone, Copy)]
/// enum Flags {
///     A = 1,
///     B = 1 << 1,
///     C = 0b0000_0100,
///     AB = Flags::A.bits() | Flags::B.bits(),
/// }
/// ```
///
/// It's possible to use more derives and attributes by simply adding them
///
/// ```rust
/// # use core::fmt::Debug as ExternalDerive
///
/// #[bitflag(u8)]
/// #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ExternalDerive)]
/// enum Flags {
///     A = 1,
///     B = 1 << 1,
///     C = 0b0000_0100,
///     AB = Flags::A.bits() | Flags::B.bits(),
/// }
/// ```
///
/// ## Known and unknown flags
///
/// The variant of the enum are flags. They will be expanded to type-associated constants. Every
/// variant value is a known flag, while every not specified value is a unknown flag.
///
/// There are operation that will truncate out the unknown values. But tha can be configured if
/// desired; more on that on [Externally defined flags](#externally-defined-flags)
///
/// ## Externally defined flags
///
/// If you're generating flags types for an external source, such as a C API, you can use the
/// `non_exhaustive` attribute to communicate to the bitflags macro that there may be more valid
/// flags then the known flags.
///
/// Without extra configuration, it defaults to `!0` (all bits set) as a mask of all bits the
/// external source may ever set, i.e. all bits are considered as possible values.
///
/// ```
/// use bitflag_attr::bitflag;
///
/// #[bitflag(u32)]
/// #[non_exhaustive] // All bits are considered as possible values.
/// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
/// pub enum Flags {
///     /// The value `A`, at bit position `0`.
///     A = 0b00000001,
///     /// The value `B`, at bit position `1`.
///     B = 0b00000010,
///     /// The value `C`, at bit position `2`.
///     C = 0b00000100,
///
///     /// The combination of `A`, `B`, and `C`.
///     ABC = A | B | C,
/// }
/// ```
///
/// But you can also configure it using the helper attribute `extra_valid_bits` with the value of
/// valid bits that the external source may ever set.
///
/// ```
/// use bitflag_attr::bitflag;
///
/// #[bitflag(u32)]
/// #[non_exhaustive] // Communicate there is more potential valid flags than the known flags
/// #[extra_valid_bits = 0b001001111] // Specify the extra bits to take into consideration.
/// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
/// pub enum Flags {
///     /// The value `A`, at bit position `0`.
///     A = 0b00000001,
///     /// The value `B`, at bit position `1`.
///     B = 0b00000010,
///     /// The value `C`, at bit position `2`.
///     C = 0b00000100,
///
///     /// The combination of `A`, `B`, and `C`.
///     ABC = A | B | C,
/// }
/// ```
///
/// Why should you do this? Generated methods like `all` and truncating operators like `!` only
/// consider bits in defined flags. Adding an unnamed flag makes those methods consider additional
/// bits, without generating additional constants for them. It helps compatibility when the external
/// source may start setting additional bits at any time.
///
/// ## Type representation
///
/// By default, the generated flag type will be `#[repr(transparent)]`, but you can explicit it on
/// the definition as long is one of the supported ones (`C`, `Rust` and `transparent`):
///
/// ```rust
/// # use bitflag_attr::bitflag;
///
/// #[repr(C)]
/// #[bitflag(u8)]
/// #[derive(Clone, Copy)]
/// enum Flags {
///     A = 1,
///     B = 1 << 1,
///     C = 1 << 2,
/// }
/// ```
///
/// ## Generated trait implementations
///
/// This macro generates some trait implementations: [`ops:Not`], [`ops:BitAnd`],
/// [`ops:BitOr`], [`ops:BitXor`], [`ops:BitAndAssign`], [`ops:BitOrAssign`], [`ops:BitXorAssign`],
/// [`fmt::Binary`], [`fmt::LowerHex`], [`fmt::UpperHex`], [`fmt::Octal`], [`From`], [`Extend`],
/// [`FromIterator`], [`FromStr`] and [`IntoIterator`].
///
/// The custom [`fmt::Debug`] implementation will only be generated if it is included in the
/// `#[derive(...)]` parameters.
///
/// ### Serde feature
///
/// If the crate is compiled with the `serde` feature, this crate will generate implementations for
/// the `serde::{Serialize, Deserialize}` traits if they are included in the `#[derive(...)]`
/// parameters, but it will not import/re-export these traits, your project must have `serde` as
/// dependency.
///
/// #### Example
/// ```no_run
/// use bitflag_attr::bitflag;
/// use serde::{Serialize, Deserialize}
///
/// #[bitflag(u32)]
/// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// pub enum Flags {
///     /// The value `A`, at bit position `0`.
///     A = 0b00000001,
///     /// The value `B`, at bit position `1`.
///     B = 0b00000010,
///     /// The value `C`, at bit position `2`.
///     C = 0b00000100,
///
///     /// The combination of `A`, `B`, and `C`.
///     ABC = A | B | C,
/// }
/// ```
///
/// ### `const-mut-ref` feature
///
/// If the crate is compiled with the `const-mut-ref` feature, all type-associated API that takes
/// `&mut self` will be generated as **const-fn**, meaning they can be used on `const` context.
///
/// **Note:** `&mut` on const function was stabilized on Rust 1.83.0, so using this feature flag on
/// Rust versions below that will cause compilation errors
///
/// ### Custom types feature
///
/// If the crate is compiled with the `custom-types` feature, it allows to use more than the types
/// defined in Rust `core` (`i8`,`u8`,`i16`,`u16`,`i32`,`u32`,`i64`,`u64`,`i128`,`u128`,`isize`,
/// `usize`,`c_char`,`c_schar`,`c_uchar`,`c_short`,`c_ushort`,`c_int`,`c_uint`,`c_long`,`c_ulong`,
/// `c_longlong`,`c_ulonglong`) as long as it is a type alias to one of those types.
///
/// The reason it is behind a feature flag is that to ensure the validity of such constrain, we have
/// to pay the price of having much worse error messages. With this feature enabled, a invalid type
/// will cause a massive wall of error message.
///
///
/// # More Examples
///
/// ```
/// use bitflag_attr::bitflag;
///
/// #[bitflag(u32)]
/// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
/// pub enum Flags {
///     /// The value `A`, at bit position `0`.
///     A = 0b00000001,
///     /// The value `B`, at bit position `1`.
///     B = 0b00000010,
///     /// The value `C`, at bit position `2`.
///     C = 0b00000100,
///
///     /// The combination of `A`, `B`, and `C`.
///     ABC = A | B | C,
/// }
/// ```
///
/// Without generating [`fmt::Debug`]:
///
/// ```
/// use bitflag_attr::bitflag;
///
/// #[bitflag(u32)]
/// #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
/// pub enum Flags {
///     /// The value `A`, at bit position `0`.
///     A = 0b00000001,
///     /// The value `B`, at bit position `1`.
///     B = 0b00000010,
///     /// The value `C`, at bit position `2`.
///     C = 0b00000100,
///
///     /// The combination of `A`, `B`, and `C`.
///     ABC = A | B | C,
/// }
/// ```
///
/// # Syntax
///
/// ```text
/// #[bitflag($ty)]
/// $visibility enum $StructName {
///     FlagOne = flag1_value_expr,
///     FlagTwo = flag2_value_expr,
///     // ...
///     FlagN = flagn_value_expr,
/// }
/// ```
///
/// [`fmt::Debug`]: core::fmt::Debug
/// [`ops:Not`]: core::ops::Not
/// [`ops:BitAnd`]: core::ops::BitAnd
/// [`ops:BitOr`]: core::ops::BitOr
/// [`ops:BitXor`]: core::ops::BitXor
/// [`ops:BitAndAssign`]: core::ops::BitAndAssign
/// [`ops:BitOrAssign`]: core::ops::BitOrAssign
/// [`ops:BitXorAssign`]: core::ops::BitXorAssign
/// [`fmt::Binary`]: core::fmt::Binary
/// [`fmt::LowerHex`]: core::fmt::LowerHex
/// [`fmt::UpperHex`]: core::fmt::UpperHex
/// [`fmt::Octal`]: core::fmt::Octal
/// [`From`]: From
/// [`FromStr`]: core::str::FromStr
#[proc_macro_attribute]
pub fn bitflag(attr: TokenStream, item: TokenStream) -> TokenStream {
    match bitflag_impl(attr, item) {
        Ok(ts) => ts,
        Err(err) => err.into_compile_error().into(),
    }
}

fn bitflag_impl(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let args: Args = syn::parse(attr)
        .map_err(|err| Error::new(err.span(), "unexpected token: expected a `{integer}` type"))?;

    let bitflag = Bitflag::parse(args, item)?;

    Ok(bitflag.to_token_stream().into())
}
