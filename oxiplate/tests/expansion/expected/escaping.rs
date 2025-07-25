#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::fmt::Display;
use oxiplate::Oxiplate;
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
{{ fn_string }}

# text:
{{ text: slice }}
{{ text: string }}
{{ text: integer }}
{{ text: float }}
{{ text: display }}
{{ text: fn_string }}

# comment:
{{ comment: slice }}
{{ comment: string }}
{{ comment: integer }}
{{ comment: float }}
{{ comment: display }}
{{ comment: fn_string }}

# raw:
{{ raw: slice }}
{{ raw: string }}
{{ raw: integer }}
{{ raw: float }}
{{ raw: display }}
{{ raw: fn_string }}
"
)]
struct Types<'a> {
    slice: &'a str,
    string: String,
    integer: u64,
    float: f64,
    display: HelloWorld,
    fn_string: String,
}
impl<'a> ::std::fmt::Display for Types<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::oxiplate::Render::render(self, f)
    }
}
impl<'a> ::oxiplate::Render for Types<'a> {
    fn render<W: ::std::fmt::Write>(&self, f: &mut W) -> ::std::fmt::Result {
        use ::std::fmt::Write;
        f.write_str("\n# default:\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
                &::std::string::ToString::to_string(&self.slice),
            ),
        )?;
        f.write_str("\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
                &::std::string::ToString::to_string(&self.string),
            ),
        )?;
        f.write_str("\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
                &::std::string::ToString::to_string(&self.integer),
            ),
        )?;
        f.write_str("\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
                &::std::string::ToString::to_string(&self.float),
            ),
        )?;
        f.write_str("\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
                &::std::string::ToString::to_string(&self.display),
            ),
        )?;
        f.write_str("\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::escapers::Escaper>::DEFAULT,
                &::std::string::ToString::to_string(&self.fn_string),
            ),
        )?;
        f.write_str("\n\n# text:\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &::oxiplate::escapers::html::HtmlEscaper::Text,
                &::std::string::ToString::to_string(&self.slice),
            ),
        )?;
        f.write_str("\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &::oxiplate::escapers::html::HtmlEscaper::Text,
                &::std::string::ToString::to_string(&self.string),
            ),
        )?;
        f.write_str("\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &::oxiplate::escapers::html::HtmlEscaper::Text,
                &::std::string::ToString::to_string(&self.integer),
            ),
        )?;
        f.write_str("\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &::oxiplate::escapers::html::HtmlEscaper::Text,
                &::std::string::ToString::to_string(&self.float),
            ),
        )?;
        f.write_str("\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &::oxiplate::escapers::html::HtmlEscaper::Text,
                &::std::string::ToString::to_string(&self.display),
            ),
        )?;
        f.write_str("\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &::oxiplate::escapers::html::HtmlEscaper::Text,
                &::std::string::ToString::to_string(&self.fn_string),
            ),
        )?;
        f.write_str("\n\n# comment:\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &::oxiplate::escapers::html::HtmlEscaper::Comment,
                &::std::string::ToString::to_string(&self.slice),
            ),
        )?;
        f.write_str("\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &::oxiplate::escapers::html::HtmlEscaper::Comment,
                &::std::string::ToString::to_string(&self.string),
            ),
        )?;
        f.write_str("\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &::oxiplate::escapers::html::HtmlEscaper::Comment,
                &::std::string::ToString::to_string(&self.integer),
            ),
        )?;
        f.write_str("\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &::oxiplate::escapers::html::HtmlEscaper::Comment,
                &::std::string::ToString::to_string(&self.float),
            ),
        )?;
        f.write_str("\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &::oxiplate::escapers::html::HtmlEscaper::Comment,
                &::std::string::ToString::to_string(&self.display),
            ),
        )?;
        f.write_str("\n")?;
        f.write_str(
            &::oxiplate::escapers::escape(
                &::oxiplate::escapers::html::HtmlEscaper::Comment,
                &::std::string::ToString::to_string(&self.fn_string),
            ),
        )?;
        f.write_str("\n\n# raw:\n")?;
        f.write_str(&::std::string::ToString::to_string(&self.slice))?;
        f.write_str("\n")?;
        f.write_str(&::std::string::ToString::to_string(&self.string))?;
        f.write_str("\n")?;
        f.write_str(&::std::string::ToString::to_string(&self.integer))?;
        f.write_str("\n")?;
        f.write_str(&::std::string::ToString::to_string(&self.float))?;
        f.write_str("\n")?;
        f.write_str(&::std::string::ToString::to_string(&self.display))?;
        f.write_str("\n")?;
        f.write_str(&::std::string::ToString::to_string(&self.fn_string))?;
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
        start_line: 63usize,
        start_col: 4usize,
        end_line: 63usize,
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
        fn_string: HelloWorld::hello(),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &r"
# default:
Hello world &amp;lt;&lt;script>&lt;!--
Hello world &amp;lt;&lt;script>&lt;!--
19
19.89
Hello world &amp;lt;&lt;script>&lt;!--
Hello world &amp;lt;&lt;script>&lt;!--

# text:
Hello world &amp;lt;&lt;script>&lt;!--
Hello world &amp;lt;&lt;script>&lt;!--
19
19.89
Hello world &amp;lt;&lt;script>&lt;!--
Hello world &amp;lt;&lt;script>&lt;!--

# comment:
Hello world &lt;‹script›‹ǃ−−
Hello world &lt;‹script›‹ǃ−−
19
19.89
Hello world &lt;‹script›‹ǃ−−
Hello world &lt;‹script›‹ǃ−−

# raw:
Hello world &lt;<script><!--
Hello world &lt;<script><!--
19
19.89
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
