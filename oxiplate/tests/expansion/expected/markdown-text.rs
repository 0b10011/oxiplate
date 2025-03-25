#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::Oxiplate;
#[oxiplate_inline = "{% for message in &messages %}\n{{ md.text: message }}\n{% endfor %}"]
struct Data<'a> {
    messages: Vec<&'a str>,
}
impl<'a> ::std::fmt::Display for Data<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        for message in &self.messages {
            f.write_fmt(
                format_args!(
                    "\n{0}\n",
                    ::oxiplate::escapers::escape(
                        &::oxiplate::escapers::markdown::MarkdownEscaper::Text,
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(format_args!("{0}", message));
                            res
                        }),
                    ),
                ),
            )?;
        }
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "variable"]
#[doc(hidden)]
pub const variable: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("variable"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/markdown-text.rs",
        start_line: 10usize,
        start_col: 4usize,
        end_line: 10usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(variable())),
};
fn variable() {
    let data = Data {
        messages: <[_]>::into_vec(
            ::alloc::boxed::box_new([
                "Hello world!",
                "&reg;</p><script>alert('hey');</script><p>&#153;",
                "\n\n**oh \t no** ",
            ]),
        ),
    };
    match (
        &::alloc::__export::must_use({
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        }),
        &r"
Hello world\!

\&reg\;\<\/p\>\<script\>alert\(\'hey\'\)\;\<\/script\>\<p\>\&\#153\;

\*\*oh no\*\*
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
    test::test_main_static(&[&variable])
}
