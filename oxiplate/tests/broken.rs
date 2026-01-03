#[test]
#[ignore = "Broken tests are expensive and can fail on slight wording changes, so they should be \
            run separately."]
fn broken() {
    unsafe {
        std::env::set_var(
            "CARGO_MANIFEST_DIR_OVERRIDE",
            std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        );
    }

    let tests = trybuild::TestCases::new();

    // To verify `trybuild` is building things as intended (i.e., `oxiplate.toml` is included)
    tests.pass("tests/broken-verify/with-group.rs");

    // The actual tests are in the `broken` directory next to this file
    tests.compile_fail("tests/broken/**/*.rs");
}
