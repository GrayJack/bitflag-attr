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

    let number_flags = item.variants.len();
    let num_flags = syn::LitInt::new(number_flags.to_string().as_str(), ty_name.span());

    let mut all_flags = Vec::with_capacity(number_flags);
    let mut all_flags_names = Vec::with_capacity(number_flags);

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
        all_flags_names.push(quote!(stringify!(#var_name)));

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
                    // #[derive(Debug, Clone, Copy)]
                    // #[allow(clippy::upper_case_acronyms)]
                    // enum AuxEnum {
                    //     #(#all_flags_names, )*
                    // }

                    #[derive(Clone, Copy)]
                    struct AuxItem(&'static str);

                    impl ::core::fmt::Debug for AuxItem {
                        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            f.pad(self.0)
                        }
                    }

                    // struct Set([Option<AuxEnum>; #num_flags]);
                    struct Set([Option<AuxItem>; #num_flags]);

                    impl Set {
                        fn insert(&mut self, val: AuxItem) {
                            for i in self.0.iter_mut() {
                                if i.is_none() {
                                    *i = Some(val);
                                    break;
                                }
                            }
                        }
                    }

                    impl ::core::fmt::Debug for Set {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            let mut dbg = f.debug_set();

                            for i in self.0.iter().flatten() {
                                dbg.entry(i);
                            }

                            dbg.finish()
                        }
                    }

                    let name = stringify!(#ty_name);

                    let mut set = Set([None; #num_flags]);

                    #(if self.contains(#all_flags) {
                        set.insert(AuxItem(#all_flags_names));
                    })*

                    f.debug_tuple(name)
                        .field(&format_args!("0b{:b}", self.0))
                        .field(&set)
                        .finish()
                }
            }
        }
    };

    let generated = quote! {
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        #(#attrs)*
        #vis struct #ty_name(#ty);

        #[allow(non_upper_case_globals)]
        impl #ty_name {
            #(#flags)*

            /// Return the underlying bits of the bitflag
            #[inline]
            #vis const fn bits(&self) -> #ty {
                self.0
            }

            /// Converts from a `bits` value. Returning [`None`] is any unknown bits are set.
            #[inline]
            #vis const fn from_bits(bits: #ty) -> Option<Self> {
                let truncated = Self::from_bits_truncate(bits).0;

                if truncated == bits {
                    Some(Self(bits))
                } else {
                    None
                }
            }

            /// Convert from `bits` value, unsetting any unknown bits.
            #[inline]
            #vis const fn from_bits_truncate(bits: #ty) -> Self {
                Self(bits & Self::all().0)
            }

            /// Convert from `bits` value exactly.
            #[inline]
            #vis const fn from_bits_retain(bits: #ty) -> Self {
                Self(bits)
            }

            /// Construct an empty bitflag.
            #[inline]
            #vis const fn empty() -> Self {
                Self(0)
            }

            /// Returns `true` if the flag is empty.
            #[inline]
            #vis const fn is_empty(&self) -> bool {
                self.0 == 0
            }

            /// Returns a bitflag that constains all value.
            ///
            /// This will include bits that do not have any flags/meaning.
            /// Use [`all`](Self::all) if you want only the specified flags set.
            #[inline]
            #vis const fn all_bits() -> Self {
                Self(!0)
            }

            /// Returns `true` if the bitflag constains all value bits set.
            ///
            /// This will check for all bits.
            /// Use [`is_all`](Self::is_all) if you want to check for all specified flags.
            #[inline]
            #vis const fn is_all_bits(&self) -> bool {
                self.0 == !0
            }

            /// Construct a bitflag with all flags set.
            ///
            /// This will only set the flags specified as associated constant.
            #[inline]
            #vis const fn all() -> Self {
                Self(#(#all_flags.0 |)* 0)
            }

            /// Returns `true` if the bitflag contais all flags.
            ///
            #[inline]
            #vis const fn is_all(&self) -> bool {
                self.0 == Self::all().0
            }

            /// Returns a bit flag that only has bits corresponding to the specified flags as associated constant.
            #[inline]
            #vis const fn truncate(&self) -> Self {
                Self(self.0 & Self::all().0)
            }

            /// Returns `true` if this bitflag intersects with any value in `other`.
            ///
            /// This is equivalent to `(self & other) != Self::empty()`
            #[inline]
            #vis const fn intersects(&self, other: Self) -> bool {
                (self.0 & other.0) != Self::empty().0
            }

            /// Returns `true` if this bitflag contains all values of `other`.
            ///
            /// This is equivalent to `(self & other) == other`
            #[inline]
            #vis const fn contains(&self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }

            /// Returns the bitwise NOT of the flag.
            #[inline]
            #[doc(alias = "complement")]
            #vis const fn not(self) -> Self {
                Self(!self.0)
            }

            /// Returns the bitwise AND of the flag.
            #[inline]
            #[doc(alias = "intersection")]
            #vis const fn and(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }

            /// Returns the bitwise OR of the flag with `other`.
            #[inline]
            #[doc(alias = "union")]
            #vis const fn or(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }

            /// Returns the bitwise XOR of the flag with `other`.
            #[inline]
            #[doc(alias = "symmetric_difference")]
            #vis const fn xor(self, other: Self) -> Self {
                Self(self.0 ^ other.0)
            }

            /// Returns the intersection from this value with `other`.
            #[inline]
            #[doc(alias = "and")]
            #vis const fn intersection(self, other: Self) -> Self {
                self.and(other)
            }

            /// Returns the union from this value with `other`
            #[inline]
            #[doc(alias = "or")]
            #vis const fn union(self, other: Self) -> Self {
                self.or(other)
            }

            /// Returns the difference from this value with `other`.
            #[inline]
            #vis const fn difference(self, other: Self) -> Self {
                self.and(other.not())
            }

            /// Returns the symmetric difference from this value with `other`.
            #[inline]
            #[doc(alias = "xor")]
            #vis const fn symmetric_difference(self, other: Self) -> Self {
                self.xor(other)
            }

            /// Returns the complement of the value.
            ///
            /// This is very similar to the [`not`](Self::not), but truncates non used bits
            #[inline]
            #[doc(alias = "not")]
            #vis const fn complement(self) -> Self {
                self.not().truncate()
            }

            /// Set the flags in `other` in the value.
            #[inline]
            #vis fn set(&mut self, other: Self) {
                self.0 = self.and(other).0
            }

            /// Unset the flags in `other` in the value.
            #[inline]
            #vis fn unset(&mut self, other: Self) {
                self.0 = self.difference(other).0
            }

            /// Toggle the flags in `other` in the value.
            #[inline]
            #vis fn toggle(&mut self, other: Self) {
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
