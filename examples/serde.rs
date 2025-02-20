use bitflag_attr::bitflag;

#[bitflag(u32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub enum SerdeFlags {
    Flag1 = 1 << 9,
    Flag2 = 1 << 12,
    Flag3 = 1,
    Flag4 = Flag1 | Flag2,
}

fn main() {
    let mut flag = SerdeFlags::Flag1 | SerdeFlags::Flag2 | SerdeFlags::Flag3;

    flag.set(SerdeFlags::from_bits_retain(1 << 5));

    println!("{:#?}", flag);

    for i in flag.iter_names() {
        println!("{i:?}");
    }
}
