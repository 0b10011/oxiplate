use alloc::format;

use oxiplate_derive::Oxiplate;
#[oxiplate_inline(
    "
{%- if do_this -%}
    This then {{ action }} :D
{%- endif %}"
)]
struct Data {
    do_this: bool,
    action: &'static str,
}
impl ::core::fmt::Display for Data {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        if self.do_this {
            oxiplate_formatter.write_str("This then ")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.action)))?;
            oxiplate_formatter.write_str(" :D")?;
        }
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "r#if::r#if::test_if"]
#[doc(hidden)]
pub static test_if: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("r#if::r#if::test_if"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/if/if.rs",
        start_line: 18usize,
        start_col: 4usize,
        end_line: 18usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_if()),
    ),
};
fn test_if() {
    let data = Data {
        do_this: true,
        action: "do something",
    };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
            &"This then do something :D",
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
