use alloc::format;

use oxiplate_derive::Oxiplate;
extern crate test;
#[rustc_test_marker = "plaintext::empty"]
#[doc(hidden)]
pub static empty: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("plaintext::empty"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/plaintext.rs",
        start_line: 6usize,
        start_col: 4usize,
        end_line: 6usize,
        end_col: 9usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(empty()),
    ),
};
fn empty() {
    #[oxiplate_inline(r#"{% plaintext %}{% endplaintext %}"#)]
    struct Data;
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Data)) }),
            &"",
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
#[rustc_test_marker = "plaintext::text"]
#[doc(hidden)]
pub static text: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("plaintext::text"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/plaintext.rs",
        start_line: 15usize,
        start_col: 4usize,
        end_line: 15usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(text()),
    ),
};
fn text() {
    #[oxiplate_inline(r#"{% plaintext %}Hello world!{% endplaintext %}"#)]
    struct Data;
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("Hello world!")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Data)) }),
            &"Hello world!",
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
#[rustc_test_marker = "plaintext::leading_whitespace_remove"]
#[doc(hidden)]
pub static leading_whitespace_remove: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("plaintext::leading_whitespace_remove"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/plaintext.rs",
        start_line: 24usize,
        start_col: 4usize,
        end_line: 24usize,
        end_col: 29usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(leading_whitespace_remove()),
    ),
};
fn leading_whitespace_remove() {
    #[oxiplate_inline(r#"{% plaintext -%}   Hello world!   {% endplaintext %}"#)]
    struct Data;
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("Hello world!")?;
            oxiplate_formatter.write_str("   ")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Data)) }),
            &"Hello world!   ",
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
#[rustc_test_marker = "plaintext::leading_whitespace_collapse"]
#[doc(hidden)]
pub static leading_whitespace_collapse: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("plaintext::leading_whitespace_collapse"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/plaintext.rs",
        start_line: 33usize,
        start_col: 4usize,
        end_line: 33usize,
        end_col: 31usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(leading_whitespace_collapse()),
    ),
};
fn leading_whitespace_collapse() {
    #[oxiplate_inline(r#"{% plaintext _%}   Hello world!   {% endplaintext %}"#)]
    struct Data;
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter.write_str("Hello world!")?;
            oxiplate_formatter.write_str("   ")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Data)) }),
            &" Hello world!   ",
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
#[rustc_test_marker = "plaintext::trailing_whitespace_remove"]
#[doc(hidden)]
pub static trailing_whitespace_remove: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("plaintext::trailing_whitespace_remove"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/plaintext.rs",
        start_line: 42usize,
        start_col: 4usize,
        end_line: 42usize,
        end_col: 30usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(trailing_whitespace_remove()),
    ),
};
fn trailing_whitespace_remove() {
    #[oxiplate_inline(r#"{% plaintext %}   Hello world!   {%- endplaintext %}"#)]
    struct Data;
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("   Hello world!")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Data)) }),
            &"   Hello world!",
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
#[rustc_test_marker = "plaintext::trailing_whitespace_collapse"]
#[doc(hidden)]
pub static trailing_whitespace_collapse: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("plaintext::trailing_whitespace_collapse"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/plaintext.rs",
        start_line: 51usize,
        start_col: 4usize,
        end_line: 51usize,
        end_col: 32usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(trailing_whitespace_collapse()),
    ),
};
fn trailing_whitespace_collapse() {
    #[oxiplate_inline(r#"{% plaintext %}   Hello world!   {%_ endplaintext %}"#)]
    struct Data;
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("   Hello world!")?;
            oxiplate_formatter.write_str(" ")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Data)) }),
            &"   Hello world! ",
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
#[rustc_test_marker = "plaintext::writ"]
#[doc(hidden)]
pub static writ: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("plaintext::writ"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/plaintext.rs",
        start_line: 60usize,
        start_col: 4usize,
        end_line: 60usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(writ()),
    ),
};
fn writ() {
    #[oxiplate_inline(r#"{% plaintext %}Hello {{ "world" }}!{% endplaintext %}"#)]
    struct Data;
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("Hello {{ \"world\" }}!")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Data)) }),
            &r#"Hello {{ "world" }}!"#,
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
#[rustc_test_marker = "plaintext::writ_broken"]
#[doc(hidden)]
pub static writ_broken: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("plaintext::writ_broken"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/plaintext.rs",
        start_line: 69usize,
        start_col: 4usize,
        end_line: 69usize,
        end_col: 15usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(writ_broken()),
    ),
};
fn writ_broken() {
    #[oxiplate_inline(r#"{% plaintext %}{{ +_)(*&^%$#@!~"';',.><=-/?` }}{% endplaintext %}"#)]
    struct Data;
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("{{ +_)(*&^%$#@!~\"';',.><=-/?` }}")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Data)) }),
            &r#"{{ +_)(*&^%$#@!~"';',.><=-/?` }}"#,
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
#[rustc_test_marker = "plaintext::statement"]
#[doc(hidden)]
pub static statement: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("plaintext::statement"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/plaintext.rs",
        start_line: 78usize,
        start_col: 4usize,
        end_line: 78usize,
        end_col: 13usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(statement()),
    ),
};
fn statement() {
    #[oxiplate_inline(
        r#"{% plaintext %}Hello {% if true %}world{% else %}friend{% endif %}!{% endplaintext %}"#
    )]
    struct Data;
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("Hello {% if true %}world{% else %}friend{% endif %}!")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Data)) }),
            &r#"Hello {% if true %}world{% else %}friend{% endif %}!"#,
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
#[rustc_test_marker = "plaintext::statement_broken"]
#[doc(hidden)]
pub static statement_broken: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("plaintext::statement_broken"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/plaintext.rs",
        start_line: 92usize,
        start_col: 4usize,
        end_line: 92usize,
        end_col: 20usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(statement_broken()),
    ),
};
fn statement_broken() {
    #[oxiplate_inline(r#"{% plaintext %}{% if +_)(*&^%$#@!~"';',.><=-/?` %}{% endplaintext %}"#)]
    struct Data;
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("{% if +_)(*&^%$#@!~\"';',.><=-/?` %}")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Data)) }),
            &r#"{% if +_)(*&^%$#@!~"';',.><=-/?` %}"#,
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
#[rustc_test_marker = "plaintext::comment"]
#[doc(hidden)]
pub static comment: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("plaintext::comment"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/plaintext.rs",
        start_line: 101usize,
        start_col: 4usize,
        end_line: 101usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(comment()),
    ),
};
fn comment() {
    #[oxiplate_inline(r#"{% plaintext %}Hello {#- world #}!{% endplaintext %}"#)]
    struct Data;
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("Hello {#- world #}!")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Data)) }),
            &r#"Hello {#- world #}!"#,
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
#[rustc_test_marker = "plaintext::comment_broken"]
#[doc(hidden)]
pub static comment_broken: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("plaintext::comment_broken"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/plaintext.rs",
        start_line: 110usize,
        start_col: 4usize,
        end_line: 110usize,
        end_col: 18usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(comment_broken()),
    ),
};
fn comment_broken() {
    #[oxiplate_inline(r#"{% plaintext %}Hello {#- world!{% endplaintext %}"#)]
    struct Data;
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("Hello {#- world!")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Data)) }),
            &r#"Hello {#- world!"#,
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
