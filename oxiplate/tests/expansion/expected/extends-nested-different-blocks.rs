#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::{Oxiplate, Render};
#[oxiplate = "./extends-nested-different-blocks.html.oxip"]
struct AbsoluteData {
    title: &'static str,
    message: &'static str,
}
impl ::std::fmt::Display for AbsoluteData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::oxiplate::Render::render_into(self, f)
    }
}
impl ::oxiplate::Render for AbsoluteData {
    const ESTIMATED_LENGTH: usize = 97usize;
    #[inline]
    fn render_into<W: ::std::fmt::Write>(&self, f: &mut W) -> ::std::fmt::Result {
        use ::std::fmt::Write;
        use ::oxiplate::unescaped_text::UnescapedText;
        f.write_str("<DOCTYPE html>\n<head>\n  <title>")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&self.title))
            .oxiplate_escape(
                f,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
            )?;
        f.write_str("</title>\n</head>\n<body>")?;
        f.write_str("<main>")?;
        f.write_str("<h1>")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&self.title))
            .oxiplate_escape(
                f,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
            )?;
        f.write_str("</h1>\n  <p>")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&self.message))
            .oxiplate_escape(
                f,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
            )?;
        f.write_str("</p>")?;
        f.write_str("</main>")?;
        f.write_str("</body>\n")?;
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
        source_file: "oxiplate/tests/extends-nested-different-blocks.rs",
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
        &"<DOCTYPE html>\n<head>\n  <title>Oxiplate \
         Example</title>\n</head>\n<body><main><h1>Oxiplate Example</h1>\n  <p>Hello \
         world!</p></main></body>\n",
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
