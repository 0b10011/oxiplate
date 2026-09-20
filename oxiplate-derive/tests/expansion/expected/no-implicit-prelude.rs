#![feature(prelude_import)]
#![no_implicit_prelude]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use ::core::assert_eq;
use ::oxiplate_derive::Oxiplate;
#[oxiplate = "extends.html.oxip"]
struct Data {
    title: &'static str,
    message: &'static str,
}
impl ::core::fmt::Display for Data {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str("<!DOCTYPE html>\n<title>")?;
        oxiplate_formatter
            .write_str(&alloc::string::ToString::to_string(&(self.title)))?;
        oxiplate_formatter.write_str("</title>\n")?;
        {
            oxiplate_formatter.write_str("<h1>")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.title)))?;
            oxiplate_formatter.write_str("</h1>\n  <p>")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.message)))?;
            oxiplate_formatter.write_str("</p>")?;
        }
        oxiplate_formatter.write_str("\n")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "no_implicit_prelude"]
#[doc(hidden)]
pub static no_implicit_prelude: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("no_implicit_prelude"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/no-implicit-prelude.rs",
        start_line: 19usize,
        start_col: 4usize,
        end_line: 19usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(no_implicit_prelude()),
    ),
};
fn no_implicit_prelude() {
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!(
                        "{0}",
                        Data {
                            title: "Oxiplate Example",
                            message: "Hello world!",
                        },
                    ),
                )
            }),
            &"<!DOCTYPE html>\n<title>Oxiplate Example</title>\n<h1>Oxiplate Example</h1>\n  <p>Hello \
         world!</p>\n",
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
        }
    };
}
extern crate test;
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> test::ExitCode {
    test::test_main_env_args(&[&no_implicit_prelude])
}
