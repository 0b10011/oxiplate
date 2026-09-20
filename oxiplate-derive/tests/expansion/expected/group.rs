#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline("{{ c * (a + b) }}")]
struct GroupCalc {
    a: usize,
    b: usize,
    c: usize,
}
impl ::core::fmt::Display for GroupCalc {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter
            .write_str(
                &alloc::string::ToString::to_string(&(self.c * (self.a + self.b))),
            )?;
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "group_calc"]
#[doc(hidden)]
pub static group_calc: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("group_calc"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/group.rs",
        start_line: 18usize,
        start_col: 4usize,
        end_line: 18usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(group_calc()),
    ),
};
fn group_calc() {
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", GroupCalc { a: 1, b: 2, c: 3 }))
            }),
            &"9",
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
    test::test_main_env_args(&[&group_calc])
}
