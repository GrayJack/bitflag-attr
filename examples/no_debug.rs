use bitflag_attr::bitflag;

#[bitflag(u32)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SimpleFlags {
    Flag1 = 1 << 9,
    Flag2 = 1 << 12,
    Flag3 = Flag1 | Flag2,
}

fn main() {
    let flag = SimpleFlags::Flag1 | SimpleFlags::Flag2;
    println!("Binary: 0b{flag:b}");
    println!();
    println!("Octal: 0o{flag:o}");
    println!();
    println!("Hex: 0x{flag:X}");
}
