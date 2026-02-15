#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use oxiplate::{Oxiplate, Render};
#[oxiplate_inline(
    html:"{% for message in &messages %}\n<p>{{ text: message }}</p>{% endfor %}\n"
)]
struct Data<'a> {
    messages: Vec<&'a str>,
}
impl<'a> ::core::fmt::Display for Data<'a> {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::oxiplate::Render::render_into(self, oxiplate_formatter)
    }
}
impl<'a> ::oxiplate::Render for Data<'a> {
    const ESTIMATED_LENGTH: usize = 19usize;
    #[inline]
    fn render_into<W: ::core::fmt::Write>(
        &self,
        oxiplate_formatter: &mut W,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        use ::oxiplate::{ToCowStr as _, UnescapedText as _};
        for message in &self.messages {
            oxiplate_formatter.write_str("\n<p>")?;
            (&&::oxiplate::UnescapedTextWrapper::new(&(message)))
                .oxiplate_escape(
                    oxiplate_formatter,
                    &::oxiplate::escapers::html::HtmlEscaper::text,
                )?;
            oxiplate_formatter.write_str("</p>")?;
        }
        oxiplate_formatter.write_str("\n")?;
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "variable"]
#[doc(hidden)]
pub const variable: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("variable"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/text.rs",
        start_line: 17usize,
        start_col: 4usize,
        end_line: 17usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(variable())),
};
fn variable() {
    let data = Data {
        messages: <[_]>::into_vec(
            ::alloc::boxed::box_new([
                "Hello world!",
                "&reg;</p><script>alert('hey');</script><p>&#153;",
            ]),
        ),
    };
    match (
        &data.render().unwrap(),
        &r"
<p>Hello world!</p>
<p>&amp;reg;&lt;/p>&lt;script>alert('hey');&lt;/script>&lt;p>&amp;#153;</p>
",
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
    test::test_main_static(&[&variable])
}
