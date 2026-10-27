use alloc::format;

use oxiplate::prelude::*;
struct User {
    name: &'static str,
}
extern crate test;
#[rustc_test_marker = "precedence::precedence"]
#[doc(hidden)]
pub static precedence: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("precedence::precedence"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/integration/precedence.rs",
        start_line: 10usize,
        start_col: 4usize,
        end_line: 10usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(precedence()),
    ),
};
fn precedence() {
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
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
                &(filters_for_oxiplate::trim(::oxiplate::CowStrWrapper::new(
                    (&&::oxiplate::ToCowStrWrapper::new(
                        &(::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!(" ({0}) ", (9 + 5 * 2) * 100 + 89))
                        })),
                    ))
                        .to_cow_str(),
                ))),
            ))
                .oxiplate_raw(oxiplate_formatter)?;
            oxiplate_formatter.write_str("\n")?;
            (&&::oxiplate::UnescapedTextWrapper::new(
                &(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!(
                        "{0} is awesome",
                        filters_for_oxiplate::upper(::oxiplate::CowStrWrapper::new(
                            (&&::oxiplate::ToCowStrWrapper::new(&(self.user.name))).to_cow_str(),
                        ),),
                    ))
                })),
            ))
                .oxiplate_raw(oxiplate_formatter)?;
            oxiplate_formatter.write_str("\n")?;
            ::core::result::Result::Ok(())
        }
    }
    let data = Data {
        user: User { name: "Liv" },
    };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
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
extern crate test;
#[rustc_test_marker = "precedence::cow_concat_field"]
#[doc(hidden)]
pub static cow_concat_field: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("precedence::cow_concat_field"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/integration/precedence.rs",
        start_line: 36usize,
        start_col: 4usize,
        end_line: 36usize,
        end_col: 20usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(cow_concat_field()),
    ),
};
fn cow_concat_field() {
    #[oxiplate_inline(r#"{{ raw: >"hello " ~ user.name }}"#)]
    struct Data {
        user: User,
    }
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::oxiplate::Render::render_into(self, oxiplate_formatter)
        }
    }
    impl ::oxiplate::Render for Data {
        const ESTIMATED_LENGTH: usize = 7usize;
        #[inline]
        fn render_into<W: ::core::fmt::Write>(
            &self,
            oxiplate_formatter: &mut W,
        ) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            use ::oxiplate::{ToCowStr as _, UnescapedText as _};
            (&&::oxiplate::UnescapedTextWrapper::new(
                &(::oxiplate::CowStrWrapper::new(
                    (&&::oxiplate::ToCowStrWrapper::new(
                        &(::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("hello {0}", self.user.name))
                        })),
                    ))
                        .to_cow_str(),
                )),
            ))
                .oxiplate_raw(oxiplate_formatter)?;
            ::core::result::Result::Ok(())
        }
    }
    let data = Data {
        user: User { name: "Liv" },
    };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
            &"hello Liv",
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
extern crate test;
#[rustc_test_marker = "precedence::cow_concat_function"]
#[doc(hidden)]
pub static cow_concat_function: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("precedence::cow_concat_function"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/integration/precedence.rs",
        start_line: 51usize,
        start_col: 4usize,
        end_line: 51usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(cow_concat_function()),
    ),
};
fn cow_concat_function() {
    fn name(user: &User) -> &'static str {
        user.name
    }
    #[oxiplate_inline(r#"{{ raw: >"hello " ~ name(&user) }}"#)]
    struct Data {
        user: User,
    }
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::oxiplate::Render::render_into(self, oxiplate_formatter)
        }
    }
    impl ::oxiplate::Render for Data {
        const ESTIMATED_LENGTH: usize = 7usize;
        #[inline]
        fn render_into<W: ::core::fmt::Write>(
            &self,
            oxiplate_formatter: &mut W,
        ) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            use ::oxiplate::{ToCowStr as _, UnescapedText as _};
            (&&::oxiplate::UnescapedTextWrapper::new(
                &(::oxiplate::CowStrWrapper::new(
                    (&&::oxiplate::ToCowStrWrapper::new(
                        &(::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("hello {0}", name(&self.user)))
                        })),
                    ))
                        .to_cow_str(),
                )),
            ))
                .oxiplate_raw(oxiplate_formatter)?;
            ::core::result::Result::Ok(())
        }
    }
    let data = Data {
        user: User { name: "Liv" },
    };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
            &"hello Liv",
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
