#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use oxiplate::{Oxiplate, Render};
#[oxiplate = "replace-escaper.html.oxip"]
struct Html {
    name: &'static str,
}
impl ::std::fmt::Display for Html {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::fmt::Result {
        ::oxiplate::Render::render_into(self, oxiplate_formatter)
    }
}
impl ::oxiplate::Render for Html {
    const ESTIMATED_LENGTH: usize = 7usize;
    #[inline]
    fn render_into<W: ::std::fmt::Write>(
        &self,
        oxiplate_formatter: &mut W,
    ) -> ::std::fmt::Result {
        use ::std::fmt::Write;
        use ::oxiplate::{ToCowStr, UnescapedText};
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::UnescapedTextWrapper::new(&(self.name)))
            .oxiplate_escape(
                oxiplate_formatter,
                &<::oxiplate::escapers::your_group::YourEscaper as ::oxiplate::Escaper>::DEFAULT,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::UnescapedTextWrapper::new(&(self.name)))
            .oxiplate_escape(
                oxiplate_formatter,
                &::oxiplate::escapers::your_group::YourEscaper::foo,
            )?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::UnescapedTextWrapper::new(&(self.name)))
            .oxiplate_escape(
                oxiplate_formatter,
                &::oxiplate::escapers::your_group::YourEscaper::bar,
            )?;
        oxiplate_formatter.write_str("\n")?;
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
        source_file: "oxiplate/tests/replace-escaper.rs",
        start_line: 10usize,
        start_col: 4usize,
        end_line: 10usize,
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
        &Html { name: r#"foo bar"# }.render().unwrap(),
        &r#"
f00 bar
f00 bar
foo b@r
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
    match (&Html { name: r#"hello"# }.render().unwrap(), &r#"
hello
hello
hello
"#) {
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
    test::test_main_static(&[&html])
}
