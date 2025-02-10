use super::*;

#[test]
fn cases() {
    case(
        TestFlags::empty(),
        "TestFlags { flags: 0x0, bits: 0b00000000, octal: 0o000, hex: 0x00 }",
        "0",
        "0",
        "0",
        "0",
    );
    case(
        TestFlags::A,
        "TestFlags { flags: A, bits: 0b00000001, octal: 0o001, hex: 0x01 }",
        "1",
        "1",
        "1",
        "1",
    );
    case(
        TestFlags::all(),
        "TestFlags { flags: A | B | C, bits: 0b00000111, octal: 0o007, hex: 0x07 }",
        "7",
        "7",
        "7",
        "111",
    );
    case(
        TestFlags::from_bits_retain(1 << 3),
        "TestFlags { flags: 0x8, bits: 0b00001000, octal: 0o010, hex: 0x08 }",
        "8",
        "8",
        "10",
        "1000",
    );
    case(
        TestFlags::A | TestFlags::from_bits_retain(1 << 3),
        "TestFlags { flags: A | 0x8, bits: 0b00001001, octal: 0o011, hex: 0x09 }",
        "9",
        "9",
        "11",
        "1001",
    );

    case(
        TestZero::ZERO,
        "TestZero { flags: 0x0, bits: 0b00000000, octal: 0o000, hex: 0x00 }",
        "0",
        "0",
        "0",
        "0",
    );
    case(
        TestZero::ZERO | TestZero::from_bits_retain(1),
        "TestZero { flags: 0x1, bits: 0b00000001, octal: 0o001, hex: 0x01 }",
        "1",
        "1",
        "1",
        "1",
    );

    case(
        TestZeroOne::ONE,
        "TestZeroOne { flags: ONE, bits: 0b00000001, octal: 0o001, hex: 0x01 }",
        "1",
        "1",
        "1",
        "1",
    );

    case(
        TestOverlapping::from_bits_retain(1 << 1),
        "TestOverlapping { flags: 0x2, bits: 0b00000010, octal: 0o002, hex: 0x02 }",
        "2",
        "2",
        "2",
        "10",
    );

    case(
        TestExternal::from_bits_retain(1 | (1 << 1) | (1 << 3)),
        "TestExternal { flags: A | B | 0x8, bits: 0b00001011, octal: 0o013, hex: 0x0B }",
        "B",
        "b",
        "13",
        "1011",
    );

    case(
        TestExternal::all(),
        "TestExternal { flags: A | B | C | 0xF8, bits: 0b11111111, octal: 0o377, hex: 0xFF }",
        "FF",
        "ff",
        "377",
        "11111111",
    );

    case(
        TestExternalFull::all(),
        "TestExternalFull { flags: 0xFF, bits: 0b11111111, octal: 0o377, hex: 0xFF }",
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
