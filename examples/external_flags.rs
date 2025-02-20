//! An example of defining an external flags definition using the [`bitflag`] macro.
use bitflag_attr::bitflag;

// If you're generating flags types from an external source, such as a C API, you can use the
// `#[non_exhaustive]` attribute to communicate to the bitflags macro that there may be more valid
// flags then the known flags.
//
// Without extra configuration, it defaults to `!0` (all bits set) as a mask of all bits the
/// external source may ever set, i.e. all bits are considered as possible values. But a value can
/// be defined using the `#[reserved_bits = <value>]` helper attribute.
#[bitflag(u32)]
#[non_exhaustive]
#[reserved_bits = 0b001001111]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SimpleFlag {
    A = 1,
    B = 1 << 1,
    C = 1 << 2,
    D = 1 << 3,
    E = 1 << 4,
    F = A | B,
}

fn main() {
    // A flag with only named flags
    let mut flag = SimpleFlag::A | SimpleFlag::B | SimpleFlag::C;

    // We added a potential external value
    flag.set(SimpleFlag::from_bits_retain(1 << 6));

    println!("{:#?}", flag);

    // We truncate the value, but all potential external value will remain
    flag.truncate();

    println!("{:#?}", flag);
}
