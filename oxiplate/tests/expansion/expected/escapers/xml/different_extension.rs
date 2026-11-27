use oxiplate::{Oxiplate, Render};
#[oxiplate = "xml-with-different-extension.tmpl"]
struct Xml {
    name: &'static str,
}
impl ::core::fmt::Display for Xml {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::oxiplate::Render::render_into(self, oxiplate_formatter)
    }
}
impl ::oxiplate::Render for Xml {
    const ESTIMATED_LENGTH: usize = 93usize;
    #[inline]
    fn render_into<W: ::core::fmt::Write>(
        &self,
        oxiplate_formatter: &mut W,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        use ::oxiplate::private::{ToCowStr as _, WriteEscaped as _, WriteRaw as _};
        use ::oxiplate::{Escaper as _, TextHandler as _};
        oxiplate_formatter.write_str("<?xml version=\"1.1\"?>\n<greeting name=\"")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.name)))
                .prepare_text_for_output(&::oxiplate::escapers::xml::XmlEscaper::attr),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::xml::XmlEscaper::attr,
            )?;
        oxiplate_formatter.write_str("\">Hello ")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.name)))
                .prepare_text_for_output(
                    &<::oxiplate::escapers::xml::XmlEscaper as ::oxiplate::Escaper>::DEFAULT,
                ),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &<::oxiplate::escapers::xml::XmlEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("!</greeting>\n<farewell>Goodbye ")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.name)))
                .prepare_text_for_output(&::oxiplate::escapers::xml::XmlEscaper::text),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::xml::XmlEscaper::text,
            )?;
        oxiplate_formatter.write_str("!</farewell>\n")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "escapers::xml::different_extension::xml"]
#[doc(hidden)]
pub static xml: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("escapers::xml::different_extension::xml"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/integration/escapers/xml/different_extension.rs",
        start_line: 10usize,
        start_col: 4usize,
        end_line: 10usize,
        end_col: 7usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(xml()),
    ),
};
fn xml() {
    {
        match (
            &Xml {
                name: r#"Hunt "<xml>" Mill"#,
            }
            .render()
            .unwrap(),
            &r#"<?xml version="1.1"?>
<greeting name="Hunt &quot;&lt;xml>&quot; Mill">Hello Hunt "&lt;xml&gt;" Mill!</greeting>
<farewell>Goodbye Hunt "&lt;xml&gt;" Mill!</farewell>
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
