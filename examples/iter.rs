use bitflag_attr::bitflag;

#[bitflag(u32)]
#[non_exhaustive]
#[extra_valid_bits = 0b001001111]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SimpleFlag {
    Flag1 = 1 << 9,
    Flag2 = 1 << 12,
    Flag3 = 1,
    Flag4 = Flag1 | Flag2,
}

fn main() {
    let mut flag = SimpleFlag::Flag1 | SimpleFlag::Flag2 | SimpleFlag::Flag3;

    flag.set(SimpleFlag::from_bits_retain(1 << 5));

    println!("{:#?}", flag);

    for i in flag.iter_names() {
        println!("{i:?}");
    }
}
