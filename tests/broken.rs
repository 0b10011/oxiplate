#[test]
fn broken() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/broken/*.rs");
}
