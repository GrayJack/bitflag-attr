use bitflag_attr::bitflag;

use arbitrary::Arbitrary;

#[bitflag(u32)]
#[derive(Clone, Copy, Arbitrary)]
enum Color {
    RED = 0x1,
    GREEN = 0x02,
    BLUE = 0x4,
}

#[test]
fn arbitrary_works() {
    let mut unstructured = arbitrary::Unstructured::new(&[0_u8; 256]);
    let _color = Color::arbitrary(&mut unstructured);
}
