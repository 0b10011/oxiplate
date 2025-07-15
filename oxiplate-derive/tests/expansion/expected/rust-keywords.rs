#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline = "
{%- if !ref.is_empty() -%}
    Referee: {{ ref }}
{%- else -%}
    {{ else }}
{%- endif %}"]
struct Data {
    r#ref: &'static str,
    r#else: &'static str,
}
impl ::std::fmt::Display for Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        if !self.r#ref.is_empty() {
            f.write_fmt(format_args!("Referee: {0}", self.r#ref))?;
        } else {
            f.write_fmt(format_args!("{0}", self.r#else))?;
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
        source_file: "oxiplate-derive/tests/rust-keywords.rs",
        start_line: 16usize,
        start_col: 4usize,
        end_line: 16usize,
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
        r#ref: "Jax",
        r#else: "No referee available",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"Referee: Jax",
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
        source_file: "oxiplate-derive/tests/rust-keywords.rs",
        start_line: 26usize,
        start_col: 4usize,
        end_line: 26usize,
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
        r#ref: "",
        r#else: "No referee available",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"No referee available",
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
#[rustc_test_marker = "syn_tokens"]
#[doc(hidden)]
pub const syn_tokens: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("syn_tokens"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/rust-keywords.rs",
        start_line: 36usize,
        start_col: 4usize,
        end_line: 36usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(syn_tokens()),
    ),
};
fn syn_tokens() {}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&syn_tokens, &test_else, &test_if])
}
