use alloc::format;

use oxiplate_derive::Oxiplate;
#[oxiplate_inline(
    "Braces ({ and }) are formatting characters in Rust and must be escaped if used in formatting \
     strings. {}"
)]
struct Data {}
impl ::core::fmt::Display for Data {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(
            "Braces ({ and }) are formatting characters in Rust and must be escaped if used in \
             formatting strings. {}",
        )?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "format_injection::format_injection"]
#[doc(hidden)]
pub static format_injection: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("format_injection::format_injection"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/format_injection.rs",
        start_line: 14usize,
        start_col: 4usize,
        end_line: 14usize,
        end_col: 20usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(format_injection()),
    ),
};
/// Ensure `{}` in a template doesn't break formatting.
fn format_injection() {
    let data = Data {};
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
            &"Braces ({ and }) are formatting characters in Rust and must be escaped if used in \
              formatting strings. {}",
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
