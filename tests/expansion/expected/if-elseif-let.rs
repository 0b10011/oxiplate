#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::Oxiplate;
#[oxiplate_inline = r#"
{%- if check -%}
bar
{%- elseif let Some(text) = ty -%}
{{ text }}
{%- endif -%}
"#]
struct Data {
    check: bool,
    ty: Option<&'static str>,
}
impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.check {
            f.write_fmt(format_args!("{0}", "bar"))?;
        } else if let Some(text) = &self.ty {
            f.write_fmt(
                format_args!(
                    "{0}", ::alloc::__export::must_use({ let res =
                    ::alloc::fmt::format(format_args!("{0}", text)); res }).chars().map(|
                    character | match character { '&' => ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(format_args!("&amp;")); res }), '<' =>
                    ::alloc::__export::must_use({ let res =
                    ::alloc::fmt::format(format_args!("&lt;")); res }), _ =>
                    ::alloc::__export::must_use({ let res =
                    ::alloc::fmt::format(format_args!("{0}", character)); res }), })
                    .collect::< String > ()
                ),
            )?;
        }
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test"]
#[doc(hidden)]
pub const test: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\if-elseif-let.rs",
        start_line: 17usize,
        start_col: 4usize,
        end_line: 17usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test())),
};
fn test() {
    let data = Data {
        check: false,
        ty: Some("foo"),
    };
    match (
        &::alloc::__export::must_use({
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        }),
        &"foo",
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
    test::test_main_static(&[&test])
}
