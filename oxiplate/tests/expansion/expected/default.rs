#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
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
impl ::std::fmt::Display for Data {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::fmt::Result {
        ::oxiplate::Render::render_into(self, oxiplate_formatter)
    }
}
impl ::oxiplate::Render for Data {
    const ESTIMATED_LENGTH: usize = 45usize;
    #[inline]
    fn render_into<W: ::std::fmt::Write>(
        &self,
        oxiplate_formatter: &mut W,
    ) -> ::std::fmt::Result {
        use ::std::fmt::Write;
        use ::oxiplate::{ToCowStr, UnescapedText};
        oxiplate_formatter.write_str("\n<!DOCTYPE html>\n<title>")?;
        (&&::oxiplate::UnescapedTextWrapper::new(
            &(crate::filters_for_oxiplate::default(self.title, "Default title")),
        ))
            .oxiplate_escape(
                oxiplate_formatter,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("</title>\n")?;
        if let Some(title) = self.title {
            oxiplate_formatter.write_str("<h1>")?;
            (&&::oxiplate::UnescapedTextWrapper::new(&(title)))
                .oxiplate_escape(
                    oxiplate_formatter,
                    &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::Escaper>::DEFAULT,
                )?;
            oxiplate_formatter.write_str("</h1>\n")?;
        }
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "some"]
#[doc(hidden)]
pub const some: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("some"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/default.rs",
        start_line: 16usize,
        start_col: 4usize,
        end_line: 16usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(some())),
};
fn some() {
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
extern crate test;
#[rustc_test_marker = "none"]
#[doc(hidden)]
pub const none: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("none"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/default.rs",
        start_line: 32usize,
        start_col: 4usize,
        end_line: 32usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(none())),
};
fn none() {
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
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&none, &some])
}
