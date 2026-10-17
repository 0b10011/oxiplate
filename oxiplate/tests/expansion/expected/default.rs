use oxiplate::prelude::*;
#[oxiplate_inline(
    html:r#"
<!DOCTYPE html>
<title>{{ title | default("Default title") }}</title>
{% if let Some(title) = title -%}
    <h1>{{ title }}</h1>
{% endif -%}
"#
)]
struct Data {
    title: Option<&'static str>,
}
impl ::core::fmt::Display for Data {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::oxiplate::Render::render_into(self, oxiplate_formatter)
    }
}
impl ::oxiplate::Render for Data {
    const ESTIMATED_LENGTH: usize = 45usize;
    #[inline]
    fn render_into<W: ::core::fmt::Write>(
        &self,
        oxiplate_formatter: &mut W,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        use ::oxiplate::{ToCowStr as _, UnescapedText as _};
        oxiplate_formatter.write_str("\n<!DOCTYPE html>\n<title>")?;
        (&&::oxiplate::UnescapedTextWrapper::new(
            &(filters_for_oxiplate::default(self.title, "Default title")),
        ))
            .oxiplate_escape(
                oxiplate_formatter,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("</title>\n")?;
        if let Some(title) = self.title {
            oxiplate_formatter.write_str("<h1>")?;
            (&&::oxiplate::UnescapedTextWrapper::new(&(title))).oxiplate_escape(
                oxiplate_formatter,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
            oxiplate_formatter.write_str("</h1>\n")?;
        }
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "default::some"]
#[doc(hidden)]
pub static some: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("default::some"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/integration/default.rs",
        start_line: 16usize,
        start_col: 4usize,
        end_line: 16usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(some()),
    ),
};
fn some() {
    {
        match (
            &r#"
<!DOCTYPE html>
<title>Hello world!</title>
<h1>Hello world!</h1>
"#,
            &Data {
                title: Some("Hello world!"),
            }
            .render()
            .unwrap(),
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
    }
}
extern crate test;
#[rustc_test_marker = "default::none"]
#[doc(hidden)]
pub static none: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("default::none"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/integration/default.rs",
        start_line: 32usize,
        start_col: 4usize,
        end_line: 32usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(none()),
    ),
};
fn none() {
    {
        match (
            &r#"
<!DOCTYPE html>
<title>Default title</title>
"#,
            &Data { title: None }.render().unwrap(),
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
    }
}
