#[test]
#[ignore = "Broken tests are expensive and can fail on slight wording changes, so they should be \
            run separately."]
fn broken() {
    unsafe {
        std::env::set_var(
            "CARGO_MANIFEST_DIR_OVERRIDE",
            std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        );
        std::env::set_var("OXIP_TEMPLATE_DIR", "missing-dir");
    }

    let tests = trybuild::TestCases::new();

    tests.compile_fail("tests/broken/broken.rs");
}
