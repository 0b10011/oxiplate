#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use oxiplate_derive::Oxiplate;
mod filters_for_oxiplate {
    extern crate alloc;
    use alloc::string::ToString as _;
    use core::fmt::Display;
    pub fn respond(expression: impl Display, yell: bool) -> impl Display {
        let expression = expression.to_string();
        match expression.as_str() {
            "hello" => if yell { "WORLD" } else { "world" }.to_string(),
            _ => "did not understand: ".to_string() + &expression,
        }
    }
}
#[oxiplate_inline(
    r#"{{ message | respond(*respond) ~ " " ~ 9 + 5 * 2 ~ " " ~ (message | respond(!respond)) }}"#
)]
struct Respond {
    message: &'static str,
    respond: &'static bool,
}
impl ::core::fmt::Display for Respond {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write as _;
            let mut string = alloc::string::String::with_capacity(5usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter
                .write_str(
                    &alloc::string::ToString::to_string(
                        &(::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "{0} {1}",
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "{0} {1}",
                                                filters_for_oxiplate::respond(self.message, *self.respond),
                                                9 + 5 * 2,
                                            ),
                                        )
                                    }),
                                    (filters_for_oxiplate::respond(self.message, !self.respond)),
                                ),
                            )
                        })),
                    ),
                )?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "combination"]
#[doc(hidden)]
pub const combination: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("combination"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/expressions.rs",
        start_line: 34usize,
        start_col: 4usize,
        end_line: 34usize,
        end_col: 15usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(combination()),
    ),
};
fn combination() {
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!(
                        "{0}",
                        Respond {
                            message: "hello",
                            respond: &false,
                        },
                    ),
                )
            }),
            &"world 19 WORLD",
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
    test::test_main_static(&[&combination])
}
