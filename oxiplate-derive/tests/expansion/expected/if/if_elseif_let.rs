use alloc::format;

use oxiplate_derive::Oxiplate;
#[oxiplate_inline(
    r"
{%- if check -%}
bar
{%- elseif let Some(text) = ty -%}
{{ text }}
{%- endif -%}
"
)]
struct Data {
    check: bool,
    ty: Option<&'static str>,
}
impl ::core::fmt::Display for Data {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        if self.check {
            oxiplate_formatter.write_str("bar")?;
        } else if let Some(text) = self.ty {
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(text)))?;
        }
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "r#if::if_elseif_let::test"]
#[doc(hidden)]
pub static test: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("r#if::if_elseif_let::test"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/if/if_elseif_let.rs",
        start_line: 21usize,
        start_col: 4usize,
        end_line: 21usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test()),
    ),
};
fn test() {
    let data = Data {
        check: false,
        ty: Some("foo"),
    };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
            &"foo",
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
