//! An example of the generated code from the following code:
//!
//! ```
//! use bitflag_attr::bitflag;
//!
//! #[bitflag(u32)]
//! #[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
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

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Flags(u32);

#[doc = " The value `A`, at bit position `0`."]
#[allow(non_upper_case_globals)]
const A: u32 = 0b00000001;
#[doc = " The value `B`, at bit position `1`."]
#[allow(non_upper_case_globals)]
const B: u32 = 0b00000010;
#[doc = " The value `C`, at bit position `2`."]
#[allow(non_upper_case_globals)]
const C: u32 = 0b00000100;
#[doc = " The combination of `A`, `B`, and `C`."]
#[allow(non_upper_case_globals)]
const ABC: u32 = A | B | C;
#[allow(non_upper_case_globals)]
impl Flags {
    #[doc = " The value `A`, at bit position `0`."]
    pub const A: Flags = Self(0b00000001);
    #[doc = " The value `B`, at bit position `1`."]
    pub const B: Flags = Self(0b00000010);
    #[doc = " The value `C`, at bit position `2`."]
    pub const C: Flags = Self(0b00000100);
    #[doc = " The combination of `A`, `B`, and `C`."]
    pub const ABC: Flags = Self(A | B | C);
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
    #[doc = r" Returns a bitflag that constains all value."]
    #[doc = r""]
    #[doc = r" This will include bits that do not have any flags/meaning."]
    #[doc = r" Use [`all`](Self::all) if you want only the specified flags set."]
    #[inline]
    pub const fn all_bits() -> Self {
        Self(!0)
    }
    #[doc = r" Returns `true` if the bitflag constains all value bits set."]
    #[doc = r""]
    #[doc = r" This will check for all bits."]
    #[doc = r" Use [`is_all`](Self::is_all) if you want to check for all specified flags."]
    #[inline]
    pub const fn is_all_bits(&self) -> bool {
        self.0 == !0
    }
    #[doc = r" Construct a bitflag with all flags set."]
    #[doc = r""]
    #[doc = r" This will only set the flags specified as associated constant."]
    #[inline]
    pub const fn all() -> Self {
        Self(Self::A.0 | Self::B.0 | Self::C.0 | Self::ABC.0 | 0)
    }
    #[doc = r" Returns `true` if the bitflag contais all flags."]
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
    pub fn set(&mut self, other: Self) {
        self.0 = self.and(other).0
    }
    #[doc = r" Unset the flags in `other` in the value."]
    #[inline]
    pub fn unset(&mut self, other: Self) {
        self.0 = self.difference(other).0
    }
    #[doc = r" Toggle the flags in `other` in the value."]
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 = self.xor(other).0
    }
}
impl core::ops::Not for Flags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        self.complement()
    }
}
impl core::ops::BitAnd for Flags {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}
impl core::ops::BitOr for Flags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}
impl core::ops::BitXor for Flags {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}
impl core::ops::BitAndAssign for Flags {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        core::ops::BitAndAssign::bitand_assign(&mut self.0, rhs.0)
    }
}
impl core::ops::BitOrAssign for Flags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        core::ops::BitOrAssign::bitor_assign(&mut self.0, rhs.0)
    }
}
impl core::ops::BitXorAssign for Flags {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        core::ops::BitXorAssign::bitxor_assign(&mut self.0, rhs.0)
    }
}
impl core::ops::Sub for Flags {
    type Output = Self;
    #[doc = r" The intersection of a source flag with the complement of a target flags value"]
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}
impl core::ops::SubAssign for Flags {
    #[doc = r" The intersection of a source flag with the complement of a target flags value"]
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.unset(rhs)
    }
}
impl From<u32> for Flags {
    #[inline]
    fn from(val: u32) -> Self {
        Self::from_bits_truncate(val)
    }
}
impl From<Flags> for u32 {
    #[inline]
    fn from(val: Flags) -> Self {
        val.0
    }
}
impl core::fmt::Binary for Flags {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Binary::fmt(&self.0, f)
    }
}
impl core::fmt::LowerHex for Flags {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(&self.0, f)
    }
}
impl core::fmt::UpperHex for Flags {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::UpperHex::fmt(&self.0, f)
    }
}
impl core::fmt::Octal for Flags {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Octal::fmt(&self.0, f)
    }
}
impl core::fmt::Debug for Flags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = "Flags";
        if f.alternate() {
            f.write_fmt(format_args!("{} ", &name))?;
            let mut tmp = f.debug_map();
            if self.contains(Self::A) {
                tmp.entry(&"A", &"set");
            } else {
                tmp.entry(&"A", &"unset");
            }
            if self.contains(Self::B) {
                tmp.entry(&"B", &"set");
            } else {
                tmp.entry(&"B", &"unset");
            }
            if self.contains(Self::C) {
                tmp.entry(&"C", &"set");
            } else {
                tmp.entry(&"C", &"unset");
            }
            if self.contains(Self::ABC) {
                tmp.entry(&"ABC", &"set");
            } else {
                tmp.entry(&"ABC", &"unset");
            }
            tmp.finish()
        } else {
            f.debug_tuple(&name).field(&self.0).finish()
        }
    }
}
