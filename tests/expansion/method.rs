#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::Oxiplate;
struct User {
    name: &'static str,
    company: &'static str,
}
impl User {
    pub fn display_name(&self) -> String {
        {
            let res = ::alloc::fmt::format(
                format_args!("{0} ({1})", self.company, self.name),
            );
            res
        }
    }
}
#[oxiplate_inline = "{{ user.display_name() }}"]
struct Data {
    user: User,
}
impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.user
                .display_name())); res } .chars().map(| character | match character { '&'
                => { let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' =>
                { let res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let
                res = ::alloc::fmt::format(format_args!("{0}", character)); res } })
                .collect::< String > ()
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
        source_file: "tests\\method.rs",
        start_line: 20usize,
        start_col: 4usize,
        end_line: 20usize,
        end_col: 9usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(field())),
};
fn field() {
    let data = Data {
        user: User {
            name: "Kiera",
            company: "Floating Air LLC",
        },
    };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        },
        &"Floating Air LLC (Kiera)",
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
    test::test_main_static(&[&field])
}
