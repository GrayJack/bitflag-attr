//! Example of the generated code by bitflag macro.

const CONST1: u32 = 0b10;
const CONST2: u32 = 0b100;

#[repr(transparent)]
#[doc = " A simple bitflag"]
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct SimpleFlag(u32);

#[allow(non_upper_case_globals)]
impl SimpleFlag {
    #[doc(hidden)]
    #[allow(clippy::unused_unit)]
    const __OG: () = {
        {
            #[doc = " A simple bitflag"]
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
            enum SimpleFlag {
                Flag1,
                Flag2,
                Flag3,
                Flag4,
                Flag5,
                Flag6,
                Flag7,
                Flag8,
                Flag9,
            }
        }
        ()
    };
    pub const Flag1: Self = Self(1 << 9);
    pub const Flag2: Self = Self(1 << 12);
    pub const Flag3: Self = Self(CONST1);
    pub const Flag4: Self = Self(!CONST1);
    pub const Flag5: Self = Self(CONST1 | CONST2 | 3);
    pub const Flag6: Self = {
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag1: u32 = 1 << 9;
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag2: u32 = 1 << 12;
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag3: u32 = CONST1;
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag4: u32 = !CONST1;
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag5: u32 = CONST1 | CONST2 | 3;
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag6: u32 = Flag1 | Flag2;
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag7: u32 = CONST1 | Flag1;
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag8: u32 = (1 << 1) | (1 << 4);
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag9: u32 = 1u8 as u32;
        Self(Flag1 | Flag2)
    };
    pub const Flag7: Self = {
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag1: u32 = 1 << 9;
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag2: u32 = 1 << 12;
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag3: u32 = CONST1;
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag4: u32 = !CONST1;
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag5: u32 = CONST1 | CONST2 | 3;
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag6: u32 = Flag1 | Flag2;
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag7: u32 = CONST1 | Flag1;
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag8: u32 = (1 << 1) | (1 << 4);
        #[allow(non_upper_case_globals, dead_code, unused)]
        const Flag9: u32 = 1u8 as u32;
        Self(CONST1 | Flag1)
    };
    pub const Flag8: Self = Self((1 << 1) | (1 << 4));
    pub const Flag9: Self = Self(1u8 as u32);
}
#[allow(non_upper_case_globals)]
impl SimpleFlag {
    #[doc = r" Return the underlying bits of the bitflag"]
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    #[doc = r" Converts from a `bits` value. Returning [`None`] is any unknown bits are set."]
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        let truncated = Self::from_bits_truncate(bits).0;
        if truncated == bits {
            Some(Self(bits))
        } else {
            None
        }
    }
    #[doc = r" Convert from `bits` value, unsetting any unknown bits."]
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(bits & Self::all().0)
    }
    #[doc = r" Convert from `bits` value exactly."]
    #[inline]
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }
    #[doc = r" Convert from a flag `name`."]
    #[inline]
    pub fn from_flag_name(name: &str) -> Option<Self> {
        match name {
            "Flag1" => Some(Self::Flag1),
            "Flag2" => Some(Self::Flag2),
            "Flag3" => Some(Self::Flag3),
            "Flag4" => Some(Self::Flag4),
            "Flag5" => Some(Self::Flag5),
            "Flag6" => Some(Self::Flag6),
            "Flag7" => Some(Self::Flag7),
            "Flag8" => Some(Self::Flag8),
            "Flag9" => Some(Self::Flag9),
            _ => None,
        }
    }
    #[doc = r" Construct an empty bitflag."]
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    #[doc = r" Returns `true` if the flag is empty."]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }
    #[doc = r" Returns a bitflag that contains all value."]
    #[doc = r""]
    #[doc = r" This will include bits that do not have any flags/meaning."]
    #[doc = r" Use [`all`](Self::all) if you want only the specified flags set."]
    #[inline]
    pub const fn all_bits() -> Self {
        Self(!0)
    }
    #[doc = r" Returns `true` if the bitflag contains all value bits set."]
    #[doc = r""]
    #[doc = r" This will check for all bits."]
    #[doc = r" Use [`is_all`](Self::is_all) if you want to check for all specified flags."]
    #[inline]
    pub const fn is_all_bits(&self) -> bool {
        self.0 == !0
    }
    #[doc = r" Construct a bitflag with all known flags set."]
    #[doc = r""]
    #[doc = r" This will only set the flags specified as associated constant."]
    #[inline]
    pub const fn all() -> Self {
        let mut all = 0;
        {
            all |= Self::Flag1.0
        }
        {
            all |= Self::Flag2.0
        }
        {
            all |= Self::Flag3.0
        }
        {
            all |= Self::Flag4.0
        }
        {
            all |= Self::Flag5.0
        }
        {
            all |= Self::Flag6.0
        }
        {
            all |= Self::Flag7.0
        }
        {
            all |= Self::Flag8.0
        }
        {
            all |= Self::Flag9.0
        }
        Self(all)
    }
    #[doc = r" Returns `true` if the bitflag contais all known flags."]
    #[doc = r""]
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.0 == Self::all().0
    }
    #[doc = r" Returns a bit flag that only has bits corresponding to the specified flags as associated constant."]
    #[inline]
    pub const fn truncate(&self) -> Self {
        Self(self.0 & Self::all().0)
    }
    #[doc = r" Returns `true` if this bitflag intersects with any value in `other`."]
    #[doc = r""]
    #[doc = r" This is equivalent to `(self & other) != Self::empty()`"]
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        (self.0 & other.0) != Self::empty().0
    }
    #[doc = r" Returns `true` if this bitflag contains all values of `other`."]
    #[doc = r""]
    #[doc = r" This is equivalent to `(self & other) == other`"]
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    #[doc = r" Returns the bitwise NOT of the flag."]
    #[doc = r""]
    #[doc = r" This function does not truncate unused bits (bits that do not have any flags/meaning)."]
    #[doc = r" Use [`complement`](Self::complement) if you want that the result to be truncated in one call."]
    #[inline]
    #[doc(alias = "complement")]
    pub const fn not(self) -> Self {
        Self(!self.0)
    }
    #[doc = r" Returns the bitwise AND of the flag."]
    #[inline]
    #[doc(alias = "intersection")]
    pub const fn and(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
    #[doc = r" Returns the bitwise OR of the flag with `other`."]
    #[inline]
    #[doc(alias = "union")]
    pub const fn or(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
    #[doc = r" Returns the bitwise XOR of the flag with `other`."]
    #[inline]
    #[doc(alias = "symmetric_difference")]
    pub const fn xor(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }
    #[doc = r" Returns the intersection from this value with `other`."]
    #[inline]
    #[doc(alias = "and")]
    pub const fn intersection(self, other: Self) -> Self {
        self.and(other)
    }
    #[doc = r" Returns the union from this value with `other`"]
    #[inline]
    #[doc(alias = "or")]
    pub const fn union(self, other: Self) -> Self {
        self.or(other)
    }
    #[doc = r" Returns the difference from this value with `other`."]
    #[doc = r""]
    #[doc = r" In other words, returns the intersection of this value with the negation of `other`."]
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        self.and(other.not())
    }
    #[doc = r" Returns the symmetric difference from this value with `other`."]
    #[inline]
    #[doc(alias = "xor")]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        self.xor(other)
    }
    #[doc = r" Returns the complement of the value."]
    #[doc = r""]
    #[doc = r" This is very similar to the [`not`](Self::not), but truncates non used bits"]
    #[inline]
    #[doc(alias = "not")]
    pub const fn complement(self) -> Self {
        self.not().truncate()
    }
    #[doc = r" Set the flags in `other` in the value."]
    #[inline]
    #[doc(alias = "insert")]
    pub fn set(&mut self, other: Self) {
        self.0 = self.or(other).0
    }
    #[doc = r" Unset the flags in `other` in the value."]
    #[inline]
    #[doc(alias = "remove")]
    pub fn unset(&mut self, other: Self) {
        self.0 = self.difference(other).0
    }
    #[doc = r" Toggle the flags in `other` in the value."]
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 = self.xor(other).0
    }
}
#[automatically_derived]
impl ::core::ops::Not for SimpleFlag {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        self.complement()
    }
}
#[automatically_derived]
impl ::core::ops::BitAnd for SimpleFlag {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}
#[automatically_derived]
impl ::core::ops::BitOr for SimpleFlag {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}
#[automatically_derived]
impl ::core::ops::BitXor for SimpleFlag {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}
#[automatically_derived]
impl ::core::ops::BitAndAssign for SimpleFlag {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        ::core::ops::BitAndAssign::bitand_assign(&mut self.0, rhs.0)
    }
}
#[automatically_derived]
impl ::core::ops::BitOrAssign for SimpleFlag {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        ::core::ops::BitOrAssign::bitor_assign(&mut self.0, rhs.0)
    }
}
#[automatically_derived]
impl ::core::ops::BitXorAssign for SimpleFlag {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        ::core::ops::BitXorAssign::bitxor_assign(&mut self.0, rhs.0)
    }
}
#[automatically_derived]
impl ::core::ops::Sub for SimpleFlag {
    type Output = Self;
    #[doc = r" The intersection of a source flag with the complement of a target flags value"]
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}
#[automatically_derived]
impl ::core::ops::SubAssign for SimpleFlag {
    #[doc = r" The intersection of a source flag with the complement of a target flags value"]
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.unset(rhs)
    }
}
#[automatically_derived]
impl ::core::convert::From<u32> for SimpleFlag {
    #[inline]
    fn from(val: u32) -> Self {
        Self::from_bits_truncate(val)
    }
}
#[automatically_derived]
impl ::core::convert::From<SimpleFlag> for u32 {
    #[inline]
    fn from(val: SimpleFlag) -> Self {
        val.0
    }
}
#[automatically_derived]
impl ::core::fmt::Binary for SimpleFlag {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Binary::fmt(&self.0, f)
    }
}
#[automatically_derived]
impl ::core::fmt::LowerHex for SimpleFlag {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::LowerHex::fmt(&self.0, f)
    }
}
#[automatically_derived]
impl ::core::fmt::UpperHex for SimpleFlag {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::UpperHex::fmt(&self.0, f)
    }
}
#[automatically_derived]
impl ::core::fmt::Octal for SimpleFlag {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Octal::fmt(&self.0, f)
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for SimpleFlag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        struct HumanReadable<'a>(&'a SimpleFlag);

        impl<'a> ::core::fmt::Debug for HumanReadable<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.0.to_writer(f)
            }
        }
        let name = "SimpleFlag";
        f.debug_struct(name)
            .field("bits", &::core::format_args!("{:#b}", self.0))
            .field("human_readable", &HumanReadable(self))
            .finish()
    }
}
impl SimpleFlag {
    const FLAGS: &'static [(&'static str, SimpleFlag)] = &[
        ("Flag1", Self::Flag1),
        ("Flag2", Self::Flag2),
        ("Flag3", Self::Flag3),
        ("Flag4", Self::Flag4),
        ("Flag5", Self::Flag5),
        ("Flag6", Self::Flag6),
        ("Flag7", Self::Flag7),
        ("Flag8", Self::Flag8),
        ("Flag9", Self::Flag9),
    ];
    #[doc = r" Yield a set of contained flags values."]
    #[doc = r""]
    #[doc = r" Each yielded flags value will correspond to a defined named flag. Any unknown bits"]
    #[doc = r" will be yielded together as a final flags value."]
    #[inline]
    pub const fn iter(&self) -> SimpleFlagIter {
        SimpleFlagIter::new(self)
    }
    #[doc = r" Yield a set of contained named flags values."]
    #[doc = r""]
    #[doc = r" This method is like [`iter`](#method.iter), except only yields bits in contained named flags."]
    #[doc = r" Any unknown bits, or bits not corresponding to a contained flag will not be yielded."]
    #[inline]
    pub const fn iter_names(&self) -> SimpleFlagIterNames {
        SimpleFlagIterNames::new(self)
    }
    #[doc = r" Helper for formatting in human readable format. Write a flags value as text."]
    #[doc = r""]
    #[doc = r" Any bits that aren't part of a contained flag will be formatted as a hex number."]
    pub(crate) fn to_writer<W>(&self, mut writer: W) -> ::core::fmt::Result
    where
        W: ::core::fmt::Write,
    {
        let mut first = true;
        let mut iter = self.iter_names();
        for (name, _) in &mut iter {
            if !first {
                writer.write_str(" | ")?;
            }
            first = false;
            writer.write_str(name)?;
        }
        let remaining = iter.remaining();
        if !remaining.is_empty() {
            if !first {
                writer.write_str(" | ")?;
            }
            writer.write_fmt(core::format_args!("{:#X}", remaining.bits()))?;
        }
        ::core::fmt::Result::Ok(())
    }
    #[doc = r" Helper for formatting in human readable format. Write a flags value as text,"]
    #[doc = r" ignoring any unknown bits."]
    pub(crate) fn to_writer_truncate<W>(&self, writer: W) -> ::core::fmt::Result
    where
        W: ::core::fmt::Write,
    {
        self.truncate().to_writer(writer)
    }
    #[doc = r" Helper for formatting in human readable format. Write only the contained, defined,"]
    #[doc = r" named flags in a flags value as text."]
    pub(crate) fn to_writer_strict<W>(&self, mut writer: W) -> ::core::fmt::Result
    where
        W: ::core::fmt::Write,
    {
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
impl ::core::iter::Extend<SimpleFlag> for SimpleFlag {
    #[doc = r" Set all flags of `iter` to self"]
    fn extend<T: ::core::iter::IntoIterator<Item = Self>>(&mut self, iter: T) {
        for item in iter {
            self.set(item);
        }
    }
}
#[automatically_derived]
impl ::core::iter::FromIterator<SimpleFlag> for SimpleFlag {
    #[doc = r" Create a `#ty_name` from a iterator of flags."]
    fn from_iter<T: ::core::iter::IntoIterator<Item = Self>>(iter: T) -> Self {
        use ::core::iter::Extend;
        let mut res = Self::empty();
        res.extend(iter);
        res
    }
}
#[automatically_derived]
impl ::core::iter::IntoIterator for SimpleFlag {
    type Item = Self;
    type IntoIter = SimpleFlagIter;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[automatically_derived]
impl ::core::iter::IntoIterator for &SimpleFlag {
    type Item = SimpleFlag;
    type IntoIter = SimpleFlagIter;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[doc = r" An iterator over flags values."]
#[doc = r""]
#[doc = r" This iterator only yields flags values for contained, defined, named flags. Any remaining bits"]
#[doc = r" won't be yielded, but can be found with the [`#iter_name_ty::remaining`] method."]
pub struct SimpleFlagIterNames {
    flags: &'static [(&'static str, SimpleFlag)],
    index: usize,
    source: SimpleFlag,
    remaining: SimpleFlag,
}
impl SimpleFlagIterNames {
    pub(crate) const fn new(flags: &SimpleFlag) -> Self {
        Self {
            flags: SimpleFlag::FLAGS,
            index: 0,
            remaining: *flags,
            source: *flags,
        }
    }
    #[doc = r" Get a flags value of any remaining bits that haven't been yielded yet."]
    #[doc = r""]
    #[doc = r" Once the iterator has finished, this method can be used to"]
    #[doc = r" check whether or not there are any bits that didn't correspond"]
    #[doc = r" to a contained, defined, named flag remaining."]
    pub const fn remaining(&self) -> SimpleFlag {
        self.remaining
    }
}
#[automatically_derived]
impl ::core::iter::Iterator for SimpleFlagIterNames {
    type Item = (&'static str, SimpleFlag);
    fn next(&mut self) -> ::core::option::Option<Self::Item> {
        while let Some((name, flag)) = self.flags.get(self.index) {
            if self.remaining.is_empty() {
                return None;
            }
            self.index += 1;
            if self.source.contains(*flag) && self.remaining.intersects(*flag) {
                self.remaining.unset(*flag);
                return Some((name, *flag));
            }
        }
        None
    }
}
#[automatically_derived]
impl ::core::iter::FusedIterator for SimpleFlagIterNames {}

#[doc = r" An iterator over flags values."]
#[doc = r""]
#[doc = r" This iterator will yield flags values for contained, defined flags first, with any remaining bits yielded"]
#[doc = r" as a final flags value."]
pub struct SimpleFlagIter {
    inner: SimpleFlagIterNames,
    done: bool,
}
impl SimpleFlagIter {
    pub(crate) const fn new(flags: &SimpleFlag) -> Self {
        Self {
            inner: SimpleFlagIterNames::new(flags),
            done: false,
        }
    }
}
#[automatically_derived]
impl ::core::iter::Iterator for SimpleFlagIter {
    type Item = SimpleFlag;
    fn next(&mut self) -> ::core::option::Option<Self::Item> {
        match self.inner.next() {
            Some((_, flag)) => Some(flag),
            None if !self.done => {
                self.done = true;
                if !self.inner.remaining().is_empty() {
                    Some(self.inner.remaining)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
#[automatically_derived]
impl ::core::iter::FusedIterator for SimpleFlagIter {}
