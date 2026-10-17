use alloc::format;

use oxiplate_derive::Oxiplate;
#[oxiplate_inline(r"{{ 'a' }}")]
struct A;
impl ::core::fmt::Display for A {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&('a')))?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "expressions::literals::char::a"]
#[doc(hidden)]
pub static a: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::literals::char::a"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/literals/char.rs",
        start_line: 10usize,
        start_col: 4usize,
        end_line: 10usize,
        end_col: 5usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(a()),
    ),
};
fn a() {
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", A)) }),
            &"a",
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
#[oxiplate_inline(r"{{ '\'' }}")]
struct SingleQuote;
impl ::core::fmt::Display for SingleQuote {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&('\'')))?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "expressions::literals::char::single_quote"]
#[doc(hidden)]
pub static single_quote: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::literals::char::single_quote"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/literals/char.rs",
        start_line: 19usize,
        start_col: 4usize,
        end_line: 19usize,
        end_col: 16usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(single_quote()),
    ),
};
fn single_quote() {
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", SingleQuote))
            }),
            &"'",
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
#[oxiplate_inline(r#"{{ '\"' }}"#)]
struct DoubleQuote;
impl ::core::fmt::Display for DoubleQuote {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&('"')))?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "expressions::literals::char::double_quote"]
#[doc(hidden)]
pub static double_quote: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::literals::char::double_quote"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/literals/char.rs",
        start_line: 28usize,
        start_col: 4usize,
        end_line: 28usize,
        end_col: 16usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(double_quote()),
    ),
};
fn double_quote() {
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", DoubleQuote))
            }),
            &r#"""#,
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
#[oxiplate_inline(r"{{ '\\' }}")]
struct Slash;
impl ::core::fmt::Display for Slash {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&('\\')))?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "expressions::literals::char::slash"]
#[doc(hidden)]
pub static slash: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::literals::char::slash"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/literals/char.rs",
        start_line: 37usize,
        start_col: 4usize,
        end_line: 37usize,
        end_col: 9usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(slash()),
    ),
};
fn slash() {
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Slash)) }),
            &r"\",
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
#[oxiplate_inline("{{ '\n' }} {{ '\\n' }}")]
struct NewLine;
impl ::core::fmt::Display for NewLine {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&('\n')))?;
        oxiplate_formatter.write_str(" ")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&('\n')))?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "expressions::literals::char::new_line"]
#[doc(hidden)]
pub static new_line: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::literals::char::new_line"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/literals/char.rs",
        start_line: 46usize,
        start_col: 4usize,
        end_line: 46usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(new_line()),
    ),
};
fn new_line() {
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", NewLine)) }),
            &"\n \n",
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
#[oxiplate_inline("{{ '\r' }} {{ '\\r' }}")]
struct CarriageReturn;
impl ::core::fmt::Display for CarriageReturn {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&('\r')))?;
        oxiplate_formatter.write_str(" ")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&('\r')))?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "expressions::literals::char::carriage_return"]
#[doc(hidden)]
pub static carriage_return: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::literals::char::carriage_return"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/literals/char.rs",
        start_line: 55usize,
        start_col: 4usize,
        end_line: 55usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(carriage_return()),
    ),
};
fn carriage_return() {
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", CarriageReturn))
            }),
            &"\r \r",
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
#[oxiplate_inline("{{ '\t' }} {{ '\\t' }}")]
struct Tab;
impl ::core::fmt::Display for Tab {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&('\t')))?;
        oxiplate_formatter.write_str(" ")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&('\t')))?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "expressions::literals::char::tab"]
#[doc(hidden)]
pub static tab: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::literals::char::tab"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/literals/char.rs",
        start_line: 64usize,
        start_col: 4usize,
        end_line: 64usize,
        end_col: 7usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(tab()),
    ),
};
fn tab() {
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Tab)) }),
            &"\t \t",
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
#[oxiplate_inline(r"{{ '\0' }}")]
struct Null;
impl ::core::fmt::Display for Null {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&('\0')))?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "expressions::literals::char::null"]
#[doc(hidden)]
pub static null: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::literals::char::null"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/literals/char.rs",
        start_line: 73usize,
        start_col: 4usize,
        end_line: 73usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(null()),
    ),
};
fn null() {
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Null)) }),
            &"\0",
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
