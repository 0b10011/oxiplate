#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::{Oxiplate, Render};
#[oxiplate = "extends-deep.html.oxip"]
struct AbsoluteData<'a> {
    title: &'a str,
    message: &'a str,
}
impl<'a> ::std::fmt::Display for AbsoluteData<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::oxiplate::Render::render_into(self, f)
    }
}
impl<'a> ::oxiplate::Render for AbsoluteData<'a> {
    const ESTIMATED_LENGTH: usize = 59usize;
    #[inline]
    fn render_into<W: ::std::fmt::Write>(&self, f: &mut W) -> ::std::fmt::Result {
        use ::std::fmt::Write;
        use ::oxiplate::unescaped_text::UnescapedText;
        f.write_str("<!DOCTYPE html>\n<title>")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&self.title))
            .oxiplate_escape(
                f,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
            )?;
        f.write_str("</title>\n")?;
        f.write_str("<h2>")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&self.title))
            .oxiplate_escape(
                f,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
            )?;
        f.write_str("</h2>\n  <div>")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&self.message))
            .oxiplate_escape(
                f,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
            )?;
        f.write_str("</div>")?;
        f.write_str("\n")?;
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
        source_file: "oxiplate/tests/extends-lifetime.rs",
        start_line: 11usize,
        start_col: 4usize,
        end_line: 11usize,
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
        &data.render().unwrap(),
        &"<!DOCTYPE html>\n<title>Oxiplate Example</title>\n<h2>Oxiplate Example</h2>\n  \
         <div>Hello world!</div>\n",
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
