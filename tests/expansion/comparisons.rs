#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::Oxiplate;
#[oxiplate_inline = r#"{% if value == "foo" %}bar{% endif %}"#]
struct Comparison {
    value: &'static str,
}
impl std::fmt::Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value == "foo" {
            f.write_fmt(format_args!("{0}", "bar"))?;
        }
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_equals_string"]
pub const test_equals_string: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_equals_string"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\comparisons.rs",
        start_line: 10usize,
        start_col: 4usize,
        end_line: 10usize,
        end_col: 22usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(test_equals_string())),
};
fn test_equals_string() {
    let data = Comparison { value: "foo" };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        },
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
#[cfg(test)]
#[rustc_test_marker = "test_does_not_equal_string"]
pub const test_does_not_equal_string: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_does_not_equal_string"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\comparisons.rs",
        start_line: 17usize,
        start_col: 4usize,
        end_line: 17usize,
        end_col: 30usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(test_does_not_equal_string())),
};
fn test_does_not_equal_string() {
    let data = Comparison { value: "baz" };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        },
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
#[no_coverage]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test_does_not_equal_string, &test_equals_string])
}
