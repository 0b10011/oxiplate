#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline(
    "
{{ 19 }}
{{ 10 + 9 }}
{{ 0 }}
{{ 000 }}
{{ 0b10011 }}
{{ 0b0 }}
{{ 0b0000 }}
{{ 0b10011 + 19 }}
{{ 19 + 0b10011 }}"
)]
struct Data;
impl ::std::fmt::Display for Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::new();
            let f = &mut string;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&19))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(10 + 9)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&0))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&000))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&0b10011))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&0b0))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&0b0000))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(0b10011 + 19)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(19 + 0b10011)))?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "field"]
#[doc(hidden)]
pub const field: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("field"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/number.rs",
        start_line: 19usize,
        start_col: 4usize,
        end_line: 19usize,
        end_col: 9usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(field())),
};
fn field() {
    let data = Data;
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"
19
19
0
0
19
0
0
38
38",
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
#[oxiplate_inline("{{ 1_234_567 }}")]
struct DecimalNumberSeparators;
impl ::std::fmt::Display for DecimalNumberSeparators {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::new();
            let f = &mut string;
            f.write_str(&::std::string::ToString::to_string(&1_234_567))?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "decimal_number_separators"]
#[doc(hidden)]
pub const decimal_number_separators: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("decimal_number_separators"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/number.rs",
        start_line: 42usize,
        start_col: 4usize,
        end_line: 42usize,
        end_col: 29usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(decimal_number_separators()),
    ),
};
fn decimal_number_separators() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", DecimalNumberSeparators))
        }),
        &"1234567",
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
#[oxiplate_inline("{{ 0b0001_0011 }}")]
struct BinaryNumberSeparators;
impl ::std::fmt::Display for BinaryNumberSeparators {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::new();
            let f = &mut string;
            f.write_str(&::std::string::ToString::to_string(&0b0001_0011))?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "binary_number_separators"]
#[doc(hidden)]
pub const binary_number_separators: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("binary_number_separators"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/number.rs",
        start_line: 51usize,
        start_col: 4usize,
        end_line: 51usize,
        end_col: 28usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(binary_number_separators()),
    ),
};
fn binary_number_separators() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", BinaryNumberSeparators))
        }),
        &"19",
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
    test::test_main_static(
        &[&binary_number_separators, &decimal_number_separators, &field],
    )
}
