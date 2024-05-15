use bitflag_attr::bitflag;

#[bitflag(u32)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SimpleFlag {
    Flag1 = 1 << 9,
    Flag2 = 1 << 12,
    Flag3 = Flag1 | Flag2,
}

fn main() {
    let flag = SimpleFlag::Flag1 | SimpleFlag::Flag2;

    assert_eq!(flag, SimpleFlag::Flag3);

    println!("Binary: {flag:b}");
    println!();
    println!("Octal: {flag:o}");
    println!();
    println!("Hex: {flag:X}");
    println!();
    println!("Debug: {flag:?}");
    println!();
    println!("Debug Pretty: {flag:#?}");
}
