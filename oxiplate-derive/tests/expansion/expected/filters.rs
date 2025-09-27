#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use oxiplate_derive::Oxiplate;
mod filters_for_oxiplate {
    use std::fmt::Display;
    pub fn respond(expression: impl Display) -> impl Display {
        let expression = expression.to_string();
        match expression.as_str() {
            "hello" => "world".to_string(),
            _ => "did not understand: ".to_string() + &expression,
        }
    }
    pub fn shorten(expression: impl Display, max_length: usize) -> impl Display {
        let expression = expression.to_string();
        if expression.len() <= max_length {
            expression
        } else {
            expression[0..=max_length - 1].to_string()
        }
    }
    pub fn pad(expression: usize, max_length: usize) -> impl Display {
        ::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0:1$}", expression, max_length))
        })
    }
}
#[oxiplate_inline(r#"{{ message | respond() }}"#)]
struct Respond {
    message: &'static str,
}
impl ::std::fmt::Display for Respond {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(1usize);
            let f = &mut string;
            f.write_str(
                &::std::string::ToString::to_string(
                    &(crate::filters_for_oxiplate::respond(self.message)),
                ),
            )?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "respond"]
#[doc(hidden)]
pub const respond: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("respond"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/filters.rs",
        start_line: 35usize,
        start_col: 4usize,
        end_line: 35usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(respond())),
};
fn respond() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", Respond { message: "hello" }))
        }),
        &"world",
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
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", Respond { message: "goodbye" }))
        }),
        &"did not understand: goodbye",
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
#[oxiplate_inline(r#"{{ message | shorten(max_length) }}"#)]
struct Shorten {
    message: &'static str,
    max_length: usize,
}
impl ::std::fmt::Display for Shorten {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(1usize);
            let f = &mut string;
            f.write_str(
                &::std::string::ToString::to_string(
                    &(crate::filters_for_oxiplate::shorten(
                        self.message,
                        self.max_length,
                    )),
                ),
            )?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "shorten"]
#[doc(hidden)]
pub const shorten: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("shorten"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/filters.rs",
        start_line: 51usize,
        start_col: 4usize,
        end_line: 51usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(shorten())),
};
fn shorten() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    Shorten {
                        message: "hello",
                        max_length: 2,
                    },
                ),
            )
        }),
        &"he",
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
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    Shorten {
                        message: "goodbye",
                        max_length: 3,
                    },
                ),
            )
        }),
        &"goo",
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
#[oxiplate_inline(r#"{{ number | pad(length) }}"#)]
struct Pad {
    number: usize,
    length: usize,
}
impl ::std::fmt::Display for Pad {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(1usize);
            let f = &mut string;
            f.write_str(
                &::std::string::ToString::to_string(
                    &(crate::filters_for_oxiplate::pad(self.number, self.length)),
                ),
            )?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "pad"]
#[doc(hidden)]
pub const pad: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("pad"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/filters.rs",
        start_line: 82usize,
        start_col: 4usize,
        end_line: 82usize,
        end_col: 7usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(pad())),
};
fn pad() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", Pad { number: 19, length: 2 }))
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
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", Pad { number: 19, length: 3 }))
        }),
        &" 19",
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
#[oxiplate_inline(r#"{{ message | respond() | shorten(length) }}"#)]
struct Multiple {
    message: &'static str,
    length: usize,
}
impl ::std::fmt::Display for Multiple {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(1usize);
            let f = &mut string;
            f.write_str(
                &::std::string::ToString::to_string(
                    &(crate::filters_for_oxiplate::shorten(
                        crate::filters_for_oxiplate::respond(self.message),
                        self.length,
                    )),
                ),
            )?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "multiple"]
#[doc(hidden)]
pub const multiple: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("multiple"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/filters.rs",
        start_line: 113usize,
        start_col: 4usize,
        end_line: 113usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(multiple())),
};
fn multiple() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    Multiple {
                        message: "hello",
                        length: 6,
                    },
                ),
            )
        }),
        &"world",
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
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    Multiple {
                        message: "hello",
                        length: 5,
                    },
                ),
            )
        }),
        &"world",
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
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    Multiple {
                        message: "hello",
                        length: 4,
                    },
                ),
            )
        }),
        &"worl",
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
    test::test_main_static(&[&multiple, &pad, &respond, &shorten])
}
