#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline(
    "
{%- if !ref.is_empty() -%}
    Referee: {{ ref }}
{%- else -%}
    {{ else }}
{%- endif %}"
)]
struct Data {
    r#ref: &'static str,
    r#else: &'static str,
}
impl ::core::fmt::Display for Data {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        if !self.r#ref.is_empty() {
            oxiplate_formatter.write_str("Referee: ")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.r#ref)))?;
        } else {
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.r#else)))?;
        }
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_if"]
#[doc(hidden)]
pub static test_if: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_if"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/rust-keywords.rs",
        start_line: 24usize,
        start_col: 4usize,
        end_line: 24usize,
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
    {
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
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_else"]
#[doc(hidden)]
pub static test_else: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_else"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/rust-keywords.rs",
        start_line: 34usize,
        start_col: 4usize,
        end_line: 34usize,
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
    {
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
        }
    };
}
extern crate test;
#[rustc_test_marker = "include"]
#[doc(hidden)]
pub static include: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("include"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/rust-keywords.rs",
        start_line: 44usize,
        start_col: 4usize,
        end_line: 44usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(include())),
};
fn include() {
    #[oxiplate_inline(
        r#"
{%- extends "rust-keywords/extends.html.oxip" %}
{% block body -%}
{% parent %}
should be the same as:
Referee: {{ ref }}
{%- endblock %}"#
    )]
    struct Data {
        r#ref: &'static str,
    }
    impl ::core::fmt::Display for Data {
        fn fmt(
            &self,
            oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("<!DOCTYPE html>\n<title>")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.r#ref)))?;
            oxiplate_formatter.write_str("</title>\n<article>")?;
            {
                {}
                {
                    oxiplate_formatter.write_str("Referee: ")?;
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(self.r#ref)))?;
                }
                {}
                {
                    oxiplate_formatter.write_str("\nshould be the same as:\nReferee: ")?;
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(self.r#ref)))?;
                }
            }
            oxiplate_formatter.write_str("</article>\n")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", Data { r#ref: "Jess" }))
            }),
            &r#"<!DOCTYPE html>
<title>Jess</title>
<article>Referee: Jess
should be the same as:
Referee: Jess</article>
"#,
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
        }
    };
}
extern crate test;
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> test::ExitCode {
    test::test_main_env_args(&[&include, &test_else, &test_if])
}
