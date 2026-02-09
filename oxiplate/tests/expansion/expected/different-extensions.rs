#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
use oxiplate::{Oxiplate, Render};
#[oxiplate = "html-with-different-extension.tmpl"]
struct Html {
    name: &'static str,
}
impl ::core::fmt::Display for Html {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
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
        use ::core::fmt::Write;
        use ::oxiplate::{ToCowStr, UnescapedText};
        oxiplate_formatter.write_str("<!DOCTYPE html>\n<p title=\"Hello ")?;
        (&&::oxiplate::UnescapedTextWrapper::new(&(self.name)))
            .oxiplate_escape(
                oxiplate_formatter,
                &::oxiplate::escapers::html::HtmlEscaper::attr,
            )?;
        oxiplate_formatter.write_str("\">Hello ")?;
        (&&::oxiplate::UnescapedTextWrapper::new(&(self.name)))
            .oxiplate_escape(
                oxiplate_formatter,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("!</p>\n<p>Goodbye ")?;
        (&&::oxiplate::UnescapedTextWrapper::new(&(self.name)))
            .oxiplate_escape(
                oxiplate_formatter,
                &::oxiplate::escapers::html::HtmlEscaper::text,
            )?;
        oxiplate_formatter.write_str("!</p>\n")?;
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "html"]
#[doc(hidden)]
pub const html: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("html"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/different-extensions.rs",
        start_line: 12usize,
        start_col: 4usize,
        end_line: 12usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(html())),
};
fn html() {
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
    };
}
#[oxiplate = "json-with-different-extension.tmpl"]
struct Json {
    name: &'static str,
}
impl ::core::fmt::Display for Json {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
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
        use ::core::fmt::Write;
        use ::oxiplate::{ToCowStr, UnescapedText};
        oxiplate_formatter.write_str("{\n    \"foo\": \"hello ")?;
        (&&::oxiplate::UnescapedTextWrapper::new(&(self.name)))
            .oxiplate_escape(
                oxiplate_formatter,
                &<::oxiplate::escapers::json::JsonEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("\",\n    \"bar\": \"goodbye ")?;
        (&&::oxiplate::UnescapedTextWrapper::new(&(self.name)))
            .oxiplate_escape(
                oxiplate_formatter,
                &::oxiplate::escapers::json::JsonEscaper::substring,
            )?;
        oxiplate_formatter.write_str("\"\n}\n")?;
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "json"]
#[doc(hidden)]
pub const json: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("json"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/different-extensions.rs",
        start_line: 33usize,
        start_col: 4usize,
        end_line: 33usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(json())),
};
fn json() {
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
    };
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&html, &json])
}
