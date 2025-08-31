#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use oxiplate::{Oxiplate, Render};
#[oxiplate_inline(html:"<!--{{ comment: comment }}-->")]
struct Data<'a> {
    comment: &'a str,
}
impl<'a> ::std::fmt::Display for Data<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::oxiplate::Render::render_into(self, f)
    }
}
impl<'a> ::oxiplate::Render for Data<'a> {
    const ESTIMATED_LENGTH: usize = 8usize;
    #[inline]
    fn render_into<W: ::std::fmt::Write>(&self, f: &mut W) -> ::std::fmt::Result {
        use ::std::fmt::Write;
        use ::oxiplate::UnescapedText;
        f.write_str("<!--")?;
        (&&::oxiplate::UnescapedTextWrapper::new(&(self.comment)))
            .oxiplate_escape(f, &::oxiplate::escapers::html::HtmlEscaper::comment)?;
        f.write_str("-->")?;
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "comment"]
#[doc(hidden)]
pub const comment: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("comment"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/comment-escaping.rs",
        start_line: 10usize,
        start_col: 4usize,
        end_line: 10usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(comment())),
};
fn comment() {
    let comments = [
        (
            "<tags> and hyphenated-text are fine!",
            "<!--<tags> and hyphenated-text are fine!-->",
            "Comment characters are normally fine as long as they're not in a special place or \
             grouped with others in a specific way",
        ),
        ("> hello", "<!--› hello-->", "Text must not start with the string `>`"),
        ("-> hey", "<!--−› hey-->", "Text must not start with the string `->`"),
        (
            "hello <!-- world",
            "<!--hello ‹ǃ−− world-->",
            "Text must not contain the string `<!--`",
        ),
        (
            "foo --> bar",
            "<!--foo −−› bar-->",
            "Text must not contain the string `-->`",
        ),
        (
            "baz --!> qux",
            "<!--baz −−ǃ› qux-->",
            "Text must not contain the string `--!>`",
        ),
        ("hey <!-", "<!--hey ‹ǃ−-->", "Text must not end with the string `<!-`"),
        (
            "- hi",
            "<!--− hi-->",
            "Hyphens at the beginning of a comment are not allowed in XML because it can cause \
             double hyphens",
        ),
        (
            "--- hi",
            "<!--−−− hi-->",
            "Hyphens at the beginning of a comment are not allowed in XML because it can cause \
             double hyphens",
        ),
        (
            "hi -",
            "<!--hi −-->",
            "Hyphens at the end of a comment are not allowed in XML because it can cause double \
             hyphens",
        ),
        (
            "hi ---",
            "<!--hi −−−-->",
            "Hyphens at the end of a comment are not allowed in XML because it can cause double \
             hyphens",
        ),
        ("hi--bye", "<!--hi−−bye-->", "Double hyphens are not allowed in XML"),
        ("hi---bye", "<!--hi−−−bye-->", "Double hyphens are not allowed in XML"),
    ];
    for (comment, expected, reason) in comments {
        let data = Data { comment };
        match (&data.render().unwrap(), &expected) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(format_args!("{0}", reason)),
                    );
                }
            }
        };
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&comment])
}
