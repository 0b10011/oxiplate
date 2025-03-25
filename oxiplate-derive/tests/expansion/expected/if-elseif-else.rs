#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline = "
{%- if do_this -%}
    This then {{ action }} :D
{%- elseif do_that -%}
    That then {{ action }} :D
{%- else -%}
    Can't {{ action }} :(
{%- endif %}"]
struct Data {
    do_this: bool,
    do_that: bool,
    action: &'static str,
}
impl ::std::fmt::Display for Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        if self.do_this {
            f.write_fmt(format_args!("This then {0} :D", self.action))?;
        } else if self.do_that {
            f.write_fmt(format_args!("That then {0} :D", self.action))?;
        } else {
            f.write_fmt(format_args!("Can\'t {0} :(", self.action))?;
        }
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_if"]
#[doc(hidden)]
pub const test_if: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_if"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/if-elseif-else.rs",
        start_line: 19usize,
        start_col: 4usize,
        end_line: 19usize,
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
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
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
#[cfg(test)]
#[rustc_test_marker = "test_else_if"]
#[doc(hidden)]
pub const test_else_if: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_else_if"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/if-elseif-else.rs",
        start_line: 30usize,
        start_col: 4usize,
        end_line: 30usize,
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
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
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
#[cfg(test)]
#[rustc_test_marker = "test_else"]
#[doc(hidden)]
pub const test_else: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_else"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/if-elseif-else.rs",
        start_line: 41usize,
        start_col: 4usize,
        end_line: 41usize,
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
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
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
