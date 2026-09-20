#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline(r###"{{ ##"jane #"the deer"# doe"## }}"###)]
struct RawString {}
impl ::core::fmt::Display for RawString {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter
            .write_str(
                &alloc::string::ToString::to_string(&("jane #\"the deer\"# doe")),
            )?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "raw_string"]
#[doc(hidden)]
pub static raw_string: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("raw_string"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/string.rs",
        start_line: 14usize,
        start_col: 4usize,
        end_line: 14usize,
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
    {
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
        }
    };
}
#[oxiplate_inline(r#"{{ "" }}"#)]
struct EmptyString {}
impl ::core::fmt::Display for EmptyString {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("")))?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "empty_string"]
#[doc(hidden)]
pub static empty_string: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("empty_string"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/string.rs",
        start_line: 25usize,
        start_col: 4usize,
        end_line: 25usize,
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
    {
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
        }
    };
}
#[oxiplate_inline("\x00 \x0F \x0f \x7F")]
struct SevenBitEscapes;
impl ::core::fmt::Display for SevenBitEscapes {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str("\u{0} \u{f} \u{f} \u{7f}")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "seven_bit_escapes"]
#[doc(hidden)]
pub static seven_bit_escapes: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("seven_bit_escapes"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/string.rs",
        start_line: 36usize,
        start_col: 4usize,
        end_line: 36usize,
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
    {
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
        }
    };
}
#[oxiplate_inline("\u{0} \u{10ffff}")]
struct UnicodeEscapes;
impl ::core::fmt::Display for UnicodeEscapes {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str("\u{0} \u{10ffff}")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "unicode_escapes"]
#[doc(hidden)]
pub static unicode_escapes: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("unicode_escapes"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/string.rs",
        start_line: 45usize,
        start_col: 4usize,
        end_line: 45usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(unicode_escapes()),
    ),
};
fn unicode_escapes() {
    {
        match (
            &"\0 \u{10ffff}",
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", UnicodeEscapes))
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
        }
    };
}
extern crate test;
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> test::ExitCode {
    test::test_main_env_args(
        &[&empty_string, &raw_string, &seven_bit_escapes, &unicode_escapes],
    )
}
