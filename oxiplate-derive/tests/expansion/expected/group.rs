#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline("{{ c * (a + b) }}")]
struct GroupCalc {
    a: usize,
    b: usize,
    c: usize,
}
impl ::std::fmt::Display for GroupCalc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(1usize);
            let f = &mut string;
            f.write_str(
                &::std::string::ToString::to_string(&(self.c * (self.a + self.b))),
            )?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "group_calc"]
#[doc(hidden)]
pub const group_calc: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("group_calc"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/group.rs",
        start_line: 12usize,
        start_col: 4usize,
        end_line: 12usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(group_calc()),
    ),
};
fn group_calc() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", GroupCalc { a: 1, b: 2, c: 3 }))
        }),
        &"9",
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
    test::test_main_static(&[&group_calc])
}
