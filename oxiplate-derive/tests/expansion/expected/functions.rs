#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use oxiplate_derive::Oxiplate;
extern crate test;
#[rustc_test_marker = "self_function"]
#[doc(hidden)]
pub static self_function: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("self_function"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/functions.rs",
        start_line: 10usize,
        start_col: 4usize,
        end_line: 10usize,
        end_col: 17usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(self_function()),
    ),
};
fn self_function() {
    #[oxiplate_inline("{{ Self::double(89) }} {{ self.double_value() }}")]
    struct Data {
        value: usize,
    }
    impl ::core::fmt::Display for Data {
        fn fmt(
            &self,
            oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(Self::double(89))))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.double_value())))?;
            ::core::result::Result::Ok(())
        }
    }
    impl Data {
        fn double(value: usize) -> usize {
            value * 2
        }
        fn double_value(&self) -> usize {
            self.value * 2
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", Data { value: 19 }))
            }),
            &"178 38",
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
#[rustc_test_marker = "expression_function"]
#[doc(hidden)]
pub static expression_function: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expression_function"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/functions.rs",
        start_line: 31usize,
        start_col: 4usize,
        end_line: 31usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(expression_function()),
    ),
};
fn expression_function() {
    #[oxiplate_inline(
        r#"
{%- for i in 0..functions.len() -%}
    {{ functions[i](value) }}
{% endfor -%}
"#
    )]
    struct Data<'a> {
        functions: &'a [fn(usize) -> usize],
        value: usize,
    }
    impl<'a> ::core::fmt::Display for Data<'a> {
        fn fmt(
            &self,
            oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            for i in 0..self.functions.len() {
                oxiplate_formatter
                    .write_str(
                        &alloc::string::ToString::to_string(
                            &(self.functions[i](self.value)),
                        ),
                    )?;
                oxiplate_formatter.write_str("\n")?;
            }
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!(
                        "{0}",
                        Data {
                            functions: &[|i| i - 1, |i| i, |i| i + 1, |i| i * 2],
                            value: 19,
                        },
                    ),
                )
            }),
            &"18\n19\n20\n38\n",
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
fn triple(value: usize) -> usize {
    value * 3
}
extern crate test;
#[rustc_test_marker = "crate_function"]
#[doc(hidden)]
pub static crate_function: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("crate_function"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/functions.rs",
        start_line: 63usize,
        start_col: 4usize,
        end_line: 63usize,
        end_col: 18usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(crate_function()),
    ),
};
fn crate_function() {
    #[oxiplate_inline("{{ crate::triple(value) }}")]
    struct Data {
        value: usize,
    }
    impl ::core::fmt::Display for Data {
        fn fmt(
            &self,
            oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter
                .write_str(
                    &alloc::string::ToString::to_string(&(crate::triple(self.value))),
                )?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", Data { value: 19 }))
            }),
            &"57",
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
#[rustc_test_marker = "global_path"]
#[doc(hidden)]
pub static global_path: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("global_path"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/functions.rs",
        start_line: 74usize,
        start_col: 4usize,
        end_line: 74usize,
        end_col: 15usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(global_path()),
    ),
};
fn global_path() {
    #[oxiplate_inline(r#"{{- ::core::borrow::Borrow::borrow(value) -}}"#)]
    struct Data {
        value: &'static str,
    }
    impl ::core::fmt::Display for Data {
        fn fmt(
            &self,
            oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter
                .write_str(
                    &alloc::string::ToString::to_string(
                        &(::core::borrow::Borrow::borrow(self.value)),
                    ),
                )?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", Data { value: "hello world" }))
            }),
            &"hello world",
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
        &[&crate_function, &expression_function, &global_path, &self_function],
    )
}
