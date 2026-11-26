use alloc::vec;
use alloc::vec::Vec;

use oxiplate::{Oxiplate, Render};
#[oxiplate_inline(
    xml:"{% for message in &messages %}\n<p>{{ text: message }}</p>{% endfor %}\n"
)]
struct Data<'a> {
    messages: Vec<&'a str>,
}
impl<'a> ::core::fmt::Display for Data<'a> {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
        use ::oxiplate::private::{ToCowStr as _, WriteEscaped as _, WriteRaw as _};
        use ::oxiplate::{Escaper as _, TextHandler as _};
        for message in &self.messages {
            oxiplate_formatter.write_str("\n<p>")?;
            (&&::oxiplate::private::UnescapedTextWrapper::new(
                (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(message)))
                    .prepare_text_for_output(&::oxiplate::escapers::xml::XmlEscaper::text),
            ))
                .oxiplate_write_escaped(
                    oxiplate_formatter,
                    &::oxiplate::escapers::xml::XmlEscaper::text,
                )?;
            oxiplate_formatter.write_str("</p>")?;
        }
        oxiplate_formatter.write_str("\n")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "escapers::xml::text::variable"]
#[doc(hidden)]
pub static variable: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("escapers::xml::text::variable"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/integration/escapers/xml/text.rs",
        start_line: 13usize,
        start_col: 4usize,
        end_line: 13usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(variable()),
    ),
};
fn variable() {
    let data = Data {
        messages: ::alloc::boxed::box_assume_init_into_vec_unsafe(
            ::alloc::intrinsics::write_box_via_move(
                ::alloc::boxed::Box::new_uninit(),
                [
                    "Hello world!",
                    "&reg;</p><script>alert('hey');</script><p>&#153;",
                ],
            ),
        ),
    };
    {
        match (
            &data.render().unwrap(),
            &r"
<p>Hello world!</p>
<p>&amp;reg;&lt;/p&gt;&lt;script&gt;alert('hey');&lt;/script&gt;&lt;p&gt;&amp;#153;</p>
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
        }
    };
}
