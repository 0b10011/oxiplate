use alloc::format;

use oxiplate_derive::Oxiplate;
#[oxiplate_inline(r###"{{ ##"jane #"the deer"# doe"## }}"###)]
struct RawString {}
impl ::core::fmt::Display for RawString {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
            &("jane #\"the deer\"# doe"),
        ))?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "expressions::literals::string::raw_string"]
#[doc(hidden)]
pub static raw_string: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::literals::string::raw_string"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/literals/string.rs",
        start_line: 10usize,
        start_col: 4usize,
        end_line: 10usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
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
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", template)) }),
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
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("")))?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "expressions::literals::string::empty_string"]
#[doc(hidden)]
pub static empty_string: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::literals::string::empty_string"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/literals/string.rs",
        start_line: 21usize,
        start_col: 4usize,
        end_line: 21usize,
        end_col: 16usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
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
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", template)) }),
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
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str("\u{0} \u{f} \u{f} \u{7f}")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "expressions::literals::string::seven_bit_escapes"]
#[doc(hidden)]
pub static seven_bit_escapes: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::literals::string::seven_bit_escapes"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/literals/string.rs",
        start_line: 32usize,
        start_col: 4usize,
        end_line: 32usize,
        end_col: 21usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
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
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str("\u{0} \u{10ffff}")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "expressions::literals::string::unicode_escapes"]
#[doc(hidden)]
pub static unicode_escapes: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::literals::string::unicode_escapes"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/literals/string.rs",
        start_line: 41usize,
        start_col: 4usize,
        end_line: 41usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
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
