#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::fmt::Display;
use oxiplate::{Oxiplate, Render};
struct HelloWorld;
impl HelloWorld {
    fn hello() -> String {
        String::from("Hello world &lt;<script><!--")
    }
}
impl Display for HelloWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Hello world &lt;<script><!--")
    }
}
#[oxiplate_inline(
    html:"
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
impl<'a> ::std::fmt::Display for Types<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::oxiplate::Render::render_into(self, f)
    }
}
impl<'a> ::oxiplate::Render for Types<'a> {
    const ESTIMATED_LENGTH: usize = 97usize;
    #[inline]
    fn render_into<W: ::std::fmt::Write>(&self, f: &mut W) -> ::std::fmt::Result {
        use ::std::fmt::Write;
        use ::oxiplate::unescaped_text::UnescapedText;
        f.write_str("\n# default:\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.slice)))
            .oxiplate_escape(
                f,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
            )?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.string)))
            .oxiplate_escape(
                f,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
            )?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.integer)))
            .oxiplate_escape(
                f,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
            )?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.float)))
            .oxiplate_escape(
                f,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
            )?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.display)))
            .oxiplate_escape(
                f,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
            )?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(
            &(self.display_borrowed),
        ))
            .oxiplate_escape(
                f,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
            )?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.fn_string)))
            .oxiplate_escape(
                f,
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
            )?;
        f.write_str("\n\n# text:\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.slice)))
            .oxiplate_escape(f, &::oxiplate::escapers::html::HtmlEscaper::text)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.string)))
            .oxiplate_escape(f, &::oxiplate::escapers::html::HtmlEscaper::text)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.integer)))
            .oxiplate_escape(f, &::oxiplate::escapers::html::HtmlEscaper::text)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.float)))
            .oxiplate_escape(f, &::oxiplate::escapers::html::HtmlEscaper::text)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.display)))
            .oxiplate_escape(f, &::oxiplate::escapers::html::HtmlEscaper::text)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(
            &(self.display_borrowed),
        ))
            .oxiplate_escape(f, &::oxiplate::escapers::html::HtmlEscaper::text)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.fn_string)))
            .oxiplate_escape(f, &::oxiplate::escapers::html::HtmlEscaper::text)?;
        f.write_str("\n\n# comment:\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.slice)))
            .oxiplate_escape(f, &::oxiplate::escapers::html::HtmlEscaper::comment)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.string)))
            .oxiplate_escape(f, &::oxiplate::escapers::html::HtmlEscaper::comment)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.integer)))
            .oxiplate_escape(f, &::oxiplate::escapers::html::HtmlEscaper::comment)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.float)))
            .oxiplate_escape(f, &::oxiplate::escapers::html::HtmlEscaper::comment)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.display)))
            .oxiplate_escape(f, &::oxiplate::escapers::html::HtmlEscaper::comment)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(
            &(self.display_borrowed),
        ))
            .oxiplate_escape(f, &::oxiplate::escapers::html::HtmlEscaper::comment)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.fn_string)))
            .oxiplate_escape(f, &::oxiplate::escapers::html::HtmlEscaper::comment)?;
        f.write_str("\n\n# raw:\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.slice)))
            .oxiplate_raw(f)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.string)))
            .oxiplate_raw(f)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.integer)))
            .oxiplate_raw(f)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.float)))
            .oxiplate_raw(f)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.display)))
            .oxiplate_raw(f)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(
            &(self.display_borrowed),
        ))
            .oxiplate_raw(f)?;
        f.write_str("\n")?;
        (&&::oxiplate::unescaped_text::UnescapedTextWrapper::new(&(self.fn_string)))
            .oxiplate_raw(f)?;
        f.write_str("\n")?;
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "types"]
#[doc(hidden)]
pub const types: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("types"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/escaping.rs",
        start_line: 68usize,
        start_col: 4usize,
        end_line: 68usize,
        end_col: 9usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(types())),
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
    match (
        &data.render().unwrap(),
        &r"
# default:
Hello world &amp;lt;&lt;script>&lt;!--
Hello world &amp;lt;&lt;script>&lt;!--
19
19.89
Hello world &amp;lt;&lt;script>&lt;!--
Hello world &amp;lt;&lt;script>&lt;!--
Hello world &amp;lt;&lt;script>&lt;!--

# text:
Hello world &amp;lt;&lt;script>&lt;!--
Hello world &amp;lt;&lt;script>&lt;!--
19
19.89
Hello world &amp;lt;&lt;script>&lt;!--
Hello world &amp;lt;&lt;script>&lt;!--
Hello world &amp;lt;&lt;script>&lt;!--

# comment:
Hello world &lt;‹script›‹ǃ−−
Hello world &lt;‹script›‹ǃ−−
19
19.89
Hello world &lt;‹script›‹ǃ−−
Hello world &lt;‹script›‹ǃ−−
Hello world &lt;‹script›‹ǃ−−

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
    };
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&types])
}
