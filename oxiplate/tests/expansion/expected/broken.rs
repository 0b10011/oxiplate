#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
extern crate test;
#[rustc_test_marker = "broken"]
#[doc(hidden)]
pub const broken: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("broken"),
        ignore: true,
        ignore_message: ::core::option::Option::Some(
            "Broken tests are expensive and can fail on slight wording changes, so they should be run separately.",
        ),
        source_file: "oxiplate/tests/broken.rs",
        start_line: 4usize,
        start_col: 4usize,
        end_line: 4usize,
        end_col: 10usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(broken())),
};
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
    tests.pass("tests/broken-verify/with-group.rs");
    tests.compile_fail("tests/broken/**/*.rs");
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&broken])
}
