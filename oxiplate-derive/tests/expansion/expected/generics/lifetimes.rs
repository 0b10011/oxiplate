use alloc::format;
use alloc::string::String;

use oxiplate_derive::Oxiplate;
extern crate test;
#[rustc_test_marker = "generics::lifetimes::none"]
#[doc(hidden)]
pub static none: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("generics::lifetimes::none"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/generics/lifetimes.rs",
        start_line: 7usize,
        start_col: 4usize,
        end_line: 7usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(none()),
    ),
};
fn none<'c>() {
    fn function(left: &str) -> String {
        ::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0:?}", left)) })
    }
    #[oxiplate_inline(r#"{{ function::<>(left) }}"#)]
    struct Data {
        left: &'static str,
    }
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(function(self.left))))?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", Data { left: "hello" }))
            }),
            &r#""hello""#,
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
#[rustc_test_marker = "generics::lifetimes::one"]
#[doc(hidden)]
pub static one: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("generics::lifetimes::one"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/generics/lifetimes.rs",
        start_line: 22usize,
        start_col: 4usize,
        end_line: 22usize,
        end_col: 7usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(one()),
    ),
};
fn one<'c>() {
    fn function<'a: 'a>(left: &'a str) -> String {
        ::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0:?}", left)) })
    }
    #[oxiplate_inline(r#"{{ function::<'c>(left) }}"#)]
    struct Data<'c> {
        left: &'c str,
    }
    impl<'c> ::core::fmt::Display for Data<'c> {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(function::<'c>(self.left)),
            ))?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", Data { left: "hello" }))
            }),
            &r#""hello""#,
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
#[rustc_test_marker = "generics::lifetimes::one_trailing_comma"]
#[doc(hidden)]
pub static one_trailing_comma: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("generics::lifetimes::one_trailing_comma"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/generics/lifetimes.rs",
        start_line: 37usize,
        start_col: 4usize,
        end_line: 37usize,
        end_col: 22usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(one_trailing_comma()),
    ),
};
fn one_trailing_comma<'c>() {
    fn function<'a: 'a>(left: &'a str) -> String {
        ::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0:?}", left)) })
    }
    #[oxiplate_inline(r#"{{ function::<'c,>(left) }}"#)]
    struct Data<'c> {
        left: &'c str,
    }
    impl<'c> ::core::fmt::Display for Data<'c> {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(function::<'c>(self.left)),
            ))?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", Data { left: "hello" }))
            }),
            &r#""hello""#,
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
#[rustc_test_marker = "generics::lifetimes::two"]
#[doc(hidden)]
pub static two: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("generics::lifetimes::two"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/generics/lifetimes.rs",
        start_line: 52usize,
        start_col: 4usize,
        end_line: 52usize,
        end_col: 7usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(two()),
    ),
};
fn two<'c, 'd>() {
    fn function<'a: 'a, 'b: 'b>(left: &'a str, right: &'b str) -> String {
        ::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0:?} {1:?}", left, right))
        })
    }
    #[oxiplate_inline(r#"{{ function::<'c, 'd>(left, right) }}"#)]
    struct Data<'c, 'd> {
        left: &'c str,
        right: &'d str,
    }
    impl<'c, 'd> ::core::fmt::Display for Data<'c, 'd> {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(function::<'c, 'd>(self.left, self.right)),
            ))?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!(
                    "{0}",
                    Data {
                        left: "hello",
                        right: "world"
                    }
                ))
            }),
            &r#""hello" "world""#,
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
#[rustc_test_marker = "generics::lifetimes::two_trailing_comma"]
#[doc(hidden)]
pub static two_trailing_comma: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("generics::lifetimes::two_trailing_comma"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/generics/lifetimes.rs",
        start_line: 77usize,
        start_col: 4usize,
        end_line: 77usize,
        end_col: 22usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(two_trailing_comma()),
    ),
};
fn two_trailing_comma<'c, 'd>() {
    fn function<'a: 'a, 'b: 'b>(left: &'a str, right: &'b str) -> String {
        ::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0:?} {1:?}", left, right))
        })
    }
    #[oxiplate_inline(r#"{{ function::<'c, 'd,>(left, right) }}"#)]
    struct Data<'c, 'd> {
        left: &'c str,
        right: &'d str,
    }
    impl<'c, 'd> ::core::fmt::Display for Data<'c, 'd> {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(function::<'c, 'd>(self.left, self.right)),
            ))?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!(
                    "{0}",
                    Data {
                        left: "hello",
                        right: "world"
                    }
                ))
            }),
            &r#""hello" "world""#,
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
