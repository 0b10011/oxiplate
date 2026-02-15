#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use oxiplate_derive::Oxiplate;
mod filters_for_oxiplate {
    use alloc::borrow::{Cow, ToOwned};
    use alloc::format;
    use oxiplate::CowStr;
    pub fn respond<'a, E: CowStr<'a>, R: CowStr<'a>>(
        expression: E,
        response: R,
    ) -> Cow<'a, str> {
        let expression = expression.cow_str();
        let response = response.cow_str();
        match expression.as_ref() {
            "hello" => response,
            _ => {
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("did not understand: {0}", expression),
                        )
                    })
                    .into()
            }
        }
    }
    pub fn shorten<'a, E: CowStr<'a>>(expression: E, max_length: usize) -> Cow<'a, str> {
        let expression = expression.cow_str();
        if expression.len() <= max_length {
            expression
        } else {
            match expression {
                Cow::Borrowed(expression) => {
                    Cow::Borrowed(&expression[0..=max_length - 1])
                }
                Cow::Owned(expression) => {
                    Cow::Owned(expression[0..=max_length - 1].to_owned())
                }
            }
        }
    }
    pub fn pad(expression: usize, max_length: usize) -> Cow<'static, str> {
        ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0:1$}", expression, max_length))
            })
            .into()
    }
}
#[oxiplate_inline(r#"{{ raw: >message | >respond(>"world") }}"#)]
struct Respond {
    message: &'static str,
}
impl ::core::fmt::Display for Respond {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::oxiplate::Render::render_into(self, oxiplate_formatter)
    }
}
impl ::oxiplate::Render for Respond {
    const ESTIMATED_LENGTH: usize = 1usize;
    #[inline]
    fn render_into<W: ::core::fmt::Write>(
        &self,
        oxiplate_formatter: &mut W,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        use ::oxiplate::{ToCowStr as _, UnescapedText as _};
        (&&::oxiplate::UnescapedTextWrapper::new(
            &(::oxiplate::CowStrWrapper::new(
                (&&::oxiplate::ToCowStrWrapper::new(
                    &(crate::filters_for_oxiplate::respond(
                        ::oxiplate::CowStrWrapper::new(
                            (&&::oxiplate::ToCowStrWrapper::new(&(self.message)))
                                .to_cow_str(),
                        ),
                        ::oxiplate::CowStrWrapper::new(
                            (&&::oxiplate::ToCowStrWrapper::new(&("world"))).to_cow_str(),
                        ),
                    )),
                ))
                    .to_cow_str(),
            )),
        ))
            .oxiplate_raw(oxiplate_formatter)?;
        Ok(())
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
        source_file: "oxiplate/tests/filters.rs",
        start_line: 50usize,
        start_col: 4usize,
        end_line: 50usize,
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
#[oxiplate_inline(r#"{{ raw: >message | shorten(max_length) }}"#)]
struct Shorten {
    message: &'static str,
    max_length: usize,
}
impl ::core::fmt::Display for Shorten {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::oxiplate::Render::render_into(self, oxiplate_formatter)
    }
}
impl ::oxiplate::Render for Shorten {
    const ESTIMATED_LENGTH: usize = 1usize;
    #[inline]
    fn render_into<W: ::core::fmt::Write>(
        &self,
        oxiplate_formatter: &mut W,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        use ::oxiplate::{ToCowStr as _, UnescapedText as _};
        (&&::oxiplate::UnescapedTextWrapper::new(
            &(crate::filters_for_oxiplate::shorten(
                ::oxiplate::CowStrWrapper::new(
                    (&&::oxiplate::ToCowStrWrapper::new(&(self.message))).to_cow_str(),
                ),
                self.max_length,
            )),
        ))
            .oxiplate_raw(oxiplate_formatter)?;
        Ok(())
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
        source_file: "oxiplate/tests/filters.rs",
        start_line: 66usize,
        start_col: 4usize,
        end_line: 66usize,
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
#[oxiplate_inline(r#"{{ raw: number | pad(length) }}"#)]
struct Pad {
    number: usize,
    length: usize,
}
impl ::core::fmt::Display for Pad {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::oxiplate::Render::render_into(self, oxiplate_formatter)
    }
}
impl ::oxiplate::Render for Pad {
    const ESTIMATED_LENGTH: usize = 1usize;
    #[inline]
    fn render_into<W: ::core::fmt::Write>(
        &self,
        oxiplate_formatter: &mut W,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        use ::oxiplate::{ToCowStr as _, UnescapedText as _};
        (&&::oxiplate::UnescapedTextWrapper::new(
            &(crate::filters_for_oxiplate::pad(self.number, self.length)),
        ))
            .oxiplate_raw(oxiplate_formatter)?;
        Ok(())
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
        source_file: "oxiplate/tests/filters.rs",
        start_line: 97usize,
        start_col: 4usize,
        end_line: 97usize,
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
#[oxiplate_inline(r#"{{ raw: >message | >respond(>"world") | shorten(length) }}"#)]
struct Multiple {
    message: &'static str,
    length: usize,
}
impl ::core::fmt::Display for Multiple {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::oxiplate::Render::render_into(self, oxiplate_formatter)
    }
}
impl ::oxiplate::Render for Multiple {
    const ESTIMATED_LENGTH: usize = 1usize;
    #[inline]
    fn render_into<W: ::core::fmt::Write>(
        &self,
        oxiplate_formatter: &mut W,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        use ::oxiplate::{ToCowStr as _, UnescapedText as _};
        (&&::oxiplate::UnescapedTextWrapper::new(
            &(crate::filters_for_oxiplate::shorten(
                ::oxiplate::CowStrWrapper::new(
                    (&&::oxiplate::ToCowStrWrapper::new(
                        &(crate::filters_for_oxiplate::respond(
                            ::oxiplate::CowStrWrapper::new(
                                (&&::oxiplate::ToCowStrWrapper::new(&(self.message)))
                                    .to_cow_str(),
                            ),
                            ::oxiplate::CowStrWrapper::new(
                                (&&::oxiplate::ToCowStrWrapper::new(&("world")))
                                    .to_cow_str(),
                            ),
                        )),
                    ))
                        .to_cow_str(),
                ),
                self.length,
            )),
        ))
            .oxiplate_raw(oxiplate_formatter)?;
        Ok(())
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
        source_file: "oxiplate/tests/filters.rs",
        start_line: 128usize,
        start_col: 4usize,
        end_line: 128usize,
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
