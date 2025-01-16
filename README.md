# bitflag-attr

[![Rust](https://github.com/GrayJack/bitflag-attr/workflows/Check/badge.svg)](https://github.com/GrayJack/bitflag-attr/actions)
[![Latest version](https://img.shields.io/crates/v/bitflag-attr.svg)](https://crates.io/crates/bitflag-attr)
[![Documentation](https://docs.rs/bitflag-attr/badge.svg)](https://docs.rs/bitflag-attr)
![License](https://img.shields.io/crates/l/bitflag-attr.svg)

This is a proc-macro Rust crate that allows to turn a C-like enum into a bitflag types with an ergonomic end-user API.

You can use this crate to:

- Provide more user-friendly bindings to C APIs where flags may or may not be fully known in advance.
- Generate efficient options types with string parsing and formatting support.

You **can't** use this crate to:

- Guarantee only bits corresponding to defined flags will ever be set. `bitflag-attr` allows access to the underlying bits type so arbitrary bits may be set.
- Define bitfields. `bitflag-attr` only generates types where set bits denote the presence of some combination of flags.

- [Documentation](https://docs.rs/bitflag-attr)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
bitflag_attr = "0.9.0"
```

and this to your source code:

```rust
use bitflag_attr::bitflag;
```

## Quick Example

Generate a flags structure:

```rust
use bitflag_attr::bitflag;

#[bitflag(u32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

If you don't want `Debug` trait to be generated, you can simply not define it on the derive attribute.

```rust
use bitflag_attr::bitflag;

#[bitflag(u32)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

## Features

- [X] Use enum native syntax to define individual known flags
- [X] Discriminant values must be defined
- [X] Generated end-user API almost entirely the same to `bitflags` crate
- [X] Most of the generated type-associated API is `const`-compatible (entirely if `const-mut-ref` feature flag enabled)
- [X] Debug formatter outputs both the binary representation and named flag representation
- [X] Optional support for serialization with
- [X] Compatible with `#[no_std]`

### Implemented traits

The macro requires that `Clone` and `Copy` are derived.

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
- [X] core::fmt::Debug (if on the derive macro list)
- [X] core::fmt::Binary
- [X] core::fmt::UpperHex
- [X] core::fmt::LowerHex
- [X] core::fmt::Octal
- [X] core::str::FromStr
- [X] core::iter::Extend
- [X] core::iter::FromIterator
- [X] core::iter::IntoIterator (for the type and reference)
- [X] From

If the `Debug` trait is defined in the `#[derive(...)]` attribute. The macro will produce a custom implementation instead of the one Rust std produces

There is a opt-in crate feature `serde` that generate a parsing error type and implements the traits:

- [X] serde::Serialize
- [X] serde::Deserialize

The custom implementation for `Serialize` and `Deserialize` will be generated only if those traits are in the `#[derive(...)]` attribute list (similar how the `Debug` works).

**Note:** This crate does not import/re-export serde traits, your project MUST have `serde` as dependency.

### Const mut ref

Most of the associated function generated for the flags type are `const`-compatible, with exceptions with the one that takes `&mut self`.

If you are on Rust version 1.83.0 or superior, you can enable the `const-mut-ref` feature flag to make those function to also be `const`-compatible.

## Alternatives

- [bitflags](https://crates.io/crates/bitflags): The OG of Rust ecosystem
- [enumflags2](https://crates.io/crates/enumflags2):

## Rust Version Support

The minimum supported Rust version is documented in the `Cargo.toml` file.
This may be bumped in minor releases as necessary.