#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::Oxiplate;
#[oxiplate_inline = r#"{% if value %}foo{% endif _%}
"#]
struct Data {
    value: bool,
}
impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value {
            f.write_fmt(format_args!("{0}", "foo"))?;
        }
        f.write_fmt(format_args!("{0}", " "))?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "adjusted_whitespace"]
pub const adjusted_whitespace: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("adjusted_whitespace"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\collapsed-whitespace-after-tag.rs",
        start_line: 11usize,
        start_col: 4usize,
        end_line: 11usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(adjusted_whitespace()),
    ),
};
fn adjusted_whitespace() {
    let template = Data { value: true };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", template));
            res
        },
        &"foo ",
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
    test::test_main_static(&[&adjusted_whitespace])
}
