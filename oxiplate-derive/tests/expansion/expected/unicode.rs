#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use oxiplate_derive::Oxiplate;
#[oxiplate = "unicode.html.oxip"]
struct Data {
    foo: &'static str,
}
impl ::core::fmt::Display for Data {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(5usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.foo)))?;
            oxiplate_formatter.write_str("\u{276f}\n")?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "external_unicode"]
#[doc(hidden)]
pub const external_unicode: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("external_unicode"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/unicode.rs",
        start_line: 16usize,
        start_col: 4usize,
        end_line: 16usize,
        end_col: 20usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(external_unicode()),
    ),
};
fn external_unicode() {
    let template = Data { foo: "bar" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", template))
        }),
        &"bar❯\n",
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
    test::test_main_static(&[&external_unicode])
}
