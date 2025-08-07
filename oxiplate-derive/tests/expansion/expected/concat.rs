#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline(r#"{{ name ~ " (" ~ company ~ ")" }}"#)]
struct User {
    name: &'static str,
    company: &'static str,
}
impl ::std::fmt::Display for User {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(1usize);
            let f = &mut string;
            f.write_str(
                &::std::string::ToString::to_string(
                    &::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} ({1})", self.name, self.company),
                        )
                    }),
                ),
            )?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "variable"]
#[doc(hidden)]
pub const variable: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("variable"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/concat.rs",
        start_line: 11usize,
        start_col: 4usize,
        end_line: 11usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(variable())),
};
fn variable() {
    let data = User {
        name: "Xavier",
        company: "XYZ Company",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"Xavier (XYZ Company)",
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
    test::test_main_static(&[&variable])
}
