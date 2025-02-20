use bitflag_attr::bitflag;

#[bitflag(u32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SimpleFlags {
    Flag1 = 1 << 9,
    Flag2 = 1 << 12,
    Flag3 = 1,
    Flag4 = Flag1 | Flag2,
}

fn main() {
    let mut flag = SimpleFlags::Flag1 | SimpleFlags::Flag2 | SimpleFlags::Flag3;

    flag.set(SimpleFlags::from_bits_retain(1 << 5));

    println!("{:#?}", flag);
}
