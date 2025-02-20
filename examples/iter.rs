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
    let flag = SimpleFlags::Flag1 | SimpleFlags::Flag2 | SimpleFlags::Flag3;

    println!("{:#?}", flag);

    for i in flag.iter_names() {
        println!("{i:?}");
    }
}
