use super::*;

#[test]
fn cases() {
    case(
        TestFlags::empty(),
        "TestFlags { bits: 0b00000000, human_readable: 0x0 }",
        "0",
        "0",
        "0",
        "0",
    );
    case(
        TestFlags::A,
        "TestFlags { bits: 0b00000001, human_readable: A }",
        "1",
        "1",
        "1",
        "1",
    );
    case(
        TestFlags::all(),
        "TestFlags { bits: 0b00000111, human_readable: A | B | C }",
        "7",
        "7",
        "7",
        "111",
    );
    case(
        TestFlags::from_bits_retain(1 << 3),
        "TestFlags { bits: 0b00001000, human_readable: 0x8 }",
        "8",
        "8",
        "10",
        "1000",
    );
    case(
        TestFlags::A | TestFlags::from_bits_retain(1 << 3),
        "TestFlags { bits: 0b00001001, human_readable: A | 0x8 }",
        "9",
        "9",
        "11",
        "1001",
    );

    case(
        TestZero::ZERO,
        "TestZero { bits: 0b00000000, human_readable: 0x0 }",
        "0",
        "0",
        "0",
        "0",
    );
    case(
        TestZero::ZERO | TestZero::from_bits_retain(1),
        "TestZero { bits: 0b00000001, human_readable: 0x1 }",
        "1",
        "1",
        "1",
        "1",
    );

    case(
        TestZeroOne::ONE,
        "TestZeroOne { bits: 0b00000001, human_readable: ONE }",
        "1",
        "1",
        "1",
        "1",
    );

    case(
        TestOverlapping::from_bits_retain(1 << 1),
        "TestOverlapping { bits: 0b00000010, human_readable: 0x2 }",
        "2",
        "2",
        "2",
        "10",
    );

    case(
        TestExternal::from_bits_retain(1 | (1 << 1) | (1 << 3)),
        "TestExternal { bits: 0b00001011, human_readable: A | B | 0x8 }",
        "B",
        "b",
        "13",
        "1011",
    );

    case(
        TestExternal::all(),
        "TestExternal { bits: 0b11111111, human_readable: A | B | C | 0xF8 }",
        "FF",
        "ff",
        "377",
        "11111111",
    );

    case(
        TestExternalFull::all(),
        "TestExternalFull { bits: 0b11111111, human_readable: 0xFF }",
        "FF",
        "ff",
        "377",
        "11111111",
    );
}

#[track_caller]
fn case<T>(value: T, debug: &str, uhex: &str, lhex: &str, oct: &str, bin: &str)
where
    T: std::fmt::Debug
        + std::fmt::UpperHex
        + std::fmt::LowerHex
        + std::fmt::Octal
        + std::fmt::Binary,
{
    assert_eq!(debug, format!("{:?}", value));
    assert_eq!(uhex, format!("{:X}", value));
    assert_eq!(lhex, format!("{:x}", value));
    assert_eq!(oct, format!("{:o}", value));
    assert_eq!(bin, format!("{:b}", value));
}
