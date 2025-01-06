#[test]
fn broken() {
    std::env::set_var(
        "CARGO_MANIFEST_DIR_OVERRIDE",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
    );

    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/broken/*.rs");
}
