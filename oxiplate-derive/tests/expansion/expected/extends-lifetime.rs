#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate_derive::Oxiplate;
#[oxiplate = "extends-deep.html.oxip"]
struct AbsoluteData<'a> {
    title: &'a str,
    message: &'a str,
}
impl<'a> ::std::fmt::Display for AbsoluteData<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(59usize);
            let f = &mut string;
            f.write_str("<!DOCTYPE html>\n<title>")?;
            f.write_str(&::std::string::ToString::to_string(&self.title))?;
            f.write_str("</title>\n")?;
            f.write_str("<h2>")?;
            f.write_str(&::std::string::ToString::to_string(&self.title))?;
            f.write_str("</h2>\n  <div>")?;
            f.write_str(&::std::string::ToString::to_string(&self.message))?;
            f.write_str("</div>")?;
            f.write_str("\n")?;
            string
        };
        f.write_str(&string)
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
        source_file: "oxiplate-derive/tests/extends-lifetime.rs",
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
        &"<!DOCTYPE html>\n<title>Oxiplate Example</title>\n<h2>Oxiplate Example</h2>\n  \
         <div>Hello world!</div>\n",
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
