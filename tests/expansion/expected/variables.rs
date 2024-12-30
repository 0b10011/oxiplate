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
        f.write_fmt(format_args!("{0}", self.title))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "/"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", self.message))?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "variables"]
#[doc(hidden)]
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
        &::alloc::__export::must_use({
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        }),
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
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&variables])
}
