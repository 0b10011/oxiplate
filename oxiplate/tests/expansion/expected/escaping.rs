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
        String::from("Hello world")
    }
}
impl Display for HelloWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Hello world")
    }
}
#[oxiplate_inline = "
{{ slice }}
{{ string }}
{{ integer }}
{{ float }}
{{ display }}
{{ fn_string }}

{{ text: slice }}
{{ text: string }}
{{ text: integer }}
{{ text: float }}
{{ text: display }}
{{ text: fn_string }}

{{ raw: slice }}
{{ raw: string }}
{{ raw: integer }}
{{ raw: float }}
{{ raw: display }}
{{ raw: fn_string }}
"]
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
        f.write_fmt(
            format_args!(
                "\n{0}\n{1}\n{2}\n{3}\n{4}\n{5}\n\n{6}\n{7}\n{8}\n{9}\n{10}\n{11}\n\n{12}\n{13}\n{14}\n{15}\n{16}\n{17}\n",
                self.slice,
                self.string,
                self.integer,
                self.float,
                self.display,
                self.fn_string,
                ::oxiplate::escapers::escape(
                    &::oxiplate::escapers::html::HtmlEscaper::Text,
                    &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}", self.slice))
                    }),
                ),
                ::oxiplate::escapers::escape(
                    &::oxiplate::escapers::html::HtmlEscaper::Text,
                    &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}", self.string))
                    }),
                ),
                ::oxiplate::escapers::escape(
                    &::oxiplate::escapers::html::HtmlEscaper::Text,
                    &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}", self.integer))
                    }),
                ),
                ::oxiplate::escapers::escape(
                    &::oxiplate::escapers::html::HtmlEscaper::Text,
                    &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}", self.float))
                    }),
                ),
                ::oxiplate::escapers::escape(
                    &::oxiplate::escapers::html::HtmlEscaper::Text,
                    &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}", self.display))
                    }),
                ),
                ::oxiplate::escapers::escape(
                    &::oxiplate::escapers::html::HtmlEscaper::Text,
                    &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}", self.fn_string))
                    }),
                ),
                self.slice,
                self.string,
                self.integer,
                self.float,
                self.display,
                self.fn_string,
            ),
        )?;
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
        start_line: 52usize,
        start_col: 4usize,
        end_line: 52usize,
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
        slice: "Hello world",
        string: String::from("Hello world"),
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
Hello world
Hello world
19
19.89
Hello world
Hello world

Hello world
Hello world
19
19.89
Hello world
Hello world

Hello world
Hello world
19
19.89
Hello world
Hello world
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
