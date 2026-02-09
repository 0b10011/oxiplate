#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use oxiplate_derive::Oxiplate;
#[oxiplate = "external.html.oxip"]
struct AbsoluteData {
    title: &'static str,
    message: &'static str,
}
impl ::core::fmt::Display for AbsoluteData {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(20usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter.write_str("<h1>")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.title)))?;
            oxiplate_formatter.write_str("</h1>\n<p>")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.message)))?;
            oxiplate_formatter.write_str("</p>\n")?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "absolute"]
#[doc(hidden)]
pub const absolute: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("absolute"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/external.rs",
        start_line: 17usize,
        start_col: 4usize,
        end_line: 17usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(absolute())),
};
fn absolute() {
    let data = AbsoluteData {
        title: "Oxiplate Example",
        message: "Hello world!",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"<h1>Oxiplate Example</h1>\n<p>Hello world!</p>\n",
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
extern crate test;
#[rustc_test_marker = "absolute_2"]
#[doc(hidden)]
pub const absolute_2: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("absolute_2"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/external.rs",
        start_line: 30usize,
        start_col: 4usize,
        end_line: 30usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(absolute_2()),
    ),
};
fn absolute_2() {
    let data = AbsoluteData {
        title: "Oxiplate Example #2",
        message: "Goodbye world!",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"<h1>Oxiplate Example #2</h1>\n<p>Goodbye world!</p>\n",
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
    test::test_main_static(&[&absolute, &absolute_2])
}
