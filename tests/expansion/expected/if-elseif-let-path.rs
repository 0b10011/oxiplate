#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::Oxiplate;
enum Type {
    Text(&'static str),
}
#[oxiplate_inline = r#"
{%- if check -%}
bar
{%- elseif let Type::Text(text) = ty -%}
{{ text }}
{%- endif -%}
"#]
struct Data {
    check: bool,
    ty: Type,
}
impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.check {
            f.write_fmt(format_args!("{0}", "bar"))?;
        } else if let Type::Text(text) = &self.ty {
            f.write_fmt(
                format_args!(
                    "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", text));
                    res } .chars().map(| character | match character { '&' => { let res =
                    ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                    ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                    ::alloc::fmt::format(format_args!("{0}", character)); res } })
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
pub const test: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\if-elseif-let-path.rs",
        start_line: 21usize,
        start_col: 4usize,
        end_line: 21usize,
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
        ty: Type::Text("foo"),
    };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        },
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
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test])
}
