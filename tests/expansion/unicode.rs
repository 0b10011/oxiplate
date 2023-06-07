#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::Oxiplate;
#[oxiplate = "unicode.html.oxip"]
struct Data {
    foo: &'static str,
}
impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.foo));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "❯"))?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "external_unicode"]
pub const external_unicode: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("external_unicode"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\unicode.rs",
        start_line: 10usize,
        start_col: 4usize,
        end_line: 10usize,
        end_col: 20usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(external_unicode())),
};
fn external_unicode() {
    let template = Data { foo: "bar" };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", template));
            res
        },
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
#[no_coverage]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&external_unicode])
}
