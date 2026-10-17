use alloc::format;

use oxiplate_derive::Oxiplate;
#[oxiplate_inline(r"{% if value == 5 %}bar{% endif %}")]
struct Comparison {
    value: u8,
}
impl ::core::fmt::Display for Comparison {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        if self.value == 5 {
            oxiplate_formatter.write_str("bar")?;
        }
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "expressions::comparisons_numbers::test_5u8"]
#[doc(hidden)]
pub static test_5u8: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::comparisons_numbers::test_5u8"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/comparisons_numbers.rs",
        start_line: 12usize,
        start_col: 4usize,
        end_line: 12usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_5u8()),
    ),
};
fn test_5u8() {
    let data = Comparison { value: 5u8 };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
            &"bar",
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
#[rustc_test_marker = "expressions::comparisons_numbers::test_5"]
#[doc(hidden)]
pub static test_5: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::comparisons_numbers::test_5"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/comparisons_numbers.rs",
        start_line: 19usize,
        start_col: 4usize,
        end_line: 19usize,
        end_col: 10usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_5()),
    ),
};
fn test_5() {
    let data = Comparison { value: 5 };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
            &"bar",
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
#[rustc_test_marker = "expressions::comparisons_numbers::test_4u8"]
#[doc(hidden)]
pub static test_4u8: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::comparisons_numbers::test_4u8"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/comparisons_numbers.rs",
        start_line: 26usize,
        start_col: 4usize,
        end_line: 26usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_4u8()),
    ),
};
fn test_4u8() {
    let data = Comparison { value: 4u8 };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
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
extern crate test;
#[rustc_test_marker = "expressions::comparisons_numbers::test_4"]
#[doc(hidden)]
pub static test_4: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::comparisons_numbers::test_4"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/comparisons_numbers.rs",
        start_line: 33usize,
        start_col: 4usize,
        end_line: 33usize,
        end_col: 10usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_4()),
    ),
};
fn test_4() {
    let data = Comparison { value: 4 };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
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
