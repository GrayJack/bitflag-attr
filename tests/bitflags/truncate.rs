use super::*;

use bitflag_attr::Flags;

#[test]
fn cases() {
    case(
        TestFlags::ABC | TestFlags::from_bits_retain(1 << 3),
        TestFlags::ABC,
    );

    case(TestZero::empty(), TestZero::empty());

    case(TestZero::all(), TestZero::all());

    case(
        TestFlags::from_bits_retain(1 << 3) | TestFlags::all(),
        TestFlags::all(),
    );
}

#[track_caller]
fn case<T>(mut before: T, after: T)
where
    T: Flags + std::fmt::Debug + PartialEq + Copy,
{
    before.truncate();
    assert_eq!(before, after, "{:?}.truncate()", before);
}
