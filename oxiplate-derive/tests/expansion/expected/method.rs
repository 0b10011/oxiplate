#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use alloc::string::String;
use oxiplate_derive::Oxiplate;
struct User {
    name: &'static str,
    company: &'static str,
}
impl User {
    pub fn display_name(&self) -> String {
        ::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0} ({1})", self.company, self.name))
        })
    }
}
#[oxiplate_inline("{{ user.display_name() }}")]
struct Data {
    user: User,
}
impl ::core::fmt::Display for Data {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(1usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter
                .write_str(
                    &alloc::string::ToString::to_string(&(self.user.display_name())),
                )?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "field"]
#[doc(hidden)]
pub const field: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("field"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/method.rs",
        start_line: 27usize,
        start_col: 4usize,
        end_line: 27usize,
        end_col: 9usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(field())),
};
fn field() {
    let data = Data {
        user: User {
            name: "Kiera",
            company: "Floating Air LLC",
        },
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"Floating Air LLC (Kiera)",
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
#[oxiplate_inline(r#"{% if user.display_name().contains("i") %}yup!{% endif %}"#)]
struct Argument {
    user: User,
}
impl ::core::fmt::Display for Argument {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(4usize);
            let oxiplate_formatter = &mut string;
            if self.user.display_name().contains("i") {
                oxiplate_formatter.write_str("yup!")?;
            }
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "field_with_argument"]
#[doc(hidden)]
pub const field_with_argument: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("field_with_argument"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/method.rs",
        start_line: 45usize,
        start_col: 4usize,
        end_line: 45usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(field_with_argument()),
    ),
};
fn field_with_argument() {
    let data = Argument {
        user: User {
            name: "Kiera",
            company: "Floating Air LLC",
        },
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"yup!",
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
#[oxiplate_inline(
    r#"{{ user.display_name().replace("a", "@",) }} {{ user.display_name().replace("a", "@") }}"#
)]
struct Arguments {
    user: User,
}
impl ::core::fmt::Display for Arguments {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(3usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter
                .write_str(
                    &alloc::string::ToString::to_string(
                        &(self.user.display_name().replace("a", "@")),
                    ),
                )?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(
                    &alloc::string::ToString::to_string(
                        &(self.user.display_name().replace("a", "@")),
                    ),
                )?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "field_with_arguments"]
#[doc(hidden)]
pub const field_with_arguments: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("field_with_arguments"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/method.rs",
        start_line: 65usize,
        start_col: 4usize,
        end_line: 65usize,
        end_col: 24usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(field_with_arguments()),
    ),
};
fn field_with_arguments() {
    let data = Arguments {
        user: User {
            name: "Kiera",
            company: "Floating Air LLC",
        },
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"Flo@ting Air LLC (Kier@) Flo@ting Air LLC (Kier@)",
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
#[oxiplate_inline(r#"{{ foo() }}"#)]
struct Callback {
    foo: fn() -> &'static str,
}
impl ::core::fmt::Display for Callback {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(1usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&((self.foo)())))?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "callback"]
#[doc(hidden)]
pub const callback: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("callback"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/method.rs",
        start_line: 86usize,
        start_col: 4usize,
        end_line: 86usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(callback())),
};
fn callback() {
    match (
        &"hello world",
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", Callback { foo: || "hello world" }))
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
    test::test_main_static(
        &[&callback, &field, &field_with_argument, &field_with_arguments],
    )
}
