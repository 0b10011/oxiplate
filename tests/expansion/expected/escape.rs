#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::Oxiplate;
#[oxiplate_inline = "{% for message in &messages %}\n<p>{{ text: message }}</p>{% endfor %}\n"]
struct Data<'a> {
    messages: Vec<&'a str>,
}
impl<'a> std::fmt::Display for Data<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for message in &self.messages {
            f.write_fmt(format_args!("{0}", "\n"))?;
            f.write_fmt(format_args!("{0}", "<p>"))?;
            f.write_fmt(
                format_args!(
                    "{0}", { let res = ::alloc::fmt::format(format_args!("{0}",
                    message)); res } .chars().map(| character | match character { '&' =>
                    { let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' =>
                    { let res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => {
                    let res = ::alloc::fmt::format(format_args!("{0}", character)); res }
                    }).collect::< String > ()
                ),
            )?;
            f.write_fmt(format_args!("{0}", "</p>"))?;
        }
        f.write_fmt(format_args!("{0}", "\n"))?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "variable"]
pub const variable: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("variable"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\escape.rs",
        start_line: 10usize,
        start_col: 4usize,
        end_line: 10usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(variable())),
};
fn variable() {
    let data = Data {
        messages: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                "Hello world!",
                "&reg;</p><script>alert('hey');</script><p>&#153;",
            ]),
        ),
    };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        },
        &r#"
<p>Hello world!</p>
<p>&amp;reg;&lt;/p>&lt;script>alert('hey');&lt;/script>&lt;p>&amp;#153;</p>
"#,
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
    test::test_main_static(&[&variable])
}
