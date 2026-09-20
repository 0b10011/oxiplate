#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use oxiplate_derive::Oxiplate;
struct User<'a> {
    name: &'a str,
}
#[oxiplate_inline("{{ user.name }}")]
struct Data<'a> {
    user: &'a User<'a>,
}
impl<'a> ::core::fmt::Display for Data<'a> {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter
            .write_str(&alloc::string::ToString::to_string(&(self.user.name)))?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "field"]
#[doc(hidden)]
pub static field: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("field"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/borrow.rs",
        start_line: 20usize,
        start_col: 4usize,
        end_line: 20usize,
        end_col: 9usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(field())),
};
fn field() {
    let name = "Liv";
    let user = User { name };
    let data = Data { user: &user };
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", data))
            }),
            &"Liv",
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
#[rustc_test_marker = "ref_and_deref"]
#[doc(hidden)]
pub static ref_and_deref: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("ref_and_deref"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/borrow.rs",
        start_line: 29usize,
        start_col: 4usize,
        end_line: 29usize,
        end_col: 17usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(ref_and_deref()),
    ),
};
fn ref_and_deref() {
    #[oxiplate_inline(
        r#"
{%- if **&value == "a" -%}
    string slice
{%- elseif value == &"b" -%}
    borrowed string slice
{%- elseif &*value == &&*"c" -%}
    double borrowed and dereferenced string slice
{%- else -%}
    no match for "{{ value }}"
{%- endif -%}
"#
    )]
    struct Data {
        value: &'static &'static str,
    }
    impl ::core::fmt::Display for Data {
        fn fmt(
            &self,
            oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            if **&self.value == "a" {
                oxiplate_formatter.write_str("string slice")?;
            } else if self.value == &"b" {
                oxiplate_formatter.write_str("borrowed string slice")?;
            } else if &*self.value == &&*"c" {
                oxiplate_formatter
                    .write_str("double borrowed and dereferenced string slice")?;
            } else {
                oxiplate_formatter.write_str("no match for \"")?;
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
                oxiplate_formatter.write_str("\"")?;
            }
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", Data { value: &"a" }))
            }),
            &"string slice",
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
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", Data { value: &"b" }))
            }),
            &"borrowed string slice",
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
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", Data { value: &"c" }))
            }),
            &"double borrowed and dereferenced string slice",
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
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", Data { value: &"d" }))
            }),
            &r#"no match for "d""#,
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
#[rustc_test_marker = "test_ref_deref_assign"]
#[doc(hidden)]
pub static test_ref_deref_assign: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_ref_deref_assign"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/borrow.rs",
        start_line: 61usize,
        start_col: 4usize,
        end_line: 61usize,
        end_col: 25usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_ref_deref_assign()),
    ),
};
fn test_ref_deref_assign() {
    #[oxiplate_inline(
        r#"
{%- let new_value = &**value -%}
{%- if new_value == "a" -%}
    A
{%- else -%}
    {{ new_value }}
{%- endif -%}
"#
    )]
    struct Data {
        value: &'static &'static str,
    }
    impl ::core::fmt::Display for Data {
        fn fmt(
            &self,
            oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            let new_value = &**self.value;
            if new_value == "a" {
                oxiplate_formatter.write_str("A")?;
            } else {
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(new_value)))?;
            }
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", Data { value: &"a" }))
            }),
            &"A",
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
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", Data { value: &"b" }))
            }),
            &"b",
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
    test::test_main_env_args(&[&field, &ref_and_deref, &test_ref_deref_assign])
}
