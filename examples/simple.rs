use bitflag_attr::bitflag;

const CONST1: u32 = 0b10;
const CONST2: u32 = 0b100;

mod namespaced {
    pub const CONST3: u32 = 0b1000;
    pub const FLAG1: u32 = super::ExampleFlags::Flag1.bits();
}

/// A example bitflag
#[bitflag(u32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ExampleFlags {
    Flag1 = 1 << 9,
    Flag2 = 1 << 12,
    Flag3 = CONST1,
    Flag4 = !CONST1,
    Flag5 = CONST1 | CONST2 | 3,
    Flag6 = Flag1 | Flag2,
    Flag7 = CONST1 | Flag1,
    Flag8 = (1 << 1) | (1 << 4),
    Flag9 = 1u8 as u32,
    Flag10 = ExampleFlags::Flag1.bits() | ExampleFlags::Flag4.bits(),
    Flag11 = namespaced::CONST3,
    Flag12 = namespaced::CONST3 & namespaced::FLAG1,
}

fn main() {
    let flag = ExampleFlags::Flag1 | ExampleFlags::Flag2;

    assert_eq!(flag, ExampleFlags::Flag6);

    println!("Binary: 0b{flag:b}");
    println!();
    println!("Octal: 0o{flag:o}");
    println!();
    println!("Hex: 0x{flag:X}");
    println!();
    println!("Debug: {flag:?}");
    println!();
    println!("Debug Pretty: {flag:#?}");
}
