#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline(r###"{{ ##"jane #"the deer"# doe"## }}"###)]
struct RawString {}
impl ::std::fmt::Display for RawString {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(21usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter
                .write_str(
                    &::std::string::ToString::to_string(&("jane #\"the deer\"# doe")),
                )?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "raw_string"]
#[doc(hidden)]
pub const raw_string: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("raw_string"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/string.rs",
        start_line: 8usize,
        start_col: 4usize,
        end_line: 8usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(raw_string()),
    ),
};
fn raw_string() {
    let template = RawString {};
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", template))
        }),
        &r###"jane #"the deer"# doe"###,
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
#[oxiplate_inline(r#"{{ "" }}"#)]
struct EmptyString {}
impl ::std::fmt::Display for EmptyString {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(0usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&("")))?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "empty_string"]
#[doc(hidden)]
pub const empty_string: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("empty_string"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/string.rs",
        start_line: 19usize,
        start_col: 4usize,
        end_line: 19usize,
        end_col: 16usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(empty_string()),
    ),
};
fn empty_string() {
    let template = EmptyString {};
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", template))
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
#[oxiplate_inline("\x00 \x0F \x0f \x7F")]
struct SevenBitEscapes;
impl ::std::fmt::Display for SevenBitEscapes {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(7usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter.write_str("\u{0} \u{f} \u{f} \u{7f}")?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "seven_bit_escapes"]
#[doc(hidden)]
pub const seven_bit_escapes: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("seven_bit_escapes"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/string.rs",
        start_line: 30usize,
        start_col: 4usize,
        end_line: 30usize,
        end_col: 21usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(seven_bit_escapes()),
    ),
};
fn seven_bit_escapes() {
    match (
        &"\0 \u{f} \u{f} \u{7f}",
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", SevenBitEscapes))
        }),
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
    test::test_main_static(&[&empty_string, &raw_string, &seven_bit_escapes])
}
