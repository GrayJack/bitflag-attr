use bitflag_attr::bitflag;

#[bitflag(u32, no_auto_debug)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SimpleFlag {
    Flag1 = 1 << 9,
    Flag2 = 1 << 12,
    Flag3 = Flag1 | Flag2,
}

fn main() {
    let flag = SimpleFlag::Flag1 | SimpleFlag::Flag2;
    println!("Binary: 0b{flag:b}");
    println!();
    println!("Octal: 0o{flag:o}");
    println!();
    println!("Hex: 0x{flag:X}");
}
