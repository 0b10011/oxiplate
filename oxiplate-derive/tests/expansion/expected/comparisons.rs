#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline(r#"{% if value == "foo" %}bar{% endif %}"#)]
struct Comparison {
    value: &'static str,
}
impl ::std::fmt::Display for Comparison {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::new();
            let f = &mut string;
            if (self.value == "foo") {
                f.write_str("bar")?;
            }
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "test_equals_string"]
#[doc(hidden)]
pub const test_equals_string: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_equals_string"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/comparisons.rs",
        start_line: 10usize,
        start_col: 4usize,
        end_line: 10usize,
        end_col: 22usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_equals_string()),
    ),
};
fn test_equals_string() {
    let data = Comparison { value: "foo" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"bar",
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_does_not_equal_string"]
#[doc(hidden)]
pub const test_does_not_equal_string: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_does_not_equal_string"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/comparisons.rs",
        start_line: 17usize,
        start_col: 4usize,
        end_line: 17usize,
        end_col: 30usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_does_not_equal_string()),
    ),
};
fn test_does_not_equal_string() {
    let data = Comparison { value: "baz" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"",
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test_does_not_equal_string, &test_equals_string])
}
