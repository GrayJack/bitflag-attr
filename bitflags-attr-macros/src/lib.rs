use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Error, Result};
use typed::{Args, Bitflag};

mod typed;

/// An attribute macro that transforms an C-like enum into a bitflag struct implementing an type API
/// similar to the `bitflags` crate, and implementing traits as listed below.
///
/// The attribute requires that the [`Clone`] and [`Copy`] traits are derived for the type.
///
/// # Generated trait implementations
/// This macro generates some trait implementations: [`ops:Not`], [`ops:BitAnd`],
/// [`ops:BitOr`], [`ops:BitXor`], [`ops:BitAndAssign`], [`ops:BitOrAssign`], [`ops:BitXorAssign`],
/// [`fmt::Binary`], [`fmt::LowerHex`], [`fmt::UpperHex`], [`fmt::Octal`], [`From`], [`Extend`], [`FromIterator`], [`IntoIterator`]
///
/// The custom [`fmt::Debug`] implementation will only be generated if it is included in the
/// `#[derive(...)]` parameters.
///
/// ## Serde feature
///
/// If the crate is compiled with the `serde` feature, this crate will generate implementations for
/// the `serde::{Serialize, Deserialize}` traits if they are included in the `#[derive(...)]`
/// parameters, but it will not import/re-export these traits, your project must have `serde` as
/// dependency.
///
/// Having this feature enabled will also generate a type to represent the parsing error and helper
/// functions to do parsing the generated type from strings. And will generate the implementation
/// for the [`FromStr`] trait.
///
/// ## Custom types
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
/// # Example
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
