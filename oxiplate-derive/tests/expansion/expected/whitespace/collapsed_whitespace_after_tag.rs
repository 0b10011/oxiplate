use alloc::format;

use oxiplate_derive::Oxiplate;
#[oxiplate_inline(
    r#"{# _#}
{% if value %}foo{% endif _%}
{{ "bar" _}}
"#
)]
struct Data {
    value: bool,
}
impl ::core::fmt::Display for Data {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(" ")?;
        if self.value {
            oxiplate_formatter.write_str("foo")?;
        }
        oxiplate_formatter.write_str(" ")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("bar")))?;
        oxiplate_formatter.write_str(" ")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "whitespace::collapsed_whitespace_after_tag::adjusted_whitespace"]
#[doc(hidden)]
pub static adjusted_whitespace: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName(
            "whitespace::collapsed_whitespace_after_tag::adjusted_whitespace",
        ),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/whitespace/collapsed_whitespace_after_tag.\
                      rs",
        start_line: 17usize,
        start_col: 4usize,
        end_line: 17usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(adjusted_whitespace()),
    ),
};
fn adjusted_whitespace() {
    let data = Data { value: true };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
            &" foo bar ",
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
