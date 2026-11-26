use oxiplate::{Oxiplate, Render};
extern crate test;
#[rustc_test_marker = "escapers::html::override_group::default"]
#[doc(hidden)]
pub static default: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("escapers::html::override_group::default"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/integration/escapers/html/override_group.rs",
        start_line: 4usize,
        start_col: 4usize,
        end_line: 4usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(default()),
    ),
};
fn default() {
    use alloc::format;
    #[oxiplate_inline(
        html:r#"{% default_escaper_group html %}{{ "<hello>" }}"#
    )]
    struct Data;
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
            use ::oxiplate::private::{ToCowStr as _, WriteEscaped as _, WriteRaw as _};
            use ::oxiplate::{Escaper as _, TextHandler as _};
            (&&::oxiplate::private::UnescapedTextWrapper::new(
                (&&::oxiplate::escapers::html::HtmlEscaper::build_text_handler(&("<hello>")))
                    .prepare_text_for_output(
                        &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::Escaper>::DEFAULT,
                    ),
            ))
                .oxiplate_write_escaped(
                    oxiplate_formatter,
                    &<::oxiplate::escapers::html::HtmlEscaper as ::oxiplate::Escaper>::DEFAULT,
                )?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Data)) }),
            &"&lt;hello>",
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
#[rustc_test_marker = "escapers::html::override_group::replace"]
#[doc(hidden)]
pub static replace: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("escapers::html::override_group::replace"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate/tests/integration/escapers/html/override_group.rs",
        start_line: 15usize,
        start_col: 4usize,
        end_line: 15usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(replace()),
    ),
};
fn replace() {
    #[oxiplate = "replace-escaper.html.oxip"]
    struct Html {
        name: &'static str,
    }
    impl ::core::fmt::Display for Html {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::oxiplate::Render::render_into(self, oxiplate_formatter)
        }
    }
    impl ::oxiplate::Render for Html {
        const ESTIMATED_LENGTH: usize = 7usize;
        #[inline]
        fn render_into<W: ::core::fmt::Write>(
            &self,
            oxiplate_formatter: &mut W,
        ) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            use ::oxiplate::private::{ToCowStr as _, WriteEscaped as _, WriteRaw as _};
            use ::oxiplate::{Escaper as _, TextHandler as _};
            oxiplate_formatter.write_str("\n")?;
            (&&::oxiplate::private::UnescapedTextWrapper::new(
                (&&::oxiplate::escapers::your_group::YourEscaper::build_text_handler(
                    &(self.name),
                ))
                    .prepare_text_for_output(
                        &<::oxiplate::escapers::your_group::YourEscaper as ::oxiplate::Escaper>::DEFAULT,
                    ),
            ))
                .oxiplate_write_escaped(
                    oxiplate_formatter,
                    &<::oxiplate::escapers::your_group::YourEscaper as ::oxiplate::Escaper>::DEFAULT,
                )?;
            oxiplate_formatter.write_str("\n")?;
            (&&::oxiplate::private::UnescapedTextWrapper::new(
                (&&::oxiplate::escapers::your_group::YourEscaper::build_text_handler(&(self.name)))
                    .prepare_text_for_output(&::oxiplate::escapers::your_group::YourEscaper::foo),
            ))
                .oxiplate_write_escaped(
                    oxiplate_formatter,
                    &::oxiplate::escapers::your_group::YourEscaper::foo,
                )?;
            oxiplate_formatter.write_str("\n")?;
            (&&::oxiplate::private::UnescapedTextWrapper::new(
                (&&::oxiplate::escapers::your_group::YourEscaper::build_text_handler(&(self.name)))
                    .prepare_text_for_output(&::oxiplate::escapers::your_group::YourEscaper::bar),
            ))
                .oxiplate_write_escaped(
                    oxiplate_formatter,
                    &::oxiplate::escapers::your_group::YourEscaper::bar,
                )?;
            oxiplate_formatter.write_str("\n")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &Html { name: r#"foo bar"# }.render().unwrap(),
            &r#"
f00 bar
f00 bar
foo b@r
"#,
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
    {
        match (
            &Html { name: r#"hello"# }.render().unwrap(),
            &r#"
hello
hello
hello
"#,
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
