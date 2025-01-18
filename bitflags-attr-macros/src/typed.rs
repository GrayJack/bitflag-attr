use syn::{
    parse::Parse, spanned::Spanned, token::Paren, Attribute, Error, Expr, Ident, ItemConst,
    ItemEnum, LitStr, Meta, MetaNameValue, Path, Visibility,
};

use proc_macro2::TokenStream;

use quote::{quote, ToTokens, TokenStreamExt};

pub struct Bitflag {
    vis: Visibility,
    attrs: Vec<Attribute>,
    name: Ident,
    inner_ty: Path,
    repr_attr: Option<ReprAttr>,
    derived_traits: Vec<Ident>,
    impl_debug: bool,
    impl_default: bool,
    impl_serialize: bool,
    impl_deserialize: bool,
    impl_arbitrary: bool,
    all_attrs: Vec<Vec<Attribute>>,
    all_flags: Vec<TokenStream>,
    all_flags_names: Vec<LitStr>,
    flags: Vec<ItemConst>,
    default_value: Option<Expr>,
    custom_known_bits: Option<Expr>,
    orig_enum: ItemEnum,
}

impl Bitflag {
    pub fn parse(args: Args, item: proc_macro::TokenStream) -> syn::Result<Self> {
        let ty = args.ty;

        let item: ItemEnum = syn::parse(item)?;
        let item_span = item.span();
        let og_attrs = item.attrs.iter().filter(|att| {
            !att.path().is_ident("derive")
                && !att.path().is_ident("extra_valid_bits")
                && !att.path().is_ident("repr")
                && !att.path().is_ident("serde")
        });

        let vis = item.vis;
        let name = item.ident;

        let has_non_exhaustive = item
            .attrs
            .iter()
            .any(|att| att.path().is_ident("non_exhaustive"));

        let serde_helper = item.attrs.iter().find(|att| att.path().is_ident("serde"));

        if let Some(serde) = serde_helper {
            return Err(Error::new(
                serde.span(),
                "`serde` helper attribute is not compatible with `bitflag` attribute: in this case, manual implementation of serde traits should be considered",
            ));
        }

        // Attributes
        let attrs = item
            .attrs
            .iter()
            .filter(|att| {
                !att.path().is_ident("derive")
                    && !att.path().is_ident("extra_valid_bits")
                    && !att.path().is_ident("repr")
            })
            .cloned()
            .collect();

        let repr_attr = item
            .attrs
            .iter()
            .find(|att| att.path().is_ident("repr"))
            .map(|att| syn::parse2::<ReprAttr>(att.meta.to_token_stream()));

        let repr_attr = match repr_attr {
            Some(repr) => {
                let repr = repr?;
                Some(repr)
            }
            None => None,
        };

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
        let mut impl_default = false;
        let mut impl_serialize = false;
        let mut impl_deserialize = false;
        let mut impl_arbitrary = false;
        let mut clone_found = false;
        let mut copy_found = false;

        for derive in derives {
            derive.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    if ident == "Debug" {
                        impl_debug = true;
                        return Ok(());
                    }

                    if ident == "Default" {
                        impl_default = true;
                        return Ok(());
                    }

                    if ident == "Serialize" && cfg!(feature = "serde") {
                        impl_serialize = true;
                        return Ok(());
                    }

                    if ident == "Deserialize" && cfg!(feature = "serde") {
                        impl_deserialize = true;
                        return Ok(());
                    }

                    if ident == "Arbitrary" && cfg!(feature = "arbitrary") {
                        impl_arbitrary = true;
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

        let number_flags = item.variants.len();

        let mut all_attrs = Vec::with_capacity(number_flags);
        let mut all_flags = Vec::with_capacity(number_flags);
        let mut all_flags_names = Vec::with_capacity(number_flags);
        let mut all_variants = Vec::with_capacity(number_flags);
        let mut all_non_doc_attrs = Vec::with_capacity(number_flags);
        let mut default_value = None;

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

            let serde_helper = var_attrs.iter().find(|attr| attr.path().is_ident("serde"));

            if let Some(serde) = serde_helper {
                return Err(Error::new(
                    serde.span(),
                    "`serde` helper attribute is not compatible with `bitflag` attribute: in this case, manual implementation of serde traits should be considered",
                ));
            }

            let default_attr = var_attrs
                .iter()
                .find(|attr| attr.path().is_ident("default"));

            if let Some(default) = default_attr {
                if !impl_debug {
                    return Err(Error::new(
                        default.span(),
                        "`default` attribute without `#[derive(Default)]`",
                    ));
                }

                default_value = Some(syn::parse2(quote!(Self::#var_name))?);
            }

            let non_doc_attrs: Vec<Attribute> = var_attrs
                .iter()
                .filter(|attr| !attr.path().is_ident("doc"))
                .cloned()
                .collect();

            let filtered_attrs = var_attrs
                .iter()
                .filter(|attr| !attr.path().is_ident("doc") && !attr.path().is_ident("default"));

            all_flags.push(quote!(Self::#var_name));
            all_flags_names.push(syn::LitStr::new(&var_name.to_string(), var_name.span()));
            all_variants.push(var_name.clone());
            all_attrs.push(filtered_attrs.clone().cloned().collect::<Vec<_>>());
            all_non_doc_attrs.push(non_doc_attrs.clone());
            raw_flags.push(quote! {
                #(#filtered_attrs)*
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

            let all_attr = var_attrs
                .iter()
                .filter(|attr| !attr.path().is_ident("default"));

            let generated = if can_simplify(expr, &all_variants) {
                quote! {
                    #(#all_attr)*
                    #vis const #var_name: Self = Self(#expr);
                }
            } else {
                quote! {
                    #(#all_attr)*
                    #vis const #var_name: Self = {
                        #(#raw_flags)*

                        Self(#expr)
                    };
                }
            };

            flags.push(syn::parse2(generated)?);
        }

        let og_derive =
            (impl_default && default_value.is_some()).then(|| quote!(#[derive(Default)]));
        let orig_enum = syn::parse2(quote! {
            #[allow(dead_code)]
            #(#og_attrs)*
            #og_derive
            enum #name {
                #(
                    #(#all_non_doc_attrs)*
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
            inner_ty: ty,
            derived_traits,
            repr_attr,
            impl_debug,
            impl_default,
            impl_serialize,
            impl_deserialize,
            impl_arbitrary,
            all_attrs,
            all_flags,
            all_flags_names,
            default_value,
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
            inner_ty,
            repr_attr,
            derived_traits,
            impl_debug,
            impl_default,
            impl_serialize,
            impl_deserialize,
            impl_arbitrary,
            all_attrs,
            all_flags,
            all_flags_names,
            default_value,
            flags,
            custom_known_bits,
            orig_enum,
        } = self;

        let extra_valid_bits = custom_known_bits
            .as_ref()
            .map(|expr| quote! {all |= #expr;});

        let extra_valid_bits_value = if let Some(expr) = custom_known_bits {
            quote! {#expr}
        } else {
            quote! {
                {
                    let mut all = 0;

                    #(
                        #(#all_attrs)*{
                            all |= #all_flags.0;
                        }
                    )*

                    all
                }
            }
        };

        let repr_attr = match repr_attr {
            Some(repr) => {
                quote! {#repr}
            }
            None => quote! {#[repr(transparent)]},
        };

        let const_mut = cfg!(feature = "const-mut-ref").then(|| quote!(mut));

        let debug_impl = impl_debug.then(|| {
            quote! {
                #[automatically_derived]
                impl ::core::fmt::Debug for #name {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        struct HumanReadable<'a>(&'a #name);

                        impl<'a> ::core::fmt::Debug for HumanReadable<'a> {
                            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                if self.0.is_empty() {
                                    ::core::write!(f, "{:#X}", self.0.0)
                                } else {
                                    ::bitflag_attr::parser::to_writer(self.0, f)
                                }
                            }
                        }

                        let name = ::core::stringify!(#name);

                        f.debug_struct(name)
                            .field("flags", &HumanReadable(self))
                            // The width `2 +` is to account for the 0b printed before the binary number
                            .field("bits", &::core::format_args!("{:#0width$b}", self.0, width = 2 + #inner_ty::BITS as usize))
                            .finish()
                    }
                }
            }
        });

        let default_impl = impl_default.then(|| {
            if let Some(expr) = default_value {
                quote! {
                    #[automatically_derived]
                    impl ::core::default::Default for #name {
                        #[inline]
                        fn default() -> Self {
                            #expr
                        }
                    }
                }
            } else {
                quote! {
                    #[automatically_derived]
                    impl ::core::default::Default for #name {
                        #[inline]
                        fn default() -> Self {
                            Self(<#inner_ty as ::core::default::Default>::default())
                        }
                    }
                }
            }
        });

        let serialize_impl = (cfg!(feature = "serde") && *impl_serialize).then(|| {
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
                                ::bitflag_attr::parser::to_writer(self.0, f)
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
        });

        let deserialize_impl = (cfg!(feature = "serde") && *impl_deserialize).then(|| {
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
                                    ::bitflag_attr::parser::from_text(flags).map_err(|e| E::custom(e))
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
        });

        let arbitrary_impl = (cfg!(feature = "arbitrary") && *impl_arbitrary).then(|| {
            quote! {
                #[automatically_derived]
                impl<'a> ::arbitrary::Arbitrary<'a> for #name {
                    fn arbitrary(u: &mut ::arbitrary::Unstructured<'a>) -> ::arbitrary::Result<Self> {
                        #name::from_bits(u.arbitrary()?).ok_or(::arbitrary::Error::IncorrectFormat)
                    }
                }
            }
        });

        let doc_from_iter = format!("Create a `{name}` from a iterator of flags.");
        let generated = quote! {
            #repr_attr
            #(#attrs)*
            #[derive(#(#derived_traits,)*)]
            #vis struct #name(#inner_ty)
            where
                #inner_ty: ::bitflag_attr::BitsPrimitive;

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

                    #extra_valid_bits

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

            #[automatically_derived]
            impl ::core::str::FromStr for #name {
                type Err = ::bitflag_attr::parser::ParseError;

                fn from_str(input: &str) -> ::core::result::Result<Self, Self::Err> {
                    ::bitflag_attr::parser::from_text(input)
                }
            }

            #debug_impl

            #default_impl

            #[automatically_derived]
            impl ::bitflag_attr::Flags for #name {
                const KNOWN_FLAGS: &'static [(&'static str, #name)] = &[#(
                    #(#all_attrs)*
                    (#all_flags_names , #all_flags) ,
                )*];

                const EXTRA_VALID_BITS: #inner_ty = #extra_valid_bits_value;

                type Bits = #inner_ty;

                fn bits(&self) -> Self::Bits {
                    self.0
                }

                fn from_bits_retain(bits: Self::Bits) -> Self {
                    Self(bits)
                }
            }

            impl #name {
                const KNOWN_FLAGS: &'static [(&'static str, #name)] = &[#(
                    #(#all_attrs)*
                    (#all_flags_names , #all_flags) ,
                )*];

                /// Yield a set of contained flags values.
                ///
                /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
                /// will be yielded together as a final flags value.
                #[inline]
                pub const fn iter(&self) -> ::bitflag_attr::iter::Iter<Self> {
                    ::bitflag_attr::iter::Iter::__private_const_new(Self::KNOWN_FLAGS, *self, *self)
                }

                /// Yield a set of contained named flags values.
                ///
                /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
                /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
                #[inline]
                pub const fn iter_names(&self) -> ::bitflag_attr::iter::IterNames<Self> {
                    ::bitflag_attr::iter::IterNames::__private_const_new(Self::KNOWN_FLAGS, *self, *self)
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
                type IntoIter = ::bitflag_attr::iter::Iter<Self>;

                fn into_iter(self) -> Self::IntoIter {
                    self.iter()
                }
            }

            #[automatically_derived]
            impl ::core::iter::IntoIterator for &#name {
                type Item = #name;
                type IntoIter = ::bitflag_attr::iter::Iter<#name>;

                fn into_iter(self) -> Self::IntoIter {
                    self.iter()
                }
            }

            #serialize_impl
            #deserialize_impl
            #arbitrary_impl
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

struct ReprAttr {
    path: Path,
    _paren_token: Paren,
    kind: ReprKind,
}

impl Parse for ReprAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let repr: syn::MetaList = input.parse()?;

        if !repr.path.is_ident("repr") {
            return Err(Error::new(repr.path.span(), "not a repr"));
        }

        let _paren_token = match repr.delimiter {
            syn::MacroDelimiter::Paren(paren) => paren,
            syn::MacroDelimiter::Brace(b) => {
                return Err(Error::new(
                    b.span.span(),
                    "invalid syntax, expected parenthesis",
                ))
            }
            syn::MacroDelimiter::Bracket(b) => {
                return Err(Error::new(
                    b.span.span(),
                    "invalid syntax, expected parenthesis",
                ))
            }
        };

        let kind: ReprKind = syn::parse2(repr.tokens)?;

        Ok(Self {
            path: repr.path,
            _paren_token,
            kind,
        })
    }
}

impl ToTokens for ReprAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { path, kind, .. } = self;
        tokens.append_all(quote! {#[#path(#kind)]});
    }
}

/// Supported repr
enum ReprKind {
    C,
    Rust,
    Transparent,
}

impl Parse for ReprKind {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let meta: Meta = input.parse()?;

        match meta {
            Meta::Path(path) => {
                let text = path
                    .get_ident()
                    .map(|p| p.to_string())
                    .unwrap_or("".to_string());

                match text.as_str() {
                    "C" => Ok(Self::C),
                    "Rust" => Ok(Self::Rust),
                    "transparent" => Ok(Self::Transparent),
                    _ => Err(Error::new(
                        path.span(),
                        "`bitflag` unsupported repr: Supported repr are `C`, `Rust` and `transparent`",
                    )),
                }
            }
            _ => Err(Error::new(
                meta.span(),
                "`bitflag` unsupported repr: Supported repr are `C`, `Rust` and `transparent`",
            )),
        }
    }
}

impl ToTokens for ReprKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ReprKind::C => tokens.append_all(quote!(C)),
            ReprKind::Rust => tokens.append_all(quote!(Rust)),
            ReprKind::Transparent => tokens.append_all(quote!(transparent)),
        }
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
///
/// To-Do:
/// In theory, something like `FlagTypeName::FlagKind.bits()` could be simplified, but demands a more complicated analysis of method call expression
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

    // compound path
    // All real usages I had it could be simplified, after simplification, even cases where type
    // don't match, the resulting native error is very good.
    if expr.path.segments.iter().count() >= 2 {
        return true;
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
