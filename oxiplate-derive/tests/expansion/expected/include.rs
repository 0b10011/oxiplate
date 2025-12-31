#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline(r#"{% include "extends.html.oxip" %}"#)]
struct Include {
    title: &'static str,
    message: &'static str,
}
impl ::std::fmt::Display for Include {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(55usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter.write_str("<!DOCTYPE html>\n<title>")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(self.title)))?;
            oxiplate_formatter.write_str("</title>\n")?;
            {
                oxiplate_formatter.write_str("<h1>")?;
                oxiplate_formatter
                    .write_str(&::std::string::ToString::to_string(&(self.title)))?;
                oxiplate_formatter.write_str("</h1>\n  <p>")?;
                oxiplate_formatter
                    .write_str(&::std::string::ToString::to_string(&(self.message)))?;
                oxiplate_formatter.write_str("</p>")?;
            }
            oxiplate_formatter.write_str("\n")?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "include"]
#[doc(hidden)]
pub const include: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("include"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/include.rs",
        start_line: 11usize,
        start_col: 4usize,
        end_line: 11usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(include())),
};
fn include() {
    let data = Include {
        title: "Oxiplate Example",
        message: "Hello world!",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"<!DOCTYPE html>\n<title>Oxiplate Example</title>\n<h1>Oxiplate Example</h1>\n  <p>Hello \
         world!</p>\n",
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
#[oxiplate_inline(r#"{% include "include-deep.html.oxip" %}"#)]
struct IncludeDeep {
    title: &'static str,
    message: &'static str,
}
impl ::std::fmt::Display for IncludeDeep {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(32usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter.write_str("<h1>")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(self.title)))?;
            oxiplate_formatter.write_str("</h1>\n")?;
            oxiplate_formatter.write_str("<p>foo</p>\n")?;
            oxiplate_formatter.write_str("\n<p>")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(self.message)))?;
            oxiplate_formatter.write_str("</p>\n")?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "include_deep"]
#[doc(hidden)]
pub const include_deep: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("include_deep"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/include.rs",
        start_line: 32usize,
        start_col: 4usize,
        end_line: 32usize,
        end_col: 16usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(include_deep()),
    ),
};
fn include_deep() {
    let data = IncludeDeep {
        title: "Oxiplate Example",
        message: "Hello world!",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"<h1>Oxiplate Example</h1>\n<p>foo</p>\n\n<p>Hello world!</p>\n",
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
    test::test_main_static(&[&include, &include_deep])
}
