//! An example on deriving the Default trait behavior when using with the [`bitflag`] macro.
use bitflag_attr::bitflag;

// You can add the Default trait to the list of derived traits. Doing so will make the default
// value to be an empty flag.
#[bitflag(u16)]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
enum DefaultFlags {
    A = 1,
    B = 1 << 1,
    C = 1 << 2,
}

// But you can also choose one of your defined flags as the default value with the `#[default]`
// helper attribute, like in any enum definition.
#[bitflag(u16)]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
enum DefaultFlags2 {
    D = 1,
    #[default]
    E = 1 << 1,
    F = 1 << 2,
}

fn main() {
    let default1 = DefaultFlags::default();
    assert!(default1.is_empty());

    let default2 = DefaultFlags2::default();
    assert_eq!(default2, DefaultFlags2::E);
}
