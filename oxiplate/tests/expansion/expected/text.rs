#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::{Oxiplate, Render};
#[oxiplate_inline(
    html:"{% for message in &messages %}\n<p>{{ text: message }}</p>{% endfor %}\n"
)]
struct Data<'a> {
    messages: Vec<&'a str>,
}
impl<'a> ::std::fmt::Display for Data<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::oxiplate::Render::render_into(self, f)
    }
}
impl<'a> ::oxiplate::Render for Data<'a> {
    const ESTIMATED_LENGTH: usize = 19usize;
    #[inline]
    fn render_into<W: ::std::fmt::Write>(&self, f: &mut W) -> ::std::fmt::Result {
        use ::std::fmt::Write;
        for message in (&self.messages) {
            f.write_str("\n<p>")?;
            ::oxiplate::escapers::escape(
                f,
                &::oxiplate::escapers::html::HtmlEscaper::Text,
                &::std::string::ToString::to_string(&message),
            )?;
            f.write_str("</p>")?;
        }
        f.write_str("\n")?;
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
        start_line: 10usize,
        start_col: 4usize,
        end_line: 10usize,
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
