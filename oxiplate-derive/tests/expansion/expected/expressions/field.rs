use alloc::format;

use oxiplate_derive::Oxiplate;
struct User {
    name: &'static str,
}
#[oxiplate_inline("{{ user.name }}")]
struct Data {
    user: User,
}
impl ::core::fmt::Display for Data {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.user.name)))?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "expressions::field::field"]
#[doc(hidden)]
pub static field: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::field::field"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/field.rs",
        start_line: 16usize,
        start_col: 4usize,
        end_line: 16usize,
        end_col: 9usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(field()),
    ),
};
fn field() {
    let data = Data {
        user: User { name: "Liv" },
    };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
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
        }
    };
}
