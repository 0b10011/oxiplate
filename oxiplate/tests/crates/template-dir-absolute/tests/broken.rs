#[test]
fn broken() {
    unsafe {
        std::env::set_var(
            "CARGO_MANIFEST_DIR_OVERRIDE",
            std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        );
        std::env::set_var("OXIP_TEMPLATE_DIR", "/templates");
    }

    let tests = trybuild::TestCases::new();

    tests.compile_fail("tests/broken/*.rs");
}
