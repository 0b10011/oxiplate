#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::Oxiplate;
struct User<'a> {
    name: &'a str,
}
#[oxiplate_inline = "{{ user.name }}"]
struct Data<'a> {
    user: &'a User<'a>,
}
impl<'a> std::fmt::Display for Data<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.user
                .name)); res } .chars().map(| character | match character { '&' => { let
                res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res
                = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "field"]
pub const field: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("field"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\borrow.rs",
        start_line: 14usize,
        start_col: 4usize,
        end_line: 14usize,
        end_col: 9usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(field())),
};
fn field() {
    let name = "Liv";
    let user = User { name: &name };
    let data = Data { user: &user };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        },
        &"Liv",
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
    test::test_main_static(&[&field])
}
