use oxiplate::{Oxiplate, Render};
#[oxiplate = "html-with-different-extension.tmpl"]
struct Html {
    name: &'static str,
}
impl ::core::fmt::Display for Html {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::oxiplate::Render::render_into(self, oxiplate_formatter)
    }
}
impl ::oxiplate::Render for Html {
    const ESTIMATED_LENGTH: usize = 66usize;
    #[inline]
    fn render_into<W: ::core::fmt::Write>(
        &self,
        oxiplate_formatter: &mut W,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        use ::oxiplate::private::{ToCowStr as _, WriteEscaped as _, WriteRaw as _};
        use ::oxiplate::{Escaper as _, TextHandler as _};
        oxiplate_formatter.write_str("<!DOCTYPE html>\n<p title=\"Hello ")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::html::HtmlEscaper::build_text_handler(&(self.name)))
                .prepare_text_for_output(&::oxiplate::escapers::html::HtmlEscaper::attr),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::html::HtmlEscaper::attr,
            )?;
        oxiplate_formatter.write_str("\">Hello ")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::html::HtmlEscaper::build_text_handler(&(self.name)))
                .prepare_text_for_output(
                    &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::Escaper>::DEFAULT,
                ),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("!</p>\n<p>Goodbye ")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::html::HtmlEscaper::build_text_handler(&(self.name)))
                .prepare_text_for_output(&::oxiplate::escapers::html::HtmlEscaper::text),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::html::HtmlEscaper::text,
            )?;
        oxiplate_formatter.write_str("!</p>\n")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "escapers::html::different_extension::html"]
#[doc(hidden)]
pub static html: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("escapers::html::different_extension::html"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/integration/escapers/html/different_extension.rs",
        start_line: 10usize,
        start_col: 4usize,
        end_line: 10usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(html()),
    ),
};
fn html() {
    {
        match (
            &Html {
                name: r#"Hunt "<html>" Mill"#,
            }
            .render()
            .unwrap(),
            &r#"<!DOCTYPE html>
<p title="Hello Hunt &#34;<html>&#34; Mill">Hello Hunt "&lt;html>" Mill!</p>
<p>Goodbye Hunt "&lt;html>" Mill!</p>
"#,
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
