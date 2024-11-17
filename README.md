# bitflag-attr

[![Rust](https://github.com/GrayJack/bitflag-attr/workflows/Check/badge.svg)](https://github.com/GrayJack/bitflag-attr/actions)
[![Latest version](https://img.shields.io/crates/v/bitflag-attr.svg)](https://crates.io/crates/bitflag-attr)
[![Documentation](https://docs.rs/bitflag-attr/badge.svg)](https://docs.rs/bitflag-attr)
![License](https://img.shields.io/crates/l/bitflag-attr.svg)

This is a proc-macro Rust crate that allows to turn a C-like enum into a bitflag structures with an API similar to `bitfields` crate.

You can use this crate to:

- provide more user-friendly bindings to C APIs where flags may or may not be fully known in advance.

You can't use this crate to:

- guarantee only bits corresponding to defined flags will ever be set. `bitflag-attr` allows access to the underlying bits type so arbitrary bits may be set.
- define bitfields. `bitflag-attr` only generates types where set bits denote the presence of some combination of flags.

## Implemented traits

The macro will also implement some traits for bitwise operations and formatting.

- [X] core::ops::Not
- [X] core::ops::BitAnd
- [X] core::ops::BitOr
- [X] core::ops::BitXor
- [X] core::ops::BitAndAssign
- [X] core::ops::BitOrAssign
- [X] core::ops::BitXorAssign
- [X] core::ops::Sub
- [X] core::ops::SubAssign
- [X] core::fmt::Debug (This can be opt-out with the `no_auto_debug`)
- [X] core::fmt::Binary
- [X] core::fmt::UpperHex
- [X] core::fmt::LowerHex
- [X] core::fmt::Octal
- [X] From
- [X] Clone
- [X] Copy

Besides the `Debug`, `Clone` and `Copy` traits, all other derivable traits can be used together with the type.

## Example

Generate a flags structure:

```rust
use bitflag_attr::bitflag;

#[bitflag(u32)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Flags {
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
    assert_eq!((e1 | e2), Flags::ABC);   // union
    assert_eq!((e1 & e2), Flags::C);     // intersection
    assert_eq!((e1 - e2), Flags::A);     // set difference
    assert_eq!(!e2, Flags::A);           // set complement
}
```

If you don't want `Debug` trait to be generated, you can pass `no_auto_debug` to the attribute.

```rust
use bitflag_attr::bitflag;

#[bitflag(u32, no_auto_debug)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Flags {
    /// The value `A`, at bit position `0`.
    A = 0b00000001,
    /// The value `B`, at bit position `1`.
    B = 0b00000010,
    /// The value `C`, at bit position `2`.
    C = 0b00000100,

    /// The combination of `A`, `B`, and `C`.
    ABC = A | B | C,
}
```


## Rust Version Support

The minimum supported Rust version is documented in the `Cargo.toml` file.
This may be bumped in minor releases as necessary.