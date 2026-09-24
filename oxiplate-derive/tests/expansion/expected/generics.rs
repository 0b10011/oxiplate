#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use alloc::string::String;
use oxiplate_derive::Oxiplate;
extern crate test;
#[rustc_test_marker = "lifetime"]
#[doc(hidden)]
pub static lifetime: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("lifetime"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/generics.rs",
        start_line: 11usize,
        start_col: 4usize,
        end_line: 11usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(lifetime())),
};
fn lifetime<'b>() {
    fn function<'a: 'a>(data: &'a str) -> String {
        ::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0:?}", data))
        })
    }
    #[oxiplate_inline(r#"{{ function::<'b>(message) }}"#)]
    struct Data<'b> {
        message: &'b str,
    }
    impl<'b> ::core::fmt::Display for Data<'b> {
        fn fmt(
            &self,
            oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter
                .write_str(
                    &alloc::string::ToString::to_string(&(function::<'b>(self.message))),
                )?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}", Data { message: "hello world" }),
                )
            }),
            &r#""hello world""#,
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
    test::test_main_env_args(&[&lifetime])
}
