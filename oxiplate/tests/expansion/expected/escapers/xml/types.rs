use alloc::string::String;
use core::fmt::{self, Display};

use oxiplate::{Oxiplate, Render};
struct HelloWorld;
impl HelloWorld {
    fn hello() -> String {
        String::from("Hello world &lt;<script><!--")
    }
}
impl Display for HelloWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Hello world &lt;<script><!--")
    }
}
#[oxiplate_inline(
    xml:"
# default:
{{ slice }}
{{ string }}
{{ integer }}
{{ float }}
{{ display }}
{{ display_borrowed }}
{{ fn_string }}

# text:
{{ text: slice }}
{{ text: string }}
{{ text: integer }}
{{ text: float }}
{{ text: display }}
{{ text: display_borrowed }}
{{ text: fn_string }}

# comment:
{{ comment: slice }}
{{ comment: string }}
{{ comment: integer }}
{{ comment: float }}
{{ comment: display }}
{{ comment: display_borrowed }}
{{ comment: fn_string }}

# raw:
{{ raw: slice }}
{{ raw: string }}
{{ raw: integer }}
{{ raw: float }}
{{ raw: display }}
{{ raw: display_borrowed }}
{{ raw: fn_string }}
"
)]
struct Types<'a> {
    slice: &'a str,
    string: String,
    integer: u64,
    float: f64,
    display: HelloWorld,
    display_borrowed: &'a HelloWorld,
    fn_string: String,
}
impl<'a> ::core::fmt::Display for Types<'a> {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::oxiplate::Render::render_into(self, oxiplate_formatter)
    }
}
impl<'a> ::oxiplate::Render for Types<'a> {
    const ESTIMATED_LENGTH: usize = 97usize;
    #[inline]
    fn render_into<W: ::core::fmt::Write>(
        &self,
        oxiplate_formatter: &mut W,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        use ::oxiplate::private::{ToCowStr as _, WriteEscaped as _, WriteRaw as _};
        use ::oxiplate::{Escaper as _, TextHandler as _};
        oxiplate_formatter.write_str("\n# default:\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.slice)))
                .prepare_text_for_output(
                    &<::oxiplate::escapers::xml::XmlEscaper as ::oxiplate::Escaper>::DEFAULT,
                ),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &<::oxiplate::escapers::xml::XmlEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.string)))
                .prepare_text_for_output(
                    &<::oxiplate::escapers::xml::XmlEscaper as ::oxiplate::Escaper>::DEFAULT,
                ),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &<::oxiplate::escapers::xml::XmlEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.integer)))
                .prepare_text_for_output(
                    &<::oxiplate::escapers::xml::XmlEscaper as ::oxiplate::Escaper>::DEFAULT,
                ),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &<::oxiplate::escapers::xml::XmlEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.float)))
                .prepare_text_for_output(
                    &<::oxiplate::escapers::xml::XmlEscaper as ::oxiplate::Escaper>::DEFAULT,
                ),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &<::oxiplate::escapers::xml::XmlEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.display)))
                .prepare_text_for_output(
                    &<::oxiplate::escapers::xml::XmlEscaper as ::oxiplate::Escaper>::DEFAULT,
                ),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &<::oxiplate::escapers::xml::XmlEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.display_borrowed)))
                .prepare_text_for_output(
                    &<::oxiplate::escapers::xml::XmlEscaper as ::oxiplate::Escaper>::DEFAULT,
                ),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &<::oxiplate::escapers::xml::XmlEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.fn_string)))
                .prepare_text_for_output(
                    &<::oxiplate::escapers::xml::XmlEscaper as ::oxiplate::Escaper>::DEFAULT,
                ),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &<::oxiplate::escapers::xml::XmlEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("\n\n# text:\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.slice)))
                .prepare_text_for_output(&::oxiplate::escapers::xml::XmlEscaper::text),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::xml::XmlEscaper::text,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.string)))
                .prepare_text_for_output(&::oxiplate::escapers::xml::XmlEscaper::text),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::xml::XmlEscaper::text,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.integer)))
                .prepare_text_for_output(&::oxiplate::escapers::xml::XmlEscaper::text),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::xml::XmlEscaper::text,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.float)))
                .prepare_text_for_output(&::oxiplate::escapers::xml::XmlEscaper::text),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::xml::XmlEscaper::text,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.display)))
                .prepare_text_for_output(&::oxiplate::escapers::xml::XmlEscaper::text),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::xml::XmlEscaper::text,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.display_borrowed)))
                .prepare_text_for_output(&::oxiplate::escapers::xml::XmlEscaper::text),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::xml::XmlEscaper::text,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.fn_string)))
                .prepare_text_for_output(&::oxiplate::escapers::xml::XmlEscaper::text),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::xml::XmlEscaper::text,
            )?;
        oxiplate_formatter.write_str("\n\n# comment:\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.slice)))
                .prepare_text_for_output(&::oxiplate::escapers::xml::XmlEscaper::comment),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::xml::XmlEscaper::comment,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.string)))
                .prepare_text_for_output(&::oxiplate::escapers::xml::XmlEscaper::comment),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::xml::XmlEscaper::comment,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.integer)))
                .prepare_text_for_output(&::oxiplate::escapers::xml::XmlEscaper::comment),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::xml::XmlEscaper::comment,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.float)))
                .prepare_text_for_output(&::oxiplate::escapers::xml::XmlEscaper::comment),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::xml::XmlEscaper::comment,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.display)))
                .prepare_text_for_output(&::oxiplate::escapers::xml::XmlEscaper::comment),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::xml::XmlEscaper::comment,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.display_borrowed)))
                .prepare_text_for_output(&::oxiplate::escapers::xml::XmlEscaper::comment),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::xml::XmlEscaper::comment,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::xml::XmlEscaper::build_text_handler(&(self.fn_string)))
                .prepare_text_for_output(&::oxiplate::escapers::xml::XmlEscaper::comment),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::xml::XmlEscaper::comment,
            )?;
        oxiplate_formatter.write_str("\n\n# raw:\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            ::oxiplate::PreparedText::AlreadyEscaped(&(self.slice)),
        ))
            .oxiplate_write_raw(oxiplate_formatter)?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            ::oxiplate::PreparedText::AlreadyEscaped(&(self.string)),
        ))
            .oxiplate_write_raw(oxiplate_formatter)?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            ::oxiplate::PreparedText::AlreadyEscaped(&(self.integer)),
        ))
            .oxiplate_write_raw(oxiplate_formatter)?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            ::oxiplate::PreparedText::AlreadyEscaped(&(self.float)),
        ))
            .oxiplate_write_raw(oxiplate_formatter)?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            ::oxiplate::PreparedText::AlreadyEscaped(&(self.display)),
        ))
            .oxiplate_write_raw(oxiplate_formatter)?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            ::oxiplate::PreparedText::AlreadyEscaped(&(self.display_borrowed)),
        ))
            .oxiplate_write_raw(oxiplate_formatter)?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            ::oxiplate::PreparedText::AlreadyEscaped(&(self.fn_string)),
        ))
            .oxiplate_write_raw(oxiplate_formatter)?;
        oxiplate_formatter.write_str("\n")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "escapers::xml::types::types"]
#[doc(hidden)]
pub static types: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("escapers::xml::types::types"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/integration/escapers/xml/types.rs",
        start_line: 69usize,
        start_col: 4usize,
        end_line: 69usize,
        end_col: 9usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(types()),
    ),
};
fn types() {
    let data = Types {
        slice: "Hello world &lt;<script><!--",
        string: String::from("Hello world &lt;<script><!--"),
        integer: 19,
        float: 19.89,
        display: HelloWorld,
        display_borrowed: &HelloWorld,
        fn_string: HelloWorld::hello(),
    };
    {
        match (
            &data.render().unwrap(),
            &r"
# default:
Hello world &amp;lt;&lt;script&gt;&lt;!--
Hello world &amp;lt;&lt;script&gt;&lt;!--
19
19.89
Hello world &amp;lt;&lt;script&gt;&lt;!--
Hello world &amp;lt;&lt;script&gt;&lt;!--
Hello world &amp;lt;&lt;script&gt;&lt;!--

# text:
Hello world &amp;lt;&lt;script&gt;&lt;!--
Hello world &amp;lt;&lt;script&gt;&lt;!--
19
19.89
Hello world &amp;lt;&lt;script&gt;&lt;!--
Hello world &amp;lt;&lt;script&gt;&lt;!--
Hello world &amp;lt;&lt;script&gt;&lt;!--

# comment:
Hello world &lt;<script><!−−
Hello world &lt;<script><!−−
19
19.89
Hello world &lt;<script><!−−
Hello world &lt;<script><!−−
Hello world &lt;<script><!−−

# raw:
Hello world &lt;<script><!--
Hello world &lt;<script><!--
19
19.89
Hello world &lt;<script><!--
Hello world &lt;<script><!--
Hello world &lt;<script><!--
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
