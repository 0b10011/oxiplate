#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline(
    "
{%- if do_this -%}
    This then {{ action }} :D
{%- elseif do_that -%}
    That then {{ action }} :D
{%- else -%}
    Can't {{ action }} :(
{%- endif %}"
)]
struct Data {
    do_this: bool,
    do_that: bool,
    action: &'static str,
}
impl ::std::fmt::Display for Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(10usize);
            let f = &mut string;
            if self.do_this {
                f.write_str("This then ")?;
                f.write_str(&::std::string::ToString::to_string(&(self.action)))?;
                f.write_str(" :D")?;
            } else if self.do_that {
                f.write_str("That then ")?;
                f.write_str(&::std::string::ToString::to_string(&(self.action)))?;
                f.write_str(" :D")?;
            } else {
                f.write_str("Can\'t ")?;
                f.write_str(&::std::string::ToString::to_string(&(self.action)))?;
                f.write_str(" :(")?;
            }
            string
        };
        f.write_str(&string)
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
        source_file: "oxiplate-derive/tests/if-elseif-else.rs",
        start_line: 21usize,
        start_col: 4usize,
        end_line: 21usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test_if())),
};
fn test_if() {
    let data = Data {
        do_this: true,
        do_that: true,
        action: "do something",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"This then do something :D",
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
#[rustc_test_marker = "test_else_if"]
#[doc(hidden)]
pub const test_else_if: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_else_if"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/if-elseif-else.rs",
        start_line: 32usize,
        start_col: 4usize,
        end_line: 32usize,
        end_col: 16usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_else_if()),
    ),
};
fn test_else_if() {
    let data = Data {
        do_this: false,
        do_that: true,
        action: "do something",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"That then do something :D",
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
#[rustc_test_marker = "test_else"]
#[doc(hidden)]
pub const test_else: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_else"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/if-elseif-else.rs",
        start_line: 43usize,
        start_col: 4usize,
        end_line: 43usize,
        end_col: 13usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test_else())),
};
fn test_else() {
    let data = Data {
        do_this: false,
        do_that: false,
        action: "do something",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"Can't do something :(",
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
    test::test_main_static(&[&test_else, &test_else_if, &test_if])
}
