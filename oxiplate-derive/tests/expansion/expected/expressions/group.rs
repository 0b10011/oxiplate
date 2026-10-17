use alloc::format;

use oxiplate_derive::Oxiplate;
#[oxiplate_inline("{{ c * (a + b) }}")]
struct GroupCalc {
    a: usize,
    b: usize,
    c: usize,
}
impl ::core::fmt::Display for GroupCalc {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
            &(self.c * (self.a + self.b)),
        ))?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "expressions::group::group_calc"]
#[doc(hidden)]
pub static group_calc: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::group::group_calc"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/group.rs",
        start_line: 14usize,
        start_col: 4usize,
        end_line: 14usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(group_calc()),
    ),
};
fn group_calc() {
    {
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
        }
    };
}
