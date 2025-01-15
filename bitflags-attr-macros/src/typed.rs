use syn::{
    parse::Parse, spanned::Spanned, Attribute, Error, Expr, Ident, ItemConst, ItemEnum, LitStr,
    Meta, MetaNameValue, Path, Visibility,
};

use proc_macro2::TokenStream;

use quote::{quote, ToTokens, TokenStreamExt};

pub struct Bitflag {
    vis: Visibility,
    attrs: Vec<Attribute>,
    name: Ident,
    iter_name_ty: Ident,
    iter_ty: Ident,
    inner_ty: Path,
    derived_traits: Vec<Ident>,
    impl_debug: bool,
    impl_serialize: bool,
    impl_deserialize: bool,
    all_attrs: Vec<Vec<Attribute>>,
    all_flags: Vec<TokenStream>,
    all_flags_names: Vec<LitStr>,
    flags: Vec<ItemConst>,
    custom_known_bits: Option<Expr>,
    orig_enum: ItemEnum,
}

impl Bitflag {
    pub fn parse(args: Args, item: proc_macro::TokenStream) -> syn::Result<Self> {
        let ty = args.ty;

        let item: ItemEnum = syn::parse(item)?;
        let item_span = item.span();
        let og_attrs = item
            .attrs
            .iter()
            .filter(|att| !att.path().is_ident("extra_valid_bits"));

        let vis = item.vis;
        let name = item.ident;

        let has_non_exhaustive = item
            .attrs
            .iter()
            .any(|att| att.path().is_ident("non_exhaustive"));

        // Attributes
        let attrs = item
            .attrs
            .iter()
            .filter(|att| {
                !att.path().is_ident("derive") && !att.path().is_ident("extra_valid_bits")
            })
            .cloned()
            .collect();

        let valid_bits_attr = item
            .attrs
            .iter()
            .find(|att| att.path().is_ident("extra_valid_bits"));

        let derives = item
            .attrs
            .iter()
            .filter(|att| att.path().is_ident("derive"));

        let mut derived_traits = Vec::new();
        let mut impl_debug = false;
        let mut impl_serialize = false;
        let mut impl_deserialize = false;
        let mut clone_found = false;
        let mut copy_found = false;

        for derive in derives {
            derive.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    if ident == "Debug" {
                        impl_debug = true;
                        return Ok(());
                    }

                    if ident == "Serialize" {
                        impl_serialize = true;
                        return Ok(());
                    }

                    if ident == "Deserialize" {
                        impl_deserialize = true;
                        return Ok(());
                    }

                    if ident == "Clone" {
                        clone_found = true;
                    }

                    if ident == "Copy" {
                        copy_found = true;
                    }

                    derived_traits.push(ident.clone());
                }
                Ok(())
            })?;
        }

        if !clone_found || !copy_found {
            return Err(syn::Error::new(
                item_span,
                "`bitflags` attribute requires the type to derive `Clone` and `Copy`",
            ));
        }

        let iter_name_ty = {
            let span = name.span();
            let mut ty_name = name.to_string();
            ty_name.push_str("IterNames");
            Ident::new(&ty_name, span)
        };
        let iter_ty = {
            let span = name.span();
            let mut ty_name = name.to_string();
            ty_name.push_str("Iter");
            Ident::new(&ty_name, span)
        };

        let number_flags = item.variants.len();

        let mut all_attrs = Vec::with_capacity(number_flags);
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

            let non_doc_attrs: Vec<Attribute> = var_attrs
                .iter()
                .filter(|attr| !attr.path().is_ident("doc"))
                .cloned()
                .collect();

            all_flags.push(quote!(Self::#var_name));
            all_flags_names.push(syn::LitStr::new(&var_name.to_string(), var_name.span()));
            all_variants.push(var_name.clone());
            all_attrs.push(non_doc_attrs.clone());
            raw_flags.push(quote! {
                #(#non_doc_attrs)*
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

            let generated = if can_simplify(expr, &all_variants) {
                quote! {
                    #(#var_attrs)*
                    #vis const #var_name: Self = Self(#expr);
                }
            } else {
                quote! {
                    #(#var_attrs)*
                    #vis const #var_name: Self = {
                        #(#raw_flags)*

                        Self(#expr)
                    };
                }
            };

            flags.push(syn::parse2(generated)?);
        }

        let orig_enum = syn::parse2(quote! {
            #(#og_attrs)*
            enum #name {
                #(
                    #(#all_attrs)*
                    #all_variants,
                )*
            }
        })?;

        let custom_known_bits: Option<Expr> = if let Some(attr) = valid_bits_attr {
            let parsed = ExtraValidBits::from_meta(&attr.meta)?;

            Some(parsed.0)
        } else if has_non_exhaustive {
            Some(syn::parse2(quote! {!0})?)
        } else {
            None
        };

        Ok(Self {
            vis,
            attrs,
            name,
            iter_name_ty,
            iter_ty,
            inner_ty: ty,
            derived_traits,
            impl_debug,
            impl_serialize,
            impl_deserialize,
            all_attrs,
            all_flags,
            all_flags_names,
            flags,
            custom_known_bits,
            orig_enum,
        })
    }
}

impl ToTokens for Bitflag {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            vis,
            attrs,
            name,
            iter_name_ty,
            iter_ty,
            inner_ty,
            derived_traits,
            impl_debug,
            impl_serialize,
            impl_deserialize,
            all_attrs,
            all_flags,
            all_flags_names,
            flags,
            custom_known_bits,
            orig_enum,
        } = self;

        let extra_valid_bits = if let Some(expr) = custom_known_bits {
            quote! {all |= #expr;}
        } else {
            quote! {}
        };

        let const_mut = if cfg!(feature = "const-mut-ref") {
            quote!(mut)
        } else {
            quote!()
        };

        let debug_impl = if !impl_debug {
            quote! {}
        } else {
            quote! {
                #[automatically_derived]
                impl ::core::fmt::Debug for #name {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        struct HumanReadable<'a>(&'a #name);

                        impl<'a> ::core::fmt::Debug for HumanReadable<'a> {
                            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                self.0.to_writer(f)
                            }
                        }

                        let name = ::core::stringify!(#name);

                        f.debug_struct(name)
                            .field("bits", &::core::format_args!("{:#b}", self.0))
                            .field("human_readable", &HumanReadable(self))
                            .finish()
                    }
                }
            }
        };

        let serialize_impl = if cfg!(feature = "serde") && *impl_serialize {
            quote! {
                #[automatically_derived]
                impl ::serde::Serialize for #name {
                    fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::Serializer
                    {
                        struct AsDisplay<'a>(&'a #name);

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
            }
        } else {
            quote!()
        };

        let deserialize_impl = if cfg!(feature = "serde") && *impl_deserialize {
            quote! {
                #[automatically_derived]
                impl<'de> ::serde::Deserialize<'de> for #name {
                    fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
                    where
                        D: ::serde::Deserializer<'de>
                    {
                        if deserializer.is_human_readable() {
                            struct HelperVisitor(::core::marker::PhantomData<#name>);

                            impl<'de> ::serde::de::Visitor<'de> for HelperVisitor {
                                type Value = #name;

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
                            let bits = #inner_ty::deserialize(deserializer)?;

                            Ok(#name::from_bits_retain(bits))
                        }
                    }
                }
            }
        } else {
            quote!()
        };

        // Serde infra_structure
        let serde_impl = if cfg!(feature = "serde") {
            let parser_error_ty = {
                let span = name.span();
                let mut ty = name.to_string();
                ty.push_str("ParserError");
                Ident::new(&ty, span)
            };
            quote! {

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

                impl #name {
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
                                let bits = #inner_ty::from_str_radix(flag, 16).map_err(|_| #parser_error_ty::InvalidHexFlag)?;

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
                        Ok(Self::from_text(input)?.truncated())
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
                impl ::core::str::FromStr for #name {
                    type Err = #parser_error_ty;

                    fn from_str(input: &str) -> ::core::result::Result<Self, Self::Err> {
                        Self::from_text(input)
                    }
                }
            }
        } else {
            quote!()
        };

        let doc_from_iter = format!("Create a `{name}` from a iterator of flags.");
        let generated = quote! {
            #[repr(transparent)]
            #(#attrs)*
            #[derive(#(#derived_traits,)*)]
            #vis struct #name(#inner_ty)
            where
                #inner_ty: ::bitflag_attr::BitflagPrimitive;

            #[allow(non_upper_case_globals)]
            impl #name {
                #[doc(hidden)]
                #[allow(clippy::unused_unit)]
                const __OG: () = {
                    {
                        // Original enum
                        // This is a hack to make LSP coloring to still sees the original enum variant as a Enum variant token.
                        #orig_enum
                    }
                    ()
                };

                #(#flags)*
            }

            #[allow(non_upper_case_globals)]
            impl #name {
                /// Return the underlying bits of this bitflag.
                #[inline]
                pub const fn bits(&self) -> #inner_ty {
                    self.0
                }

                /// Converts from a `bits` value. Returning [`None`] is any unknown bits are set.
                #[inline]
                pub const fn from_bits(bits: #inner_ty) -> Option<Self> {
                    let truncated = Self::from_bits_truncate(bits).0;

                    if truncated == bits {
                        Some(Self(bits))
                    } else {
                        None
                    }
                }

                /// Convert from `bits` value, unsetting any unknown bits.
                #[inline]
                pub const fn from_bits_truncate(bits: #inner_ty) -> Self {
                    Self(bits & Self::all().0)
                }

                /// Convert from `bits` value exactly.
                #[inline]
                pub const fn from_bits_retain(bits: #inner_ty) -> Self {
                    Self(bits)
                }

                /// Convert from a flag `name`.
                #[inline]
                pub fn from_flag_name(name: &str) -> Option<Self> {
                    match name {
                        #(
                            #(#all_attrs)*
                            #all_flags_names => Some(#all_flags),
                        )*
                        _ => None
                    }
                }

                /// Construct a flags value with all bits unset.
                #[inline]
                pub const fn empty() -> Self {
                    Self(0)
                }

                /// Returns `true` if the flag value has all bits unset.
                #[inline]
                pub const fn is_empty(&self) -> bool {
                    self.0 == 0
                }

                /// Returns a flag value that contains all value.
                ///
                /// This will include bits that do not have any flags/meaning.
                /// Use [`all`](Self::all) if you want only the specified flags set.
                #[inline]
                pub const fn all_bits() -> Self {
                    Self(!0)
                }

                /// Returns `true` if the flag value contains all value bits set.
                ///
                /// This will check for all bits.
                /// Use [`is_all`](Self::is_all) if you want to check for all specified flags.
                #[inline]
                pub const fn is_all_bits(&self) -> bool {
                    self.0 == !0
                }

                /// Construct a flag value with all known flags set.
                ///
                /// This will only set the flags specified as associated constant.
                #[inline]
                pub const fn all() -> Self {
                    let mut all = 0;

                    #(
                        #(#all_attrs)*{
                            all |= #all_flags.0;
                        }
                    )*

                    #extra_valid_bits;

                    Self(all)
                }

                /// Returns `true` if the flag value contais all known flags.
                #[inline]
                pub const fn is_all(&self) -> bool {
                    Self::all().0 | self.0 == self.0
                }

                /// Returns `true` if there are any unknown bits set in the flag value.
                #[inline]
                pub const fn contains_unknown_bits(&self) -> bool {
                    Self::all().0 & self.0 != self.0
                }

                /// Returns a bit flag that only has bits corresponding to the specified flags as associated constant.
                #[inline]
                pub const fn truncated(&self) -> Self {
                    Self(self.0 & Self::all().0)
                }

                /// Removes unknown bits from the flag value.
                #[inline]
                pub #const_mut fn truncate(&mut self) {
                    *self = Self::from_bits_truncate(self.0);
                }

                /// Returns `true` if this flag value intersects with any value in `other`.
                ///
                /// This is equivalent to `(self & other) != Self::empty()`
                #[inline]
                pub const fn intersects(&self, other: Self) -> bool {
                    (self.0 & other.0) != Self::empty().0
                }

                /// Returns `true` if this flag value contains all values of `other`.
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

                /// Returns the union from this value with `other`.
                #[inline]
                #[doc(alias = "or")]
                pub const fn union(self, other: Self) -> Self {
                    self.or(other)
                }

                /// Returns the difference from this value with `other`.
                ///
                /// In other words, returns the intersection of this value with the negation of `other`.
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `difference` won't truncate `other`, but the `!` operator will.
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
                /// This is very similar to the [`not`](Self::not), but truncates non used bits.
                #[inline]
                #[doc(alias = "not")]
                pub const fn complement(self) -> Self {
                    self.not().truncated()
                }

                /// Set the flags in `other` in the value.
                #[inline]
                #[doc(alias = "insert")]
                pub #const_mut fn set(&mut self, other: Self) {
                    self.0 = self.or(other).0
                }

                /// Unset the flags bits in `other` in the value.
                #[inline]
                #[doc(alias = "remove")]
                pub #const_mut fn unset(&mut self, other: Self) {
                    self.0 = self.difference(other).0
                }

                /// Toggle the flags in `other` in the value.
                #[inline]
                pub #const_mut fn toggle(&mut self, other: Self) {
                    self.0 = self.xor(other).0
                }
            }

            #[automatically_derived]
            impl ::core::ops::Not for #name {
                type Output = Self;

                #[inline]
                fn not(self) -> Self::Output {
                    self.complement()
                }
            }

            #[automatically_derived]
            impl ::core::ops::BitAnd for #name {
                type Output = Self;

                #[inline]
                fn bitand(self, rhs: Self) -> Self::Output {
                    self.and(rhs)
                }
            }

            #[automatically_derived]
            impl ::core::ops::BitOr for #name {
                type Output = Self;

                #[inline]
                fn bitor(self, rhs: Self) -> Self::Output {
                    self.or(rhs)
                }
            }

            #[automatically_derived]
            impl ::core::ops::BitXor for #name {
                type Output = Self;

                #[inline]
                fn bitxor(self, rhs: Self) -> Self::Output {
                    self.xor(rhs)
                }
            }

            #[automatically_derived]
            impl ::core::ops::BitAndAssign for #name {
                #[inline]
                fn bitand_assign(&mut self, rhs: Self) {
                    ::core::ops::BitAndAssign::bitand_assign(&mut self.0, rhs.0)
                }
            }

            #[automatically_derived]
            impl ::core::ops::BitOrAssign for #name {
                #[inline]
                fn bitor_assign(&mut self, rhs: Self) {
                    ::core::ops::BitOrAssign::bitor_assign(&mut self.0, rhs.0)
                }
            }

            #[automatically_derived]
            impl ::core::ops::BitXorAssign for #name {
                #[inline]
                fn bitxor_assign(&mut self, rhs: Self) {
                    ::core::ops::BitXorAssign::bitxor_assign(&mut self.0, rhs.0)
                }
            }

            #[automatically_derived]
            impl ::core::ops::Sub for #name {
                type Output = Self;

                /// The intersection of a source flag with the complement of a target flags value
                #[inline]
                fn sub(self, rhs: Self) -> Self::Output {
                    self.difference(rhs)
                }
            }

            #[automatically_derived]
            impl ::core::ops::SubAssign for #name {
                /// The intersection of a source flag with the complement of a target flags value
                #[inline]
                fn sub_assign(&mut self, rhs: Self) {
                    self.unset(rhs)
                }
            }

            #[automatically_derived]
            impl ::core::convert::From<#inner_ty> for #name {
                #[inline]
                fn from(val: #inner_ty) -> Self {
                    Self::from_bits_truncate(val)
                }
            }

            #[automatically_derived]
            impl ::core::convert::From<#name> for #inner_ty {
                #[inline]
                fn from(val: #name) -> Self {
                    val.0
                }
            }

            #[automatically_derived]
            impl ::core::fmt::Binary for #name {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::Binary::fmt(&self.0, f)
                }
            }

            #[automatically_derived]
            impl ::core::fmt::LowerHex for #name {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::LowerHex::fmt(&self.0, f)
                }
            }

            #[automatically_derived]
            impl ::core::fmt::UpperHex for #name {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::UpperHex::fmt(&self.0, f)
                }
            }

            #[automatically_derived]
            impl ::core::fmt::Octal for #name {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::Octal::fmt(&self.0, f)
                }
            }

            #debug_impl

            impl #name {
                const FLAGS: &'static [(&'static str, #name)] = &[#(
                    #(#all_attrs)*
                    (#all_flags_names , #all_flags) ,
                )*];

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
                pub(crate) fn to_writer_truncate<W>(&self, writer: W) -> ::core::fmt::Result
                where
                    W: ::core::fmt::Write
                {
                    self.truncated().to_writer(writer)
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
            impl ::core::iter::Extend<#name> for #name {
                /// Set all flags of `iter` to self
                fn extend<T: ::core::iter::IntoIterator<Item = Self>>(&mut self, iter: T) {
                    for item in iter {
                        self.set(item);
                    }
                }
            }

            #[automatically_derived]
            impl ::core::iter::FromIterator<#name> for #name {
                #[doc = #doc_from_iter]
                fn from_iter<T: ::core::iter::IntoIterator<Item = Self>>(iter: T) -> Self {
                    use ::core::iter::Extend;

                    let mut res = Self::empty();
                    res.extend(iter);
                    res
                }
            }

            #[automatically_derived]
            impl ::core::iter::IntoIterator for #name {
                type Item = Self;
                type IntoIter = #iter_ty;

                fn into_iter(self) -> Self::IntoIter {
                    self.iter()
                }
            }

            #[automatically_derived]
            impl ::core::iter::IntoIterator for &#name {
                type Item = #name;
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
                flags: &'static [(&'static str, #name)],
                index: usize,
                source: #name,
                remaining: #name,
            }

            impl #iter_name_ty {
                pub(crate) const fn new(flags: &#name) -> Self {
                    Self {
                        flags: #name::FLAGS,
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
                pub const fn remaining(&self) -> #name {
                    self.remaining
                }
            }

            #[automatically_derived]
            impl ::core::iter::Iterator for #iter_name_ty {
                type Item = (&'static str, #name);

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
                pub(crate) const fn new(flags: &#name) -> Self  {
                    Self {
                        inner: #iter_name_ty::new(flags),
                        done: false,
                    }
                }
            }

            #[automatically_derived]
            impl ::core::iter::Iterator for #iter_ty {
                type Item = #name;

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

            #serialize_impl
            #deserialize_impl
        };

        tokens.append_all(generated);
    }
}

pub struct Args {
    ty: Path,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ty: Path = input.parse().map_err(|err| {
            Error::new(err.span(), "unexpected token: expected a `{integer}` type")
        })?;

        if !cfg!(feature = "custom-types") {
            if let Some(ident) = ty.get_ident() {
                if !VALID_TYPES.contains(&ident.to_string().as_str()) {
                    return Err(Error::new_spanned(ident, "type must be a `{integer}` type"));
                }
            }
        }

        Ok(Args { ty })
    }
}

struct ExtraValidBits(Expr);

impl ExtraValidBits {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta {
            Meta::NameValue(m) => {
                if !m.path.is_ident("extra_valid_bits") {
                    return Err(Error::new(
                        m.span(),
                        "not a valid `extra_valid_bits` attribute",
                    ));
                }

                Ok(Self(m.value.clone()))
            }
            _ => Err(Error::new(
                meta.span(),
                "extra_valid_bits must follow the syntax `extra_valid_bits = <expr>`",
            )),
        }
    }
}

impl Parse for ExtraValidBits {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let meta: MetaNameValue = input.parse()?;

        if !meta.path.is_ident("extra_valid_bits") {
            return Err(Error::new(meta.span(), "not a extra_valid_bits attribute"));
        }

        Ok(Self(meta.value))
    }
}

/// Recursively check if a expression can be simplified to a simple wrap of `Self(<expr>)`.
///
/// Logic behind this:
/// A literal and a path where it is not fancy and is not one of the variants names are always able to be simplified.
///
/// A unary expression can be simplified if it's underlying expression is also able to be simplified.
///
/// A binary expression can be simplified if both expression that compose it also are able to be simplified.
///
/// A parenthesized expression can be simplified if it's underlying expression is also able to be simplified.
///
/// A "as" cast can be simplified if it's underlying expression is also able to be simplified.
fn can_simplify(expr: &syn::Expr, variants: &[Ident]) -> bool {
    match expr {
        syn::Expr::Lit(_) => true,
        syn::Expr::Path(expr_path) if is_simple_path(expr_path, variants) => true,
        syn::Expr::Unary(expr_unary) => can_simplify(&expr_unary.expr, variants),
        syn::Expr::Binary(expr_binary) => {
            can_simplify(&expr_binary.left, variants) && can_simplify(&expr_binary.right, variants)
        }
        syn::Expr::Paren(expr_paren) => can_simplify(&expr_paren.expr, variants),
        syn::Expr::Cast(expr_cast) => can_simplify(&expr_cast.expr, variants),
        _ => false,
    }
}

fn is_simple_path(expr: &syn::ExprPath, variants: &[Ident]) -> bool {
    if expr.qself.is_some() {
        return false;
    }

    // simplest path
    if let Some(ident) = expr.path.get_ident() {
        // if the ident is in variants, it is not a simple path
        if !variants.contains(ident) {
            return true;
        }
    }

    false
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
