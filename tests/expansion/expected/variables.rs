#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::Oxiplate;
#[oxiplate_inline = "{{ title }} / {{ message }}"]
struct Data {
    title: &'static str,
    message: &'static str,
}
impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.title));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "/"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self
                .message)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "variables"]
pub const variables: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("variables"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\variables.rs",
        start_line: 11usize,
        start_col: 4usize,
        end_line: 11usize,
        end_col: 13usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(variables())),
};
fn variables() {
    let data = Data {
        title: "Foo Bar",
        message: "Hello world!",
    };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        },
        &"Foo Bar / Hello world!",
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
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&variables])
}
