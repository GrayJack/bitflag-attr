use bitflag_attr::bitflag;

#[bitflag(u32)]
#[derive(Debug, Clone, Copy,PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Flags {
    /// The value `A`, at bit position `0`.
    A = 0b00000001,
    /// The value `B`, at bit position `1`.
    B = 0b00000010,
    /// The value `C`, at bit position `2`.
    C = 0b00000100,

    /// The combination of `A`, `B`, and `C`.
    ABC = A | B | C,
}

fn main() {
    let e1 = Flags::A | Flags::C;
    let e2 = Flags::B | Flags::C;
    assert_eq!((e1 | e2), Flags::ABC); // union
    assert_eq!((e1 & e2), Flags::C); // intersection
    assert_eq!((e1 - e2), Flags::A); // set difference
    assert_eq!(!e2, Flags::A); // set complement
}
