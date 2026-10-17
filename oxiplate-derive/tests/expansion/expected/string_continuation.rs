use alloc::format;

use oxiplate_derive::Oxiplate;
#[oxiplate_inline(
    "\
    hello world"
)]
struct Data {}
impl ::core::fmt::Display for Data {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str("hello world")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "string_continuation::external_unicode"]
#[doc(hidden)]
pub static external_unicode: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("string_continuation::external_unicode"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/string_continuation.rs",
        start_line: 13usize,
        start_col: 4usize,
        end_line: 13usize,
        end_col: 20usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(external_unicode()),
    ),
};
fn external_unicode() {
    let template = Data {};
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", template)) }),
            &"hello world",
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
