use alloc::format;

use oxiplate::prelude::*;
extern crate test;
#[rustc_test_marker = "filter_block::basic_lower"]
#[doc(hidden)]
pub static basic_lower: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("filter_block::basic_lower"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/integration/filter_block.rs",
        start_line: 6usize,
        start_col: 4usize,
        end_line: 6usize,
        end_col: 15usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(basic_lower()),
    ),
};
fn basic_lower() {
    #[oxiplate_inline(
        r#"
{%- filter raw | lower -%}
    Hello World!
{%- endfilter -%}
"#
    )]
    struct Data;
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::oxiplate::Render::render_into(self, oxiplate_formatter)
        }
    }
    impl ::oxiplate::Render for Data {
        const ESTIMATED_LENGTH: usize = 12usize;
        #[inline]
        fn render_into<W: ::core::fmt::Write>(
            &self,
            oxiplate_formatter: &mut W,
        ) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            use ::oxiplate::private::{ToCowStr as _, WriteEscaped as _, WriteRaw as _};
            use ::oxiplate::{Escaper as _, TextHandler as _};
            (&&::oxiplate::private::UnescapedTextWrapper::new(
                ::oxiplate::PreparedText::AlreadyEscaped(
                    &({
                        let oxiplate_template_string = {
                            let mut oxiplate_formatter_owned =
                                alloc::string::String::with_capacity(12usize);
                            let oxiplate_formatter = &mut oxiplate_formatter_owned;
                            oxiplate_formatter.write_str("Hello World!")?;
                            oxiplate_formatter_owned
                        };
                        filters_for_oxiplate::lower(::oxiplate::private::CowStrWrapper::new(
                            (&&::oxiplate::private::ToCowStrWrapper::new(
                                &(oxiplate_template_string),
                            ))
                                .to_cow_str(),
                        ))
                    }),
                ),
            ))
                .oxiplate_write_raw(oxiplate_formatter)?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Data)) }),
            &"hello world!",
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
#[rustc_test_marker = "filter_block::basic_lower_upper"]
#[doc(hidden)]
pub static basic_lower_upper: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("filter_block::basic_lower_upper"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/integration/filter_block.rs",
        start_line: 21usize,
        start_col: 4usize,
        end_line: 21usize,
        end_col: 21usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(basic_lower_upper()),
    ),
};
fn basic_lower_upper() {
    #[oxiplate_inline(
        r#"
{%- filter raw | >lower | upper -%}
    Hello World!
{%- endfilter -%}
"#
    )]
    struct Data;
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::oxiplate::Render::render_into(self, oxiplate_formatter)
        }
    }
    impl ::oxiplate::Render for Data {
        const ESTIMATED_LENGTH: usize = 12usize;
        #[inline]
        fn render_into<W: ::core::fmt::Write>(
            &self,
            oxiplate_formatter: &mut W,
        ) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            use ::oxiplate::private::{ToCowStr as _, WriteEscaped as _, WriteRaw as _};
            use ::oxiplate::{Escaper as _, TextHandler as _};
            (&&::oxiplate::private::UnescapedTextWrapper::new(
                ::oxiplate::PreparedText::AlreadyEscaped(
                    &({
                        let oxiplate_template_string = {
                            let mut oxiplate_formatter_owned =
                                alloc::string::String::with_capacity(12usize);
                            let oxiplate_formatter = &mut oxiplate_formatter_owned;
                            oxiplate_formatter.write_str("Hello World!")?;
                            oxiplate_formatter_owned
                        };
                        filters_for_oxiplate::upper(::oxiplate::private::CowStrWrapper::new(
                            (&&::oxiplate::private::ToCowStrWrapper::new(
                                &(filters_for_oxiplate::lower(
                                    ::oxiplate::private::CowStrWrapper::new(
                                        (&&::oxiplate::private::ToCowStrWrapper::new(
                                            &(oxiplate_template_string),
                                        ))
                                            .to_cow_str(),
                                    ),
                                )),
                            ))
                                .to_cow_str(),
                        ))
                    }),
                ),
            ))
                .oxiplate_write_raw(oxiplate_formatter)?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Data)) }),
            &"HELLO WORLD!",
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
#[rustc_test_marker = "filter_block::writs_with_html"]
#[doc(hidden)]
pub static writs_with_html: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("filter_block::writs_with_html"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/integration/filter_block.rs",
        start_line: 42usize,
        start_col: 4usize,
        end_line: 42usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(writs_with_html()),
    ),
};
/// Test to ensure content is output as-is when `raw` is set.
/// Uppercase HTML tags isn't really supposed to be a use-case,
/// but it does essentially test what would happen
/// if that HTML was generated by one of the filters.
/// Easier than writing a filter
/// that generates HTML and escapes the content passed to it.
fn writs_with_html() {
    #[oxiplate_inline(
        r#"
{%- filter raw | upper -%}
    {{ message }}
{%- endfilter -%}
"#
    )]
    struct Data {
        message: &'static str,
    }
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::oxiplate::Render::render_into(self, oxiplate_formatter)
        }
    }
    impl ::oxiplate::Render for Data {
        const ESTIMATED_LENGTH: usize = 1usize;
        #[inline]
        fn render_into<W: ::core::fmt::Write>(
            &self,
            oxiplate_formatter: &mut W,
        ) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            use ::oxiplate::private::{ToCowStr as _, WriteEscaped as _, WriteRaw as _};
            use ::oxiplate::{Escaper as _, TextHandler as _};
            (&&::oxiplate::private::UnescapedTextWrapper::new(
                ::oxiplate::PreparedText::AlreadyEscaped(
                    &({
                        let oxiplate_template_string = {
                            let mut oxiplate_formatter_owned =
                                alloc::string::String::with_capacity(1usize);
                            let oxiplate_formatter = &mut oxiplate_formatter_owned;
                            (&&::oxiplate::private::UnescapedTextWrapper::new(
                                ::oxiplate::PreparedText::AlreadyEscaped(&(self.message)),
                            ))
                                .oxiplate_write_raw(oxiplate_formatter)?;
                            oxiplate_formatter_owned
                        };
                        filters_for_oxiplate::upper(::oxiplate::private::CowStrWrapper::new(
                            (&&::oxiplate::private::ToCowStrWrapper::new(
                                &(oxiplate_template_string),
                            ))
                                .to_cow_str(),
                        ))
                    }),
                ),
            ))
                .oxiplate_write_raw(oxiplate_formatter)?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!(
                    "{0}",
                    Data {
                        message: "<strong>Hello</strong> World!",
                    },
                ))
            }),
            &"<STRONG>HELLO</STRONG> WORLD!",
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
#[rustc_test_marker = "filter_block::writs_with_unsafe_chars"]
#[doc(hidden)]
pub static writs_with_unsafe_chars: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("filter_block::writs_with_unsafe_chars"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/integration/filter_block.rs",
        start_line: 67usize,
        start_col: 4usize,
        end_line: 67usize,
        end_col: 27usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(writs_with_unsafe_chars()),
    ),
};
fn writs_with_unsafe_chars() {
    #[oxiplate_inline(
        r#"
{%- filter html.text | upper -%}
    {{ message }}
{%- endfilter -%}
"#
    )]
    struct Data {
        message: &'static str,
    }
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::oxiplate::Render::render_into(self, oxiplate_formatter)
        }
    }
    impl ::oxiplate::Render for Data {
        const ESTIMATED_LENGTH: usize = 1usize;
        #[inline]
        fn render_into<W: ::core::fmt::Write>(
            &self,
            oxiplate_formatter: &mut W,
        ) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            use ::oxiplate::private::{ToCowStr as _, WriteEscaped as _, WriteRaw as _};
            use ::oxiplate::{Escaper as _, TextHandler as _};
            (&&::oxiplate::private::UnescapedTextWrapper::new(
                (&&::oxiplate::escapers::html::HtmlEscaper::build_text_handler(
                    &({
                        let oxiplate_template_string = {
                            let mut oxiplate_formatter_owned =
                                alloc::string::String::with_capacity(1usize);
                            let oxiplate_formatter = &mut oxiplate_formatter_owned;
                            (&&::oxiplate::private::UnescapedTextWrapper::new(
                                ::oxiplate::PreparedText::AlreadyEscaped(&(self.message)),
                            ))
                                .oxiplate_write_raw(oxiplate_formatter)?;
                            oxiplate_formatter_owned
                        };
                        filters_for_oxiplate::upper(::oxiplate::private::CowStrWrapper::new(
                            (&&::oxiplate::private::ToCowStrWrapper::new(
                                &(oxiplate_template_string),
                            ))
                                .to_cow_str(),
                        ))
                    }),
                ))
                    .prepare_text_for_output(&::oxiplate::escapers::html::HtmlEscaper::text),
            ))
                .oxiplate_write_escaped(
                    oxiplate_formatter,
                    &::oxiplate::escapers::html::HtmlEscaper::text,
                )?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!(
                    "{0}",
                    Data {
                        message: "<Hello> World!"
                    }
                ))
            }),
            &"&lt;HELLO> WORLD!",
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
