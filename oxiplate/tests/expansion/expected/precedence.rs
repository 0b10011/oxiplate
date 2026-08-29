#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use oxiplate::prelude::*;
struct User {
    name: &'static str,
}
#[oxiplate_inline(
    r#"
{{ raw: >" (" ~ (9 + 5 * 2) * 100 + 89 ~ ") " | trim }}
{{ raw: >user.name | upper ~ " is awesome" }}
"#
)]
struct Data {
    user: User,
}
impl ::core::fmt::Display for Data {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::oxiplate::Render::render_into(self, oxiplate_formatter)
    }
}
impl ::oxiplate::Render for Data {
    const ESTIMATED_LENGTH: usize = 20usize;
    #[inline]
    fn render_into<W: ::core::fmt::Write>(
        &self,
        oxiplate_formatter: &mut W,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        use ::oxiplate::{ToCowStr as _, UnescapedText as _};
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::UnescapedTextWrapper::new(
            &(filters_for_oxiplate::trim(
                ::oxiplate::CowStrWrapper::new(
                    (&&::oxiplate::ToCowStrWrapper::new(
                        &(::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "{0}) ",
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(" ({0}", (9 + 5 * 2) * 100 + 89),
                                        )
                                    }),
                                ),
                            )
                        })),
                    ))
                        .to_cow_str(),
                ),
            )),
        ))
            .oxiplate_raw(oxiplate_formatter)?;
        oxiplate_formatter.write_str("\n")?;
        (&&::oxiplate::UnescapedTextWrapper::new(
            &(::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!(
                        "{0} is awesome",
                        filters_for_oxiplate::upper(
                            ::oxiplate::CowStrWrapper::new(
                                (&&::oxiplate::ToCowStrWrapper::new(&(self.user.name)))
                                    .to_cow_str(),
                            ),
                        ),
                    ),
                )
            })),
        ))
            .oxiplate_raw(oxiplate_formatter)?;
        oxiplate_formatter.write_str("\n")?;
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "field"]
#[doc(hidden)]
pub const field: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("field"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/precedence.rs",
        start_line: 25usize,
        start_col: 4usize,
        end_line: 25usize,
        end_col: 9usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(field())),
};
#[rustc_test_entrypoint_marker]
fn field() {
    let data = Data { user: User { name: "Liv" } };
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", data))
            }),
            &"
(1989)
LIV is awesome
",
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
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&field])
}
