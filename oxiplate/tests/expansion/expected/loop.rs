#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use oxiplate::prelude::*;
#[oxiplate_inline(
    html:"
{% for (loop, value) in &values | loop -%}
    {% if loop.is_first -%}
        first:
    {%_ endif -%}

    #{{ loop.index1 }} (#{{ loop.index0 }}) {{ value }}
{% endfor %}"
)]
struct Loop {
    values: Vec<usize>,
}
impl ::std::fmt::Display for Loop {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::fmt::Result {
        ::oxiplate::Render::render_into(self, oxiplate_formatter)
    }
}
impl ::oxiplate::Render for Loop {
    const ESTIMATED_LENGTH: usize = 35usize;
    #[inline]
    fn render_into<W: ::std::fmt::Write>(
        &self,
        oxiplate_formatter: &mut W,
    ) -> ::std::fmt::Result {
        use ::std::fmt::Write;
        use ::oxiplate::{ToCowStr, UnescapedText};
        oxiplate_formatter.write_str("\n")?;
        for (r#loop, value) in crate::filters_for_oxiplate::r#loop(&self.values) {
            if r#loop.is_first {
                oxiplate_formatter.write_str("first: ")?;
            }
            oxiplate_formatter.write_str("#")?;
            (&&::oxiplate::UnescapedTextWrapper::new(&(r#loop.index1)))
                .oxiplate_escape(
                    oxiplate_formatter,
                    &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::Escaper>::DEFAULT,
                )?;
            oxiplate_formatter.write_str(" (#")?;
            (&&::oxiplate::UnescapedTextWrapper::new(&(r#loop.index0)))
                .oxiplate_escape(
                    oxiplate_formatter,
                    &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::Escaper>::DEFAULT,
                )?;
            oxiplate_formatter.write_str(") ")?;
            (&&::oxiplate::UnescapedTextWrapper::new(&(value)))
                .oxiplate_escape(
                    oxiplate_formatter,
                    &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::Escaper>::DEFAULT,
                )?;
            oxiplate_formatter.write_str("\n")?;
        }
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "test_loop"]
#[doc(hidden)]
pub const test_loop: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_loop"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/loop.rs",
        start_line: 18usize,
        start_col: 4usize,
        end_line: 18usize,
        end_col: 13usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test_loop())),
};
fn test_loop() {
    let data = Loop {
        values: <[_]>::into_vec(::alloc::boxed::box_new([19, 89, 42])),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &r"
first: #1 (#0) 19
#2 (#1) 89
#3 (#2) 42
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
    test::test_main_static(&[&test_loop])
}
