#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use oxiplate::Oxiplate;
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
        ::oxiplate::Render::render_into(self, oxiplate_formatter)
    }
}
impl ::oxiplate::Render for Include {
    const ESTIMATED_LENGTH: usize = 55usize;
    #[inline]
    fn render_into<W: ::std::fmt::Write>(
        &self,
        oxiplate_formatter: &mut W,
    ) -> ::std::fmt::Result {
        use ::std::fmt::Write;
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
#[rustc_test_marker = "include"]
#[doc(hidden)]
pub const include: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("include"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/include.rs",
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
        ::oxiplate::Render::render_into(self, oxiplate_formatter)
    }
}
impl ::oxiplate::Render for IncludeDeep {
    const ESTIMATED_LENGTH: usize = 32usize;
    #[inline]
    fn render_into<W: ::std::fmt::Write>(
        &self,
        oxiplate_formatter: &mut W,
    ) -> ::std::fmt::Result {
        use ::std::fmt::Write;
        use ::oxiplate::{ToCowStr, UnescapedText};
        oxiplate_formatter.write_str("<h1>")?;
        (&&::oxiplate::UnescapedTextWrapper::new(&(self.title)))
            .oxiplate_escape(
                oxiplate_formatter,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("</h1>\n")?;
        oxiplate_formatter.write_str("<p>foo</p>\n")?;
        oxiplate_formatter.write_str("\n<p>")?;
        (&&::oxiplate::UnescapedTextWrapper::new(&(self.message)))
            .oxiplate_escape(
                oxiplate_formatter,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("</p>\n")?;
        Ok(())
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
        source_file: "oxiplate/tests/include.rs",
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
