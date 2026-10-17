use alloc::format;

use oxiplate_derive::Oxiplate;
#[oxiplate_inline(
    "
{%- if do_this -%}
    This then {{ action }} :D
{%- else -%}
    Can't {{ action }} :(
{%- endif %}"
)]
struct Data {
    do_this: bool,
    action: &'static str,
}
impl ::core::fmt::Display for Data {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        if self.do_this {
            oxiplate_formatter.write_str("This then ")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.action)))?;
            oxiplate_formatter.write_str(" :D")?;
        } else {
            oxiplate_formatter.write_str("Can\'t ")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.action)))?;
            oxiplate_formatter.write_str(" :(")?;
        }
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "r#if::if_else::test_if"]
#[doc(hidden)]
pub static test_if: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("r#if::if_else::test_if"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/if/if_else.rs",
        start_line: 20usize,
        start_col: 4usize,
        end_line: 20usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_if()),
    ),
};
fn test_if() {
    let data = Data {
        do_this: true,
        action: "do something",
    };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
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
        }
    };
}
extern crate test;
#[rustc_test_marker = "r#if::if_else::test_else"]
#[doc(hidden)]
pub static test_else: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("r#if::if_else::test_else"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/if/if_else.rs",
        start_line: 30usize,
        start_col: 4usize,
        end_line: 30usize,
        end_col: 13usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_else()),
    ),
};
fn test_else() {
    let data = Data {
        do_this: false,
        action: "do something",
    };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
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
        }
    };
}
