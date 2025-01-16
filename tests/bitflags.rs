#![allow(mixed_script_confusables, clippy::module_inception)]
#[path = "bitflags/all.rs"]
mod all;
#[path = "bitflags/bitflag_match.rs"]
mod bitflag_match;
#[path = "bitflags/bits.rs"]
mod bits;
#[path = "bitflags/complement.rs"]
mod complement;
#[path = "bitflags/contains.rs"]
mod contains;
#[path = "bitflags/difference.rs"]
mod difference;
#[path = "bitflags/empty.rs"]
mod empty;
#[path = "bitflags/eq.rs"]
mod eq;
#[path = "bitflags/extend.rs"]
mod extend;
#[path = "bitflags/flags.rs"]
mod flags;
#[path = "bitflags/fmt.rs"]
mod fmt;
#[path = "bitflags/from_bits.rs"]
mod from_bits;
#[path = "bitflags/from_bits_retain.rs"]
mod from_bits_retain;
#[path = "bitflags/from_bits_truncate.rs"]
mod from_bits_truncate;
#[path = "bitflags/from_name.rs"]
mod from_name;
// #[path = "bitflags/insert.rs"]
// mod insert;
#[path = "bitflags/intersection.rs"]
mod intersection;
#[path = "bitflags/intersects.rs"]
mod intersects;
#[path = "bitflags/is_all.rs"]
mod is_all;
#[path = "bitflags/is_empty.rs"]
mod is_empty;
#[path = "bitflags/iter.rs"]
mod iter;
#[path = "bitflags/parser.rs"]
mod parser;
// #[path = "bitflags/remove.rs"]
// mod remove;
#[path = "bitflags/symmetric_difference.rs"]
mod symmetric_difference;
#[path = "bitflags/truncate.rs"]
mod truncate;
#[path = "bitflags/union.rs"]
mod union;
#[path = "bitflags/unknown.rs"]
mod unknown;

use bitflag_attr::bitflag;

#[bitflag(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestFlags {
    A = 1,
    B = 1 << 1,
    C = 1 << 2,
    ABC = A | B | C,
}

#[bitflag(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestFlagsInvert {
    ABC = A | B | C,
    A = 1,
    B = 1 << 1,
    C = 1 << 2,
}

#[bitflag(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestZero {
    ZERO = 0,
}

#[bitflag(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestZeroOne {
    ZERO = 0,
    ONE = 1,
}

#[bitflag(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestUnicode {
    一 = 1,
    二 = 1 << 1,
}

#[bitflag(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestEmpty {}

#[bitflag(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestOverlapping {
    AB = 1 | (1 << 1),
    BC = (1 << 1) | (1 << 2),
}

#[bitflag(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestOverlappingFull {
    A = 1,
    B = 1,
    C = 1,
    D = 1 << 1,
}

#[bitflag(u8)]
#[non_exhaustive] // External = !0
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestExternal {
    A = 1,
    B = 1 << 1,
    C = 1 << 2,
    ABC = A | B | C,
}

#[bitflag(u8)]
#[non_exhaustive] // External = !0
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestExternalFull {}
