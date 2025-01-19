use bitflag_attr::bitflag;

#[bitflag(u32)]
#[derive(Clone, Copy, Pod, Zeroable)]
enum Color {
    RED = 0x1,
    GREEN = 0x02,
    BLUE = 0x4,
}

#[test]
fn bytemuck_works() {
    assert_eq!(0x1, bytemuck::cast::<Color, u32>(Color::RED));
}
