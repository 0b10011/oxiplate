#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use oxiplate_derive::Oxiplate;
#[oxiplate = "./extends-nested-different-blocks.html.oxip"]
struct AbsoluteData {
    title: &'static str,
    message: &'static str,
}
impl ::std::fmt::Display for AbsoluteData {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(97usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter.write_str("<DOCTYPE html>\n<head>\n  <title>")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(self.title)))?;
            oxiplate_formatter.write_str("</title>\n</head>\n<body>")?;
            {
                oxiplate_formatter.write_str("<main>")?;
                {
                    oxiplate_formatter.write_str("<h1>")?;
                    oxiplate_formatter
                        .write_str(&::std::string::ToString::to_string(&(self.title)))?;
                    oxiplate_formatter.write_str("</h1>\n  <p>")?;
                    oxiplate_formatter
                        .write_str(
                            &::std::string::ToString::to_string(&(self.message)),
                        )?;
                    oxiplate_formatter.write_str("</p>")?;
                }
                oxiplate_formatter.write_str("</main>")?;
            }
            oxiplate_formatter.write_str("</body>\n")?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "absolute"]
#[doc(hidden)]
pub const absolute: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("absolute"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/extends-nested-different-blocks.rs",
        start_line: 11usize,
        start_col: 4usize,
        end_line: 11usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(absolute())),
};
fn absolute() {
    let data = AbsoluteData {
        title: "Oxiplate Example",
        message: "Hello world!",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"<DOCTYPE html>\n<head>\n  <title>Oxiplate \
         Example</title>\n</head>\n<body><main><h1>Oxiplate Example</h1>\n  <p>Hello \
         world!</p></main></body>\n",
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
    test::test_main_static(&[&absolute])
}
