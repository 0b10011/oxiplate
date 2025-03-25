#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline = "Braces ({ and }) are formatting characters in Rust and must be escaped. {}"]
struct Data {}
impl ::std::fmt::Display for Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_fmt(
            format_args!(
                "Braces ({0} and {1}) are formatting characters in Rust and must be escaped. {2}{3}",
                "{",
                "}",
                "{",
                "}",
            ),
        )?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "format_injection"]
#[doc(hidden)]
pub const format_injection: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("format_injection"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/format-injection.rs",
        start_line: 9usize,
        start_col: 4usize,
        end_line: 9usize,
        end_col: 20usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(format_injection()),
    ),
};
/// Ensure `{}` in a template doesn't break formatting.
fn format_injection() {
    let template = Data {};
    match (
        &::alloc::__export::must_use({
            let res = ::alloc::fmt::format(format_args!("{0}", template));
            res
        }),
        &"Braces ({ and }) are formatting characters in Rust and must be escaped. {}",
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
    test::test_main_static(&[&format_injection])
}
