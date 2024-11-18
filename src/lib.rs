#![doc = include_str!("../README.md")]

use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, Error, Ident, ItemEnum, Result, Token};

/// An attribute macro that transforms an C-like enum into a bitflag struct implementing an type API
/// similar to the `bitflags` crate, and implementing traits as listed below.
///
/// # Generated trait implementations
/// This macro generates some trait implementations: [`fmt::Debug`], [`ops:Not`], [`ops:BitAnd`],
/// [`ops:BitOr`], [`ops:BitXor`], [`ops:BitAndAssign`], [`ops:BitOrAssign`], [`ops:BitXorAssign`],
/// [`fmt::Binary`], [`fmt::LowerHex`], [`fmt::UpperHex`], [`fmt::Octal`], [`From`], [`Clone`],
/// [`Copy`]
///
/// If the macro receives `no_auto_debug`, the trait [`fmt::Debug`] will not be generated. Use this
/// flag when you want to implement [`fmt::Debug`] manually or use the standard derive.
///
/// # Example
///
/// ```
/// use bitflag_attr::bitflag;
///
/// #[bitflag(u32)]
/// #[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
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
/// #[bitflag(u32, no_auto_debug)]
/// #[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
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
/// #[bitflag($ty[, no_auto_debug])]
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
#[proc_macro_attribute]
pub fn bitflag(attr: TokenStream, item: TokenStream) -> TokenStream {
    match bitflag_impl(attr, item) {
        Ok(ts) => ts,
        Err(err) => err.into_compile_error().into(),
    }
}

fn bitflag_impl(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let args: Args = syn::parse(attr)?;
    let ty = args.ty;
    // let ty = parse_ty(attr)?;

    let item: ItemEnum = syn::parse(item)?;

    let vis = item.vis;
    let attrs = item.attrs;
    let ty_name = item.ident;

    let iter_name_ty = {
        let span = ty_name.span();
        let mut ty_name = ty_name.to_string();
        ty_name.push_str("IterNames");
        Ident::new(&ty_name, span)
    };
    let iter_ty = {
        let span = ty_name.span();
        let mut ty_name = ty_name.to_string();
        ty_name.push_str("Iter");
        Ident::new(&ty_name, span)
    };

    let number_flags = item.variants.len();

    let mut all_flags = Vec::with_capacity(number_flags);
    let mut all_flags_names = Vec::with_capacity(number_flags);
    let mut all_variants = Vec::with_capacity(number_flags);

    // The raw flags as private itens to allow defining flags referencing other flag definitions
    let mut raw_flags = Vec::with_capacity(number_flags);

    let mut flags = Vec::with_capacity(number_flags); // Associated constants

    // First generate the raw_flags
    for variant in item.variants.iter() {
        let var_attrs = &variant.attrs;
        let var_name = &variant.ident;

        let expr = match variant.discriminant.as_ref() {
            Some((_, expr)) => expr,
            None => {
                return Err(Error::new_spanned(
                    variant,
                    "a discriminant must be defined",
                ))
            }
        };

        raw_flags.push(quote! {
            #(#var_attrs)*
            #[allow(non_upper_case_globals, dead_code, unused)]
            const #var_name: #ty = #expr;
        });
    }

    for variant in item.variants.iter() {
        let var_attrs = &variant.attrs;
        let var_name = &variant.ident;

        let expr = match variant.discriminant.as_ref() {
            Some((_, expr)) => expr,
            None => {
                return Err(Error::new_spanned(
                    variant,
                    "a discriminant must be defined",
                ))
            }
        };

        all_flags.push(quote!(Self::#var_name));
        // all_flags_names.push(quote!(stringify!(#var_name)));
        all_flags_names.push(syn::LitStr::new(&var_name.to_string(), var_name.span()));
        all_variants.push(quote!(#var_name));

        flags.push(quote! {
            #(#var_attrs)*
            #vis const #var_name: Self = {
                #(#raw_flags)*

                Self(#expr)
            };
        });
    }

    let debug_impl = if args.no_auto_debug {
        quote! {}
    } else {
        quote! {
            #[automatically_derived]
            impl ::core::fmt::Debug for #ty_name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    struct HumanReadable<'a>(&'a #ty_name);

                    impl<'a> ::core::fmt::Debug for HumanReadable<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            self.0.to_writer(f)
                        }
                    }

                    let name = ::core::stringify!(#ty_name);

                    f.debug_struct(name)
                        .field("bits", &::core::format_args!("{:#b}", self.0))
                        .field("human_readable", &HumanReadable(self))
                        .finish()
                }
            }
        }
    };

    let serde_impl = if cfg!(feature = "serde") {
        let parser_error_ty = {
            let span = ty_name.span();
            let mut ty = ty_name.to_string();
            ty.push_str("ParserError");
            Ident::new(&ty, span)
        };
        quote! {
            #[automatically_derived]
            impl ::serde::Serialize for #ty_name {
                fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer
                {
                    struct AsDisplay<'a>(&'a #ty_name);

                    impl<'a> ::core::fmt::Display for AsDisplay<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            self.0.to_writer(f)
                        }
                    }

                    // Serialize human-readable flags as a string like `"A | B"`
                    if serializer.is_human_readable() {
                        serializer.collect_str(&AsDisplay(self))
                    }
                    // Serialize non-human-readable flags directly as the underlying bits
                    else {
                        self.bits().serialize(serializer)
                    }
                }
            }

            #[automatically_derived]
            impl<'de> ::serde::Deserialize<'de> for #ty_name {
                fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>
                {
                    if deserializer.is_human_readable() {
                        struct HelperVisitor(::core::marker::PhantomData<#ty_name>);

                        impl<'de> ::serde::de::Visitor<'de> for HelperVisitor {
                            type Value = #ty_name;

                            fn expecting(&self,  f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                f.write_str("a string value of `|` separated flags")
                            }

                            fn visit_str<E>(self, flags: &str) -> ::core::result::Result<Self::Value, E>
                            where
                                E: ::serde::de::Error,
                            {
                                Self::Value::from_text(flags).map_err(|e| E::custom(e))
                            }
                        }

                        deserializer.deserialize_str(HelperVisitor(::core::marker::PhantomData))
                    } else {
                        let bits = #ty::deserialize(deserializer)?;

                        Ok(#ty_name::from_bits_retain(bits))
                    }
                }
            }

            #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
            pub enum #parser_error_ty {
                EmptyFlag,
                InvalidNamedFlag,
                InvalidHexFlag,
            }

            #[automatically_derived]
            impl ::core::error::Error for #parser_error_ty {}

            #[automatically_derived]
            impl ::core::fmt::Display for #parser_error_ty {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        Self::EmptyFlag => write!(f, "encountered empty flag"),
                        Self::InvalidNamedFlag => write!(f, "unrecognized named flag"),
                        Self::InvalidHexFlag => write!(f, "invalid hex flag"),
                    }
                }
            }

            impl #ty_name {
                /// Helper to parse flags from human readable format. Parse a flags value from text.
                ///
                /// This function will fail on any names that don't correspond to defined flags.
                /// Unknown bits will be retained.
                pub(crate) fn from_text(input: &str) -> ::core::result::Result<Self, #parser_error_ty> {
                    let mut parsed_flags = Self::empty();

                    // If the input is empty, then return an empty set of flags
                    if input.trim().is_empty() {
                        return Ok(parsed_flags);
                    }

                    for flag in input.split('|') {
                        let flag = flag.trim();

                        // If the flag is empty, then we've got a missing input
                        if flag.is_empty() {
                            return Err(#parser_error_ty::EmptyFlag);
                        }

                        // If the flag starts with `0x` ten it's a hex number
                        // Parse it directly to the underlying bits
                        let parsed_flag =  if let Some(flag) = flag.strip_prefix("0x") {
                            let bits = #ty::from_str_radix(flag, 16).map_err(|_| #parser_error_ty::InvalidHexFlag)?;

                            Self::from_bits_retain(bits)
                        } else {
                            // Otherwise, the flag is a name
                            // The generated flags type will determine whether or not it is a valid
                            // identifier
                            Self::from_flag_name(flag).ok_or_else(|| #parser_error_ty::InvalidNamedFlag)?
                        };

                        parsed_flags.set(parsed_flag);
                    }

                    Ok(parsed_flags)
                }

                /// Helper to parse flags from human readable format. Parse a flags value from text.
                ///
                /// This function will fail on any names that don't correspond to defined flags.
                /// Unknown bits will be ignored.
                pub(crate) fn from_text_truncate(input: &str) -> ::core::result::Result<Self, #parser_error_ty> {
                    Ok(Self::from_text(input)?.truncate())
                }

                /// Helper to parse flags from human readable format. Parse a flags value from text.
                ///
                /// This function will fail on any names that don't correspond to defined flags.
                /// This function will fail to parse hex values.
                pub(crate) fn from_text_strict(input: &str) -> ::core::result::Result<Self, #parser_error_ty> {
                    let mut parsed_flags = Self::empty();

                    // If the input is empty, then return an empty set of flags
                    if input.trim().is_empty() {
                        return Ok(parsed_flags);
                    }

                    for flag in input.split('|') {
                        let flag =  flag.trim();

                        // If the flag is empty, then we've got a missing input
                        if flag.is_empty() {
                            return Err(#parser_error_ty::EmptyFlag);
                        }

                        // If the flag starts with `0x` then it is a hex number
                        // There aren't supported in the strict parser
                        if flag.starts_with("0x") {
                            return Err(#parser_error_ty::InvalidHexFlag);
                        }

                        let parsed_flag = Self::from_flag_name(flag).ok_or_else(|| #parser_error_ty::InvalidNamedFlag)?;

                        parsed_flags.set(parsed_flag);
                    }

                    Ok(parsed_flags)
                }
            }

            #[automatically_derived]
            impl ::core::str::FromStr for #ty_name {
                type Err = #parser_error_ty;

                fn from_str(input: &str) -> ::core::result::Result<Self, Self::Err> {
                    Self::from_text(input)
                }
            }
        }
    } else {
        quote!()
    };

    let generated = quote! {
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        #(#attrs)*
        #vis struct #ty_name(#ty);

        #[allow(non_upper_case_globals)]
        impl #ty_name {
            #[doc(hidden)]
            #[allow(clippy::unused_unit)]
            const __OG: () = {
                {
                    // Original enum
                    // This is a hack to make LSP coloring to still sees the original enum variant as a Enum variant.
                    enum Original {
                        #(#all_variants, )*
                    }
                }
                ()
            };

            #(#flags)*

            /// Return the underlying bits of the bitflag
            #[inline]
            pub const fn bits(&self) -> #ty {
                self.0
            }

            /// Converts from a `bits` value. Returning [`None`] is any unknown bits are set.
            #[inline]
            pub const fn from_bits(bits: #ty) -> Option<Self> {
                let truncated = Self::from_bits_truncate(bits).0;

                if truncated == bits {
                    Some(Self(bits))
                } else {
                    None
                }
            }

            /// Convert from `bits` value, unsetting any unknown bits.
            #[inline]
            pub const fn from_bits_truncate(bits: #ty) -> Self {
                Self(bits & Self::all().0)
            }

            /// Convert from `bits` value exactly.
            #[inline]
            pub const fn from_bits_retain(bits: #ty) -> Self {
                Self(bits)
            }

            /// Convert from a flag `name`.
            #[inline]
            pub fn from_flag_name(name: &str) -> Option<Self> {
                match name {
                    #(#all_flags_names => Some(#all_flags),)*
                    _ => None
                }
            }

            /// Construct an empty bitflag.
            #[inline]
            pub const fn empty() -> Self {
                Self(0)
            }

            /// Returns `true` if the flag is empty.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.0 == 0
            }

            /// Returns a bitflag that contains all value.
            ///
            /// This will include bits that do not have any flags/meaning.
            /// Use [`all`](Self::all) if you want only the specified flags set.
            #[inline]
            pub const fn all_bits() -> Self {
                Self(!0)
            }

            /// Returns `true` if the bitflag contains all value bits set.
            ///
            /// This will check for all bits.
            /// Use [`is_all`](Self::is_all) if you want to check for all specified flags.
            #[inline]
            pub const fn is_all_bits(&self) -> bool {
                self.0 == !0
            }

            /// Construct a bitflag with all known flags set.
            ///
            /// This will only set the flags specified as associated constant.
            #[inline]
            pub const fn all() -> Self {
                Self(#(#all_flags.0 |)* 0)
            }

            /// Returns `true` if the bitflag contais all known flags.
            ///
            #[inline]
            pub const fn is_all(&self) -> bool {
                self.0 == Self::all().0
            }

            /// Returns a bit flag that only has bits corresponding to the specified flags as associated constant.
            #[inline]
            pub const fn truncate(&self) -> Self {
                Self(self.0 & Self::all().0)
            }

            /// Returns `true` if this bitflag intersects with any value in `other`.
            ///
            /// This is equivalent to `(self & other) != Self::empty()`
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                (self.0 & other.0) != Self::empty().0
            }

            /// Returns `true` if this bitflag contains all values of `other`.
            ///
            /// This is equivalent to `(self & other) == other`
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }

            /// Returns the bitwise NOT of the flag.
            ///
            /// This function does not truncate unused bits (bits that do not have any flags/meaning).
            /// Use [`complement`](Self::complement) if you want that the result to be truncated in one call.
            #[inline]
            #[doc(alias = "complement")]
            pub const fn not(self) -> Self {
                Self(!self.0)
            }

            /// Returns the bitwise AND of the flag.
            #[inline]
            #[doc(alias = "intersection")]
            pub const fn and(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }

            /// Returns the bitwise OR of the flag with `other`.
            #[inline]
            #[doc(alias = "union")]
            pub const fn or(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }

            /// Returns the bitwise XOR of the flag with `other`.
            #[inline]
            #[doc(alias = "symmetric_difference")]
            pub const fn xor(self, other: Self) -> Self {
                Self(self.0 ^ other.0)
            }

            /// Returns the intersection from this value with `other`.
            #[inline]
            #[doc(alias = "and")]
            pub const fn intersection(self, other: Self) -> Self {
                self.and(other)
            }

            /// Returns the union from this value with `other`
            #[inline]
            #[doc(alias = "or")]
            pub const fn union(self, other: Self) -> Self {
                self.or(other)
            }

            /// Returns the difference from this value with `other`.
            ///
            /// In other words, returns the intersection of this value with the negation of `other`.
            #[inline]
            pub const fn difference(self, other: Self) -> Self {
                self.and(other.not())
            }

            /// Returns the symmetric difference from this value with `other`.
            #[inline]
            #[doc(alias = "xor")]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                self.xor(other)
            }

            /// Returns the complement of the value.
            ///
            /// This is very similar to the [`not`](Self::not), but truncates non used bits
            #[inline]
            #[doc(alias = "not")]
            pub const fn complement(self) -> Self {
                self.not().truncate()
            }

            /// Set the flags in `other` in the value.
            #[inline]
            #[doc(alias = "insert")]
            pub fn set(&mut self, other: Self) {
                self.0 = self.or(other).0
            }

            /// Unset the flags in `other` in the value.
            #[inline]
            #[doc(alias = "remove")]
            pub fn unset(&mut self, other: Self) {
                self.0 = self.difference(other).0
            }

            /// Toggle the flags in `other` in the value.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                self.0 = self.xor(other).0
            }
        }

        #[automatically_derived]
        impl ::core::ops::Not for #ty_name {
            type Output = Self;

            #[inline]
            fn not(self) -> Self::Output {
                self.complement()
            }
        }

        #[automatically_derived]
        impl ::core::ops::BitAnd for #ty_name {
            type Output = Self;

            #[inline]
            fn bitand(self, rhs: Self) -> Self::Output {
                self.and(rhs)
            }
        }

        #[automatically_derived]
        impl ::core::ops::BitOr for #ty_name {
            type Output = Self;

            #[inline]
            fn bitor(self, rhs: Self) -> Self::Output {
                self.or(rhs)
            }
        }

        #[automatically_derived]
        impl ::core::ops::BitXor for #ty_name {
            type Output = Self;

            #[inline]
            fn bitxor(self, rhs: Self) -> Self::Output {
                self.xor(rhs)
            }
        }

        #[automatically_derived]
        impl ::core::ops::BitAndAssign for #ty_name {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                ::core::ops::BitAndAssign::bitand_assign(&mut self.0, rhs.0)
            }
        }

        #[automatically_derived]
        impl ::core::ops::BitOrAssign for #ty_name {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                ::core::ops::BitOrAssign::bitor_assign(&mut self.0, rhs.0)
            }
        }

        #[automatically_derived]
        impl ::core::ops::BitXorAssign for #ty_name {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self) {
                ::core::ops::BitXorAssign::bitxor_assign(&mut self.0, rhs.0)
            }
        }

        #[automatically_derived]
        impl ::core::ops::Sub for #ty_name {
            type Output = Self;

            /// The intersection of a source flag with the complement of a target flags value
            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                self.difference(rhs)
            }
        }

        #[automatically_derived]
        impl ::core::ops::SubAssign for #ty_name {
            /// The intersection of a source flag with the complement of a target flags value
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.unset(rhs)
            }
        }

        #[automatically_derived]
        impl ::core::convert::From<#ty> for #ty_name {
            #[inline]
            fn from(val: #ty) -> Self {
                Self::from_bits_truncate(val)
            }
        }

        #[automatically_derived]
        impl ::core::convert::From<#ty_name> for #ty {
            #[inline]
            fn from(val: #ty_name) -> Self {
                val.0
            }
        }

        #[automatically_derived]
        impl ::core::fmt::Binary for #ty_name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Binary::fmt(&self.0, f)
            }
        }

        #[automatically_derived]
        impl ::core::fmt::LowerHex for #ty_name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::LowerHex::fmt(&self.0, f)
            }
        }

        #[automatically_derived]
        impl ::core::fmt::UpperHex for #ty_name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::UpperHex::fmt(&self.0, f)
            }
        }

        #[automatically_derived]
        impl ::core::fmt::Octal for #ty_name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Octal::fmt(&self.0, f)
            }
        }

        #debug_impl

        impl #ty_name {
            const FLAGS: &[(&str, #ty_name)] = &[#((#all_flags_names , #all_flags) ,)*];

            /// Yield a set of contained flags values.
            ///
            /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
            /// will be yielded together as a final flags value.
            #[inline]
            pub const fn iter(&self) -> #iter_ty {
                #iter_ty::new(self)
            }

            /// Yield a set of contained named flags values.
            ///
            /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
            /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
            #[inline]
            pub const fn iter_names(&self) -> #iter_name_ty {
                #iter_name_ty::new(self)
            }

            /// Helper for formatting in human readable format. Write a flags value as text.
            ///
            /// Any bits that aren't part of a contained flag will be formatted as a hex number.
            pub(crate) fn to_writer<W>(&self, mut writer: W) -> ::core::fmt::Result
            where
                W: ::core::fmt::Write,
            {
                // A formatter for bitflags that produces text output like:
                //
                // A | B | 0xf6
                //
                // The names of set flags are written in a bar-separated-format,
                // followed by a hex number of any remaining bits that are set
                // but don't correspond to any flags.

                // Iterate over known flag values
                let mut first = true;
                let mut iter = self.iter_names();
                for (name, _) in &mut iter {
                    if !first {
                        writer.write_str(" | ")?;
                    }

                    first = false;
                    writer.write_str(name)?;
                }

                // Append any extra bits that correspond to flags to the end of the format
                let remaining = iter.remaining();
                if !remaining.is_empty() {
                    if !first {
                        writer.write_str(" | ")?;
                    }

                    ::core::write!(writer, "{:#X}", remaining.bits())?;
                }

                ::core::fmt::Result::Ok(())
            }

            /// Helper for formatting in human readable format. Write a flags value as text,
            /// ignoring any unknown bits.
            pub(crate) fn to_writer_truncate<W>(&self, mut writer: W) -> ::core::fmt::Result
            where
                W: ::core::fmt::Write
            {
                self.truncate().to_writer(writer)
            }

            /// Helper for formatting in human readable format. Write only the contained, defined,
            /// named flags in a flags value as text.
            pub(crate) fn to_writer_strict<W>(&self, mut writer: W) -> ::core::fmt::Result
            where
                W: ::core::fmt::Write
            {
                // This is a simplified version of `to_writer` that ignores
                // any bits not corresponding to a named flag

                let mut first = true;
                let mut iter = self.iter_names();
                for (name, _) in &mut iter {
                    if !first {
                        writer.write_str(" | ")?;
                    }

                    first = false;
                    writer.write_str(name)?;
                }

                ::core::fmt::Result::Ok(())
            }
        }

        #[automatically_derived]
        impl ::core::iter::Extend<#ty_name> for #ty_name {
            /// Set all flags of `iter` to self
            fn extend<T: ::core::iter::IntoIterator<Item = Self>>(&mut self, iter: T) {
                for item in iter {
                    self.set(item);
                }
            }
        }

        #[automatically_derived]
        impl ::core::iter::FromIterator<#ty_name> for #ty_name {
            /// Create a `#ty_name` from a iterator of flags.
            fn from_iter<T: ::core::iter::IntoIterator<Item = Self>>(iter: T) -> Self {
                use ::core::iter::Extend;

                let mut res = Self::empty();
                res.extend(iter);
                res
            }
        }

        #[automatically_derived]
        impl ::core::iter::IntoIterator for #ty_name {
            type Item = Self;
            type IntoIter = #iter_ty;

            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        #[automatically_derived]
        impl ::core::iter::IntoIterator for &#ty_name {
            type Item = #ty_name;
            type IntoIter = #iter_ty;

            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        /// An iterator over flags values.
        ///
        /// This iterator only yields flags values for contained, defined, named flags. Any remaining bits
        /// won't be yielded, but can be found with the [`#iter_name_ty::remaining`] method.
        #vis struct #iter_name_ty {
            flags: &'static [(&'static str, #ty_name)],
            index: usize,
            source: #ty_name,
            remaining: #ty_name,
        }

        impl #iter_name_ty {
            pub(crate) const fn new(flags: &#ty_name) -> Self {
                Self {
                    flags: #ty_name::FLAGS,
                    index: 0,
                    remaining: *flags,
                    source: *flags,
                }
            }

            /// Get a flags value of any remaining bits that haven't been yielded yet.
            ///
            /// Once the iterator has finished, this method can be used to
            /// check whether or not there are any bits that didn't correspond
            /// to a contained, defined, named flag remaining.
            pub const fn remaining(&self) -> #ty_name {
                self.remaining
            }
        }

        #[automatically_derived]
        impl ::core::iter::Iterator for #iter_name_ty {
            type Item = (&'static str, #ty_name);

            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                while let Some((name, flag)) = self.flags.get(self.index) {
                    // Short-circuit if our state is empty
                    if self.remaining.is_empty() {
                        return None;
                    }

                    self.index += 1;

                    // If the flag is set in the original source _and_ it has bits that haven't
                    // been covered by a previous flag yet then yield it. These conditions cover
                    // two cases for multi-bit flags:
                    //
                    // 1. When flags partially overlap, such as `0b00000001` and `0b00000101`, we'll
                    // yield both flags.
                    // 2. When flags fully overlap, such as in convenience flags that are a shorthand for others,
                    // we won't yield both flags.
                    if self.source.contains(*flag) && self.remaining.intersects(*flag) {
                        self.remaining.unset(*flag);

                        return Some((name, *flag))
                    }
                }

                None
            }
        }

        #[automatically_derived]
        impl ::core::iter::FusedIterator for #iter_name_ty {}

        /// An iterator over flags values.
        ///
        /// This iterator will yield flags values for contained, defined flags first, with any remaining bits yielded
        /// as a final flags value.
        #vis struct #iter_ty {
            inner: #iter_name_ty,
            done: bool,
        }

        impl #iter_ty {
            pub(crate) const fn new(flags: &#ty_name) -> Self  {
                Self {
                    inner: #iter_name_ty::new(flags),
                    done: false,
                }
            }
        }

        #[automatically_derived]
        impl ::core::iter::Iterator for #iter_ty {
            type Item = #ty_name;

            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                match self.inner.next() {
                    Some((_, flag)) => Some(flag),
                    None if !self.done => {
                        self.done = true;

                        // After iterating through valid names, if there are any bits left over
                        // then return one final value that includes them. This makes `into_iter`
                        // and `from_iter` roundtrip
                        if !self.inner.remaining().is_empty() {
                            Some(self.inner.remaining)
                        } else {
                            None
                        }
                    }
                    None => None
                }
            }
        }

        #[automatically_derived]
        impl ::core::iter::FusedIterator for #iter_ty {}

        #serde_impl
    };

    Ok(generated.into())
}

static VALID_TYPES: [&str; 23] = [
    "i8",
    "u8",
    "i16",
    "u16",
    "i32",
    "u32",
    "i64",
    "u64",
    "i128",
    "u128",
    "isize",
    "usize",
    "c_char",
    "c_schar",
    "c_uchar",
    "c_short",
    "c_ushort",
    "c_int",
    "c_uint",
    "c_long",
    "c_ulong",
    "c_longlong",
    "c_ulonglong",
];

struct Args {
    ty: Ident,
    no_auto_debug: bool,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let content: Punctuated<_, _> = input.parse_terminated(Ident::parse, Token![,])?;

        if content.empty_or_trailing() {
            return Ok(Args {
                ty: Ident::new("u32", Span::call_site().into()),
                no_auto_debug: false,
            });
        }

        if content.len() > 2 {
            return Err(Error::new_spanned(
                content.last().unwrap(),
                "more arguments than expected. Expected a max of one integer type and one `no_auto_debug` flag",
            ));
        }

        let mut no_debug_set = false;
        let mut ty_set = false;

        let mut no_auto_debug = false;
        let mut ty = Ident::new("u32", Span::call_site().into());

        for i in content {
            if i == "no_auto_debug" {
                if no_debug_set {
                    return Err(Error::new_spanned(
                        i,
                        "there must be only one instance of `no_auto_debug` flag",
                    ));
                }
                no_auto_debug = true;
                no_debug_set = true;
                continue;
            }
            if VALID_TYPES.contains(&i.to_string().as_str()) {
                if ty_set {
                    return Err(Error::new_spanned(
                        i,
                        "there must be only one instance of `{integer}` type specified",
                    ));
                }
                ty = i;
                ty_set = true;
                continue;
            } else {
                return Err(Error::new_spanned(i, "type must be a integer"));
            }
        }

        Ok(Args { ty, no_auto_debug })
    }
}

#[cfg(doc)]
mod example_generated;
