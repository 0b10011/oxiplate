#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::Oxiplate;
#[oxiplate_inline = "
{%- if do_this -%}
    This then {{ action }} :D
{%- endif %}"]
struct Data {
    do_this: bool,
    action: &'static str,
}
impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.do_this {
            f.write_fmt(format_args!("{0}", "This then"))?;
            f.write_fmt(format_args!("{0}", " "))?;
            f.write_fmt(
                format_args!(
                    "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self
                    .action)); res } .chars().map(| character | match character { '&' =>
                    { let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' =>
                    { let res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => {
                    let res = ::alloc::fmt::format(format_args!("{0}", character)); res }
                    }).collect::< String > ()
                ),
            )?;
            f.write_fmt(format_args!("{0}", " "))?;
            f.write_fmt(format_args!("{0}", ":D"))?;
        }
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_if"]
pub const test_if: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_if"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\if.rs",
        start_line: 14usize,
        start_col: 4usize,
        end_line: 14usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(test_if())),
};
fn test_if() {
    let data = Data {
        do_this: true,
        action: "do something",
    };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        },
        &"This then do something :D",
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
    test::test_main_static(&[&test_if])
}
