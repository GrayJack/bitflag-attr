#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/00-smoke");
    t.compile_fail("tests/01-invalid_type");
    t.compile_fail("tests/02-no_value");
    t.compile_fail("tests/03-too_many_args");
    t.compile_fail("tests/04-repetitive_args");
}
