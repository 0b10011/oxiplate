use oxiplate::{Oxiplate, Render};
#[oxiplate = "json-with-different-extension.tmpl"]
struct Json {
    name: &'static str,
}
impl ::core::fmt::Display for Json {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::oxiplate::Render::render_into(self, oxiplate_formatter)
    }
}
impl ::oxiplate::Render for Json {
    const ESTIMATED_LENGTH: usize = 49usize;
    #[inline]
    fn render_into<W: ::core::fmt::Write>(
        &self,
        oxiplate_formatter: &mut W,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        use ::oxiplate::private::{ToCowStr as _, WriteEscaped as _, WriteRaw as _};
        use ::oxiplate::{Escaper as _, TextHandler as _};
        oxiplate_formatter.write_str("{\n    \"foo\": \"hello ")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::json::JsonEscaper::build_text_handler(&(self.name)))
                .prepare_text_for_output(
                    &<::oxiplate::escapers::json::JsonEscaper as ::oxiplate::Escaper>::DEFAULT,
                ),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &<::oxiplate::escapers::json::JsonEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("\",\n    \"bar\": \"goodbye ")?;
        (&&::oxiplate::private::UnescapedTextWrapper::new(
            (&&::oxiplate::escapers::json::JsonEscaper::build_text_handler(&(self.name)))
                .prepare_text_for_output(&::oxiplate::escapers::json::JsonEscaper::substring),
        ))
            .oxiplate_write_escaped(
                oxiplate_formatter,
                &::oxiplate::escapers::json::JsonEscaper::substring,
            )?;
        oxiplate_formatter.write_str("\"\n}\n")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "escapers::json::different_extension::json"]
#[doc(hidden)]
pub static json: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("escapers::json::different_extension::json"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/integration/escapers/json/different_extension.rs",
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
        || test::assert_test_result(json()),
    ),
};
fn json() {
    {
        match (
            &Json {
                name: r#"Jane "JSON" Sonder"#,
            }
            .render()
            .unwrap(),
            &r#"{
    "foo": "hello Jane \"JSON\" Sonder",
    "bar": "goodbye Jane \"JSON\" Sonder"
}
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
