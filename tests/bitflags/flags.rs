use super::*;

// use bitflag_attr::Flags;

#[test]
fn cases() {
    let flags = TestFlags::NAMED_FLAGS
        .iter()
        .map(|(name, flag)| (*name, flag.bits()))
        .collect::<Vec<_>>();

    assert_eq!(
        vec![
            ("A", 1u8),
            ("B", 1 << 1),
            ("C", 1 << 2),
            ("ABC", 1 | (1 << 1) | (1 << 2)),
        ],
        flags,
    );

    assert_eq!(0, TestEmpty::NAMED_FLAGS.len());
}

mod external {
    use super::*;

    #[test]
    fn cases() {
        let flags = TestExternal::NAMED_FLAGS
            .iter()
            .map(|(name, flag)| (*name, flag.bits()))
            .collect::<Vec<_>>();

        assert_eq!(
            vec![
                ("A", 1u8),
                ("B", 1 << 1),
                ("C", 1 << 2),
                ("ABC", 1 | (1 << 1) | (1 << 2)),
                // ("", !0),
            ],
            flags,
        );
    }
}
