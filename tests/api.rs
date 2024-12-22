use bitflag_attr::bitflag;

#[bitflag(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum TestFlags {
    F1 = 1 << 0,
    F2 = 1 << 1,
    F3 = 1 << 3,
    F4 = 1 << 4,
    F1_3 = F1 | F3,
}

#[test]
fn constructors_works() {
    let empty = TestFlags::empty();
    assert_eq!(empty.bits(), 0);

    let all_known = TestFlags::all();
    assert_eq!(all_known.bits(), (1 << 0) | (1 << 1) | (1 << 3) | (1 << 4));

    let all_bits_set = TestFlags::all_bits();
    assert_eq!(all_bits_set.bits(), !0);

    let from_bits = TestFlags::from_bits((1 << 0) | (1 << 1));
    assert!(from_bits.is_some());
    assert_eq!(from_bits, Some(TestFlags::F1 | TestFlags::F2));

    let from_bits = TestFlags::from_bits(0x1276);
    assert!(from_bits.is_none());

    let from_truncate = TestFlags::from_bits_truncate(1 | 2 | 0x20);
    assert_eq!(from_truncate, TestFlags::F1 | TestFlags::F2);

    let from_name = TestFlags::from_flag_name("F1");
    assert_eq!(from_name, Some(TestFlags::F1));
    let from_name = TestFlags::from_flag_name("F2");
    assert_eq!(from_name, Some(TestFlags::F2));
    let from_name = TestFlags::from_flag_name("F3");
    assert_eq!(from_name, Some(TestFlags::F3));
    let from_name = TestFlags::from_flag_name("F4");
    assert_eq!(from_name, Some(TestFlags::F4));
    let from_name = TestFlags::from_flag_name("F1_3");
    assert_eq!(from_name, Some(TestFlags::F1_3));
    let from_name = TestFlags::from_flag_name("NOOOO");
    assert!(from_name.is_none());
}

#[test]
fn truncate_works() {
    // Flag with known flags won't change
    let test = TestFlags::all().truncate();
    assert_eq!(
        test,
        TestFlags::F1 | TestFlags::F2 | TestFlags::F3 | TestFlags::F4
    );
    let test = TestFlags::F1 | TestFlags::F2 | TestFlags::F3;
    assert_eq!(test.truncate(), test);

    let test = TestFlags::all_bits().truncate();
    assert_eq!(test, TestFlags::all());
}

#[test]
fn intersects_works() {
    let g1 = TestFlags::F1 | TestFlags::F3;
    let g2 = TestFlags::F3 | TestFlags::F4;
    let g3 = TestFlags::F2 | TestFlags::F4;
    assert!(g1.intersects(g2));
    assert!(!g1.intersects(g3));
    assert!(g2.intersects(g3));
}

#[test]
fn contains_works() {
    let test = TestFlags::F1 | TestFlags::F3;
    assert!(test.contains(TestFlags::F1));
    assert!(!test.contains(TestFlags::F2));
    assert!(test.contains(TestFlags::F3));
    assert!(test.contains(TestFlags::F1_3));
    assert!(test.contains(TestFlags::F1 | TestFlags::F3));
}

#[test]
fn set_works() {
    let mut test = TestFlags::empty();
    assert!(test.is_empty());
    test.set(TestFlags::F1);
    assert!(!test.is_empty());
    assert!(test.contains(TestFlags::F1));
    assert_eq!(test, TestFlags::F1);
    test.set(TestFlags::F2);
    assert!(!test.is_empty());
    assert!(test.contains(TestFlags::F2));
    assert_eq!(test, TestFlags::F1 | TestFlags::F2);
    test.set(TestFlags::F3);
    assert!(!test.is_empty());
    assert!(test.contains(TestFlags::F3));
    assert_eq!(test, TestFlags::F1 | TestFlags::F2 | TestFlags::F3);
    test.set(TestFlags::F4);
    assert!(!test.is_empty());
    assert!(test.contains(TestFlags::F4));
    assert_eq!(test, TestFlags::all());

    let unknown = TestFlags::from_bits_retain(1 << 12);
    test.set(unknown);
    assert!(!test.is_empty());
    assert!(test.contains(unknown));
    assert_eq!(test, TestFlags::all() | unknown);
}

#[test]
fn unset_works() {
    let unknown = TestFlags::from_bits_retain(1 << 12);
    let mut test = TestFlags::all() | unknown;
    assert!(!test.is_empty());
    assert!(test.contains(TestFlags::F1 | TestFlags::F2 | TestFlags::F3 | TestFlags::F4));
    assert!(test.contains(unknown));

    test.unset(TestFlags::F1);
    assert!(!test.is_empty());
    assert!(!test.contains(TestFlags::F1));
    assert!(test.contains(TestFlags::F2 | TestFlags::F3 | TestFlags::F4 | unknown));

    test.unset(TestFlags::F2);
    assert!(!test.is_empty());
    assert!(!test.contains(TestFlags::F2));
    assert!(test.contains(TestFlags::F3 | TestFlags::F4 | unknown));

    test.unset(TestFlags::F3);
    assert!(!test.is_empty());
    assert!(!test.contains(TestFlags::F3));
    assert!(test.contains(TestFlags::F4 | unknown));

    test.unset(TestFlags::F4);
    assert!(!test.is_empty());
    assert!(!test.contains(TestFlags::F4));
    assert!(test.contains(unknown));

    test.unset(unknown);
    assert!(test.is_empty());
    assert!(!test.contains(unknown));
}

#[test]
fn toggle_works() {
    let mut test = TestFlags::empty();
    assert!(test.is_empty());

    test.toggle(TestFlags::F1);
    assert!(!test.is_empty());
    assert!(test.contains(TestFlags::F1));
    test.toggle(TestFlags::F1);
    assert!(test.is_empty());
}

#[test]
fn complement_works() {
    // Complement should truncate, so assert_eq! should be used instead of contains.
    let test = TestFlags::F1 | TestFlags::F3;
    assert_eq!(test.complement(), TestFlags::F2 | TestFlags::F4);

    // Not trait defined as complement with truncation
    let unknown = TestFlags::from_bits_retain(1 << 12);
    let test = TestFlags::F3 | unknown;
    assert_eq!(!test, TestFlags::F1 | TestFlags::F4 | TestFlags::F2);
}

#[test]
fn intersection_works() {
    let g1 = TestFlags::F1;
    let g2 = TestFlags::F1 | TestFlags::F2;
    let g3 = TestFlags::F3 | TestFlags::F2;
    assert_eq!(g1.intersection(g2), TestFlags::F1);
    assert!(g1.intersection(g3).is_empty());
    assert_eq!(g2.intersection(g3), TestFlags::F2);
}

#[test]
fn union_works() {
    let g1 = TestFlags::F1;
    let g2 = TestFlags::F1 | TestFlags::F2;
    let g3 = TestFlags::F3 | TestFlags::F2;

    assert_eq!(g1.union(g2), g2);
    assert_eq!(g1.union(g3), TestFlags::F3 | TestFlags::F2 | TestFlags::F1);
    assert_eq!(g2.union(g3), TestFlags::F3 | TestFlags::F2 | TestFlags::F1);
}

#[test]
fn difference_works() {
    let g1 = TestFlags::F1;
    let g2 = TestFlags::F1 | TestFlags::F2;
    let g3 = TestFlags::F3 | TestFlags::F2;

    assert_eq!(g1.difference(g2), TestFlags::empty());
    assert_eq!(g2.difference(g1), TestFlags::F2);
    assert_eq!(g1.difference(g3), TestFlags::F1);
    assert_eq!(g3.difference(g1), g3);
    assert_eq!(g2.difference(g3), TestFlags::F1);
    assert_eq!(g3.difference(g2), TestFlags::F3);
}

#[test]
fn symmetric_difference_works() {
    let g1 = TestFlags::F1;
    let g2 = TestFlags::F1 | TestFlags::F2;
    let g3 = TestFlags::F3 | TestFlags::F2;

    assert_eq!(g1.symmetric_difference(g2), TestFlags::F2);
    assert_eq!(g2.symmetric_difference(g1), TestFlags::F2);
    assert_eq!(
        g1.symmetric_difference(g3),
        TestFlags::F1 | TestFlags::F2 | TestFlags::F3
    );
    assert_eq!(
        g3.symmetric_difference(g1),
        TestFlags::F1 | TestFlags::F2 | TestFlags::F3
    );
    assert_eq!(g2.symmetric_difference(g3), TestFlags::F1 | TestFlags::F3);
    assert_eq!(g3.symmetric_difference(g2), TestFlags::F1 | TestFlags::F3);
}
