#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use oxiplate_derive::Oxiplate;
mod filters_for_oxiplate {
    extern crate alloc;
    use alloc::borrow::ToOwned as _;
    use alloc::format;
    use alloc::string::ToString as _;
    use core::fmt::Display;
    pub fn respond(expression: impl Display, yell: bool) -> impl Display {
        let expression = expression.to_string();
        match expression.as_str() {
            "hello" => if yell { "WORLD" } else { "world" }.to_string(),
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
    pub fn trim(expression: impl Display) -> impl Display {
        expression.to_string().trim().to_owned()
    }
    pub fn replace(
        expression: impl Display,
        from: impl Display,
        to: impl Display,
    ) -> impl Display {
        expression.to_string().replace(&from.to_string(), &to.to_string()).to_owned()
    }
}
#[oxiplate_inline(
    r#"{{ message | respond(*respond) }} {{ message | respond(!*respond) }}"#
)]
struct Respond {
    message: &'static str,
    respond: &'static bool,
}
impl ::core::fmt::Display for Respond {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write as _;
            let mut string = alloc::string::String::with_capacity(3usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter
                .write_str(
                    &alloc::string::ToString::to_string(
                        &(crate::filters_for_oxiplate::respond(
                            self.message,
                            *self.respond,
                        )),
                    ),
                )?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(
                    &alloc::string::ToString::to_string(
                        &(crate::filters_for_oxiplate::respond(
                            self.message,
                            !*self.respond,
                        )),
                    ),
                )?;
            string
        };
        oxiplate_formatter.write_str(&string)
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
        start_line: 58usize,
        start_col: 4usize,
        end_line: 58usize,
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
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    Respond {
                        message: "hello",
                        respond: &false,
                    },
                ),
            )
        }),
        &"world WORLD",
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
                    Respond {
                        message: "goodbye",
                        respond: &false,
                    },
                ),
            )
        }),
        &"did not understand: goodbye did not understand: goodbye",
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
impl ::core::fmt::Display for Shorten {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write as _;
            let mut string = alloc::string::String::with_capacity(1usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter
                .write_str(
                    &alloc::string::ToString::to_string(
                        &(crate::filters_for_oxiplate::shorten(
                            self.message,
                            self.max_length,
                        )),
                    ),
                )?;
            string
        };
        oxiplate_formatter.write_str(&string)
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
        start_line: 89usize,
        start_col: 4usize,
        end_line: 89usize,
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
impl ::core::fmt::Display for Pad {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write as _;
            let mut string = alloc::string::String::with_capacity(1usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter
                .write_str(
                    &alloc::string::ToString::to_string(
                        &(crate::filters_for_oxiplate::pad(self.number, self.length)),
                    ),
                )?;
            string
        };
        oxiplate_formatter.write_str(&string)
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
        start_line: 120usize,
        start_col: 4usize,
        end_line: 120usize,
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
#[oxiplate_inline(r#"{{ message | respond(false) | shorten(length) }}"#)]
struct Multiple {
    message: &'static str,
    length: usize,
}
impl ::core::fmt::Display for Multiple {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write as _;
            let mut string = alloc::string::String::with_capacity(1usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter
                .write_str(
                    &alloc::string::ToString::to_string(
                        &(crate::filters_for_oxiplate::shorten(
                            crate::filters_for_oxiplate::respond(self.message, false),
                            self.length,
                        )),
                    ),
                )?;
            string
        };
        oxiplate_formatter.write_str(&string)
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
        start_line: 151usize,
        start_col: 4usize,
        end_line: 151usize,
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
#[oxiplate_inline(r#"{{ value | trim }} {{ value | trim() }}"#)]
struct Trim {
    value: &'static str,
}
impl ::core::fmt::Display for Trim {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write as _;
            let mut string = alloc::string::String::with_capacity(3usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter
                .write_str(
                    &alloc::string::ToString::to_string(
                        &(crate::filters_for_oxiplate::trim(self.value)),
                    ),
                )?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(
                    &alloc::string::ToString::to_string(
                        &(crate::filters_for_oxiplate::trim(self.value)),
                    ),
                )?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "trim"]
#[doc(hidden)]
pub const trim: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("trim"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/filters.rs",
        start_line: 191usize,
        start_col: 4usize,
        end_line: 191usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(trim())),
};
fn trim() {
    match (
        &"hi hi",
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", Trim { value: " hi " }))
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
}
#[oxiplate_inline(r#"{{ value | replace("ar", "oo") }}"#)]
struct Replace {
    value: &'static str,
}
impl ::core::fmt::Display for Replace {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write as _;
            let mut string = alloc::string::String::with_capacity(1usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter
                .write_str(
                    &alloc::string::ToString::to_string(
                        &(crate::filters_for_oxiplate::replace(self.value, "ar", "oo")),
                    ),
                )?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "replace"]
#[doc(hidden)]
pub const replace: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("replace"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/filters.rs",
        start_line: 202usize,
        start_col: 4usize,
        end_line: 202usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(replace())),
};
fn replace() {
    match (
        &"boo boo",
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", Replace { value: "bar bar" }))
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
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&multiple, &pad, &replace, &respond, &shorten, &trim])
}
