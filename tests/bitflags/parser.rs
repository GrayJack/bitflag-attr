use super::*;

use bitflag_attr::{parser::*, Flags};

#[test]
#[cfg(not(miri))] // Very slow in miri
fn roundtrip() {
    let mut s = String::new();

    for a in 0u8..=255 {
        for b in 0u8..=255 {
            let f = TestFlags::from_bits_retain(a | b);

            s.clear();
            to_writer(&f, &mut s).unwrap();

            assert_eq!(f, from_text::<TestFlags>(&s).unwrap());
        }
    }
}

#[test]
#[cfg(not(miri))] // Very slow in miri
fn roundtrip_truncate() {
    let mut s = String::new();

    for a in 0u8..=255 {
        for b in 0u8..=255 {
            let f = TestFlags::from_bits_retain(a | b);

            s.clear();
            to_writer_truncate(&f, &mut s).unwrap();

            assert_eq!(
                TestFlags::from_bits_truncate(f.bits()),
                from_text_truncate::<TestFlags>(&s).unwrap()
            );
        }
    }
}

#[test]
#[cfg(not(miri))] // Very slow in miri
fn roundtrip_strict() {
    let mut s = String::new();

    for a in 0u8..=255 {
        for b in 0u8..=255 {
            let f = TestFlags::from_bits_retain(a | b);

            s.clear();
            to_writer_strict(&f, &mut s).unwrap();

            let mut strict = TestFlags::empty();
            for (_, flag) in f.iter_names() {
                strict |= flag;
            }
            let f = strict;

            if let Ok(s) = from_text_strict::<TestFlags>(&s) {
                assert_eq!(f, s);
            }
        }
    }
}

mod from_text {
    use super::*;

    #[test]
    fn valid() {
        assert_eq!(0, from_text::<TestFlags>("").unwrap().bits());

        assert_eq!(1, from_text::<TestFlags>("A").unwrap().bits());
        assert_eq!(1, from_text::<TestFlags>(" A ").unwrap().bits());
        assert_eq!(
            1 | (1 << 1) | (1 << 2),
            from_text::<TestFlags>("A | B | C").unwrap().bits()
        );
        assert_eq!(
            1 | (1 << 1) | (1 << 2),
            from_text::<TestFlags>("A\n|\tB\r\n|   C ").unwrap().bits()
        );
        assert_eq!(
            1 | (1 << 1) | (1 << 2),
            from_text::<TestFlags>("A|B|C").unwrap().bits()
        );

        assert_eq!(1 << 3, from_text::<TestFlags>("0x8").unwrap().bits());
        assert_eq!(
            1 | (1 << 3),
            from_text::<TestFlags>("A | 0x8").unwrap().bits()
        );
        assert_eq!(
            1 | (1 << 1) | (1 << 3),
            from_text::<TestFlags>("0x1 | 0x8 | B").unwrap().bits()
        );

        assert_eq!(
            1 | (1 << 1),
            from_text::<TestUnicode>("一 | 二").unwrap().bits()
        );
    }

    #[test]
    fn invalid() {
        assert!(from_text::<TestFlags>("a")
            .unwrap_err()
            .to_string()
            .starts_with("unrecognized named flag"));
        assert!(from_text::<TestFlags>("A & B")
            .unwrap_err()
            .to_string()
            .starts_with("unrecognized named flag"));

        assert!(from_text::<TestFlags>("0xg")
            .unwrap_err()
            .to_string()
            .starts_with("invalid hex flag"));
        assert!(from_text::<TestFlags>("0xffffffffffff")
            .unwrap_err()
            .to_string()
            .starts_with("invalid hex flag"));
    }
}

mod to_writer {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!("", write(TestFlags::empty()));
        assert_eq!("A", write(TestFlags::A));
        assert_eq!("A | B | C", write(TestFlags::all()));
        assert_eq!("0x8", write(TestFlags::from_bits_retain(1 << 3)));
        assert_eq!(
            "A | 0x8",
            write(TestFlags::A | TestFlags::from_bits_retain(1 << 3))
        );

        assert_eq!("", write(TestZero::ZERO));

        assert_eq!("ABC", write(TestFlagsInvert::all()));

        assert_eq!("0x1", write(TestOverlapping::from_bits_retain(1)));

        assert_eq!("A", write(TestOverlappingFull::C));
        assert_eq!(
            "A | D",
            write(TestOverlappingFull::C | TestOverlappingFull::D)
        );
    }

    fn write<F: Flags>(value: F) -> String {
        let mut s = String::new();

        to_writer(&value, &mut s).unwrap();
        s
    }
}

mod from_text_truncate {
    use super::*;

    #[test]
    fn valid() {
        assert_eq!(0, from_text_truncate::<TestFlags>("").unwrap().bits());

        assert_eq!(1, from_text_truncate::<TestFlags>("A").unwrap().bits());
        assert_eq!(1, from_text_truncate::<TestFlags>(" A ").unwrap().bits());
        assert_eq!(
            1 | (1 << 1) | (1 << 2),
            from_text_truncate::<TestFlags>("A | B | C").unwrap().bits()
        );
        assert_eq!(
            1 | (1 << 1) | (1 << 2),
            from_text_truncate::<TestFlags>("A\n|\tB\r\n|   C ")
                .unwrap()
                .bits()
        );
        assert_eq!(
            1 | (1 << 1) | (1 << 2),
            from_text_truncate::<TestFlags>("A|B|C").unwrap().bits()
        );

        assert_eq!(0, from_text_truncate::<TestFlags>("0x8").unwrap().bits());
        assert_eq!(
            1,
            from_text_truncate::<TestFlags>("A | 0x8").unwrap().bits()
        );
        assert_eq!(
            1 | (1 << 1),
            from_text_truncate::<TestFlags>("0x1 | 0x8 | B")
                .unwrap()
                .bits()
        );

        assert_eq!(
            1 | (1 << 1),
            from_text_truncate::<TestUnicode>("一 | 二").unwrap().bits()
        );
    }
}

mod to_writer_truncate {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!("", write(TestFlags::empty()));
        assert_eq!("A", write(TestFlags::A));
        assert_eq!("A | B | C", write(TestFlags::all()));
        assert_eq!("", write(TestFlags::from_bits_retain(1 << 3)));
        assert_eq!(
            "A",
            write(TestFlags::A | TestFlags::from_bits_retain(1 << 3))
        );

        assert_eq!("", write(TestZero::ZERO));

        assert_eq!("ABC", write(TestFlagsInvert::all()));

        assert_eq!("0x1", write(TestOverlapping::from_bits_retain(1)));

        assert_eq!("A", write(TestOverlappingFull::C));
        assert_eq!(
            "A | D",
            write(TestOverlappingFull::C | TestOverlappingFull::D)
        );
    }

    fn write<F: Flags>(value: F) -> String {
        let mut s = String::new();

        to_writer_truncate(&value, &mut s).unwrap();
        s
    }
}

mod from_text_strict {
    use super::*;

    #[test]
    fn valid() {
        assert_eq!(0, from_text_strict::<TestFlags>("").unwrap().bits());

        assert_eq!(1, from_text_strict::<TestFlags>("A").unwrap().bits());
        assert_eq!(1, from_text_strict::<TestFlags>(" A ").unwrap().bits());
        assert_eq!(
            1 | (1 << 1) | (1 << 2),
            from_text_strict::<TestFlags>("A | B | C").unwrap().bits()
        );
        assert_eq!(
            1 | (1 << 1) | (1 << 2),
            from_text_strict::<TestFlags>("A\n|\tB\r\n|   C ")
                .unwrap()
                .bits()
        );
        assert_eq!(
            1 | (1 << 1) | (1 << 2),
            from_text_strict::<TestFlags>("A|B|C").unwrap().bits()
        );

        assert_eq!(
            1 | (1 << 1),
            from_text_strict::<TestUnicode>("一 | 二").unwrap().bits()
        );
    }

    #[test]
    fn invalid() {
        assert!(from_text_strict::<TestFlags>("a")
            .unwrap_err()
            .to_string()
            .starts_with("unrecognized named flag"));
        assert!(from_text_strict::<TestFlags>("A & B")
            .unwrap_err()
            .to_string()
            .starts_with("unrecognized named flag"));

        assert!(from_text_strict::<TestFlags>("0x1")
            .unwrap_err()
            .to_string()
            .starts_with("invalid hex flag"));
        assert!(from_text_strict::<TestFlags>("0xg")
            .unwrap_err()
            .to_string()
            .starts_with("invalid hex flag"));
        assert!(from_text_strict::<TestFlags>("0xffffffffffff")
            .unwrap_err()
            .to_string()
            .starts_with("invalid hex flag"));
    }
}

mod to_writer_strict {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!("", write(TestFlags::empty()));
        assert_eq!("A", write(TestFlags::A));
        assert_eq!("A | B | C", write(TestFlags::all()));
        assert_eq!("", write(TestFlags::from_bits_retain(1 << 3)));
        assert_eq!(
            "A",
            write(TestFlags::A | TestFlags::from_bits_retain(1 << 3))
        );

        assert_eq!("", write(TestZero::ZERO));

        assert_eq!("ABC", write(TestFlagsInvert::all()));

        assert_eq!("", write(TestOverlapping::from_bits_retain(1)));

        assert_eq!("A", write(TestOverlappingFull::C));
        assert_eq!(
            "A | D",
            write(TestOverlappingFull::C | TestOverlappingFull::D)
        );
    }

    fn write<F: Flags>(value: F) -> String {
        let mut s = String::new();

        to_writer_strict(&value, &mut s).unwrap();
        s
    }
}
