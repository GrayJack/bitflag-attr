use bitflag_attr::bitflag;

#[bitflag(u32)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SimpleFlag {
    Flag1 = 1 << 9,
    Flag2 = 1 << 12,
    Flag3 = 1,
    Flag4 = Flag1 | Flag2,
}

fn main() {
    let flag = SimpleFlag::Flag1 | SimpleFlag::Flag2 | SimpleFlag::Flag3;

    for i in flag.iter_names() {
        println!("{i:?}");
    }
}
