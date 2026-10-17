use alloc::format;

use oxiplate_derive::Oxiplate;
mod core {}
#[oxiplate_inline("{{ foo }}")]
struct Data {
    foo: &'static str,
}
impl ::core::fmt::Display for Data {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.foo)))?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "overridden_core::overridden_std"]
#[doc(hidden)]
pub static overridden_std: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("overridden_core::overridden_std"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/overridden_core.rs",
        start_line: 14usize,
        start_col: 4usize,
        end_line: 14usize,
        end_col: 18usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(overridden_std()),
    ),
};
fn overridden_std() {
    let data = Data {
        foo: "Hello world!",
    };
    {
        match (
            &"Hello world!",
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
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
