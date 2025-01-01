#[test]
fn broken() {
    unsafe {
        std::env::set_var(
            "CARGO_MANIFEST_DIR_OVERRIDE",
            std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        );
    }

    let tests = trybuild::TestCases::new();

    // To verify `trybuild` is building things as intended (i.e., `oxiplate.toml` is included)
    tests.pass("tests/broken-verify/correct-group.rs");

    // The actual tests are in the `broken` directory next to this file
    tests.compile_fail("tests/broken/*.rs");
}
