#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "broken"]
#[doc(hidden)]
pub const broken: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("broken"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\broken.rs",
        start_line: 2usize,
        start_col: 4usize,
        end_line: 2usize,
        end_col: 10usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(broken())),
};
fn broken() {
    std::env::set_var(
        "CARGO_MANIFEST_DIR_OVERRIDE",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
    );
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/broken/*.rs");
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&broken])
}
