#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline = "
{%- if !foo -%}
    Yay
{%- endif %}"]
struct Not {
    foo: bool,
}
impl ::std::fmt::Display for Not {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        if !self.foo {
            f.write_str("Yay")?;
        }
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_if"]
#[doc(hidden)]
pub const test_if: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_if"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/not.rs",
        start_line: 13usize,
        start_col: 4usize,
        end_line: 13usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test_if())),
};
fn test_if() {
    let not = Not { foo: false };
    match (
        &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", not)) }),
        &"Yay",
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
    let not = Not { foo: true };
    match (
        &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", not)) }),
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
    test::test_main_static(&[&test_if])
}
