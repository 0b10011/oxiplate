#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use oxiplate_derive::Oxiplate;
#[oxiplate = "extends.html.oxip"]
struct AbsoluteData {
    title: &'static str,
    message: &'static str,
}
impl ::core::fmt::Display for AbsoluteData {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::oxiplate::Render::render_into(self, oxiplate_formatter)
    }
}
impl ::oxiplate::Render for AbsoluteData {
    const ESTIMATED_LENGTH: usize = 55usize;
    #[inline]
    fn render_into<W: ::core::fmt::Write>(
        &self,
        oxiplate_formatter: &mut W,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write;
        use ::oxiplate::{ToCowStr, UnescapedText};
        oxiplate_formatter.write_str("<!DOCTYPE html>\n<title>")?;
        (&&::oxiplate::UnescapedTextWrapper::new(&(self.title)))
            .oxiplate_escape(
                oxiplate_formatter,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("</title>\n")?;
        {
            oxiplate_formatter.write_str("<h1>")?;
            (&&::oxiplate::UnescapedTextWrapper::new(&(self.title)))
                .oxiplate_escape(
                    oxiplate_formatter,
                    &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::Escaper>::DEFAULT,
                )?;
            oxiplate_formatter.write_str("</h1>\n  <p>")?;
            (&&::oxiplate::UnescapedTextWrapper::new(&(self.message)))
                .oxiplate_escape(
                    oxiplate_formatter,
                    &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::Escaper>::DEFAULT,
                )?;
            oxiplate_formatter.write_str("</p>")?;
        }
        oxiplate_formatter.write_str("\n")?;
        Ok(())
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
        source_file: "oxiplate/tests/extends.rs",
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
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&absolute])
}
