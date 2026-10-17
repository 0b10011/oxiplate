use alloc::format;

use oxiplate_derive::Oxiplate;
#[oxiplate = "extends.html.oxip"]
struct AbsoluteData {
    title: &'static str,
    message: &'static str,
}
impl ::core::fmt::Display for AbsoluteData {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str("<!DOCTYPE html>\n<title>")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.title)))?;
        oxiplate_formatter.write_str("</title>\n")?;
        {
            oxiplate_formatter.write_str("<h1>")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.title)))?;
            oxiplate_formatter.write_str("</h1>\n  <p>")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.message)))?;
            oxiplate_formatter.write_str("</p>")?;
        }
        oxiplate_formatter.write_str("\n")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "external::extends::absolute"]
#[doc(hidden)]
pub static absolute: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("external::extends::absolute"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/external/extends.rs",
        start_line: 13usize,
        start_col: 4usize,
        end_line: 13usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(absolute()),
    ),
};
fn absolute() {
    let data = AbsoluteData {
        title: "Oxiplate Example",
        message: "Hello world!",
    };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
            &"<!DOCTYPE html>\n<title>Oxiplate Example</title>\n<h1>Oxiplate Example</h1>\n  \
              <p>Hello world!</p>\n",
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
#[rustc_test_marker = "external::extends::absolute_2"]
#[doc(hidden)]
pub static absolute_2: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("external::extends::absolute_2"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/external/extends.rs",
        start_line: 27usize,
        start_col: 4usize,
        end_line: 27usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(absolute_2()),
    ),
};
fn absolute_2() {
    let data = AbsoluteData {
        title: "Oxiplate Example #2",
        message: "Goodbye world!",
    };
    {
        match (&::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}", data))
                        }),
                &"<!DOCTYPE html>\n<title>Oxiplate Example #2</title>\n<h1>Oxiplate Example #2</h1>\n  \
         <p>Goodbye world!</p>\n")
            {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
            }
        }
    };
}
#[oxiplate_inline(
    r#"{#- Should be ignored -#}
{% extends "extends-wrapper.html.oxip" %}
{#- Should be ignored -#}
{% block content -%}
    {#- Should be ignored -#}
    <p>{{ message }}</p>
    {#- Should be ignored -#}
    {%- parent %}
    {#- Should be ignored -#}
{%- endblock %}
{#- Should be ignored -#}
"#
)]
struct Comments {
    title: &'static str,
    message: &'static str,
}
impl ::core::fmt::Display for Comments {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str("<!DOCTYPE html>\n<title>")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.title)))?;
        oxiplate_formatter.write_str("</title>\n")?;
        {
            {
                oxiplate_formatter.write_str("<p>")?;
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(self.message)))?;
                oxiplate_formatter.write_str("</p>")?;
            }
            {
                oxiplate_formatter.write_str("test")?;
            }
            {}
            {}
        }
        oxiplate_formatter.write_str("\n")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "external::extends::comments"]
#[doc(hidden)]
pub static comments: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("external::extends::comments"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/external/extends.rs",
        start_line: 61usize,
        start_col: 4usize,
        end_line: 61usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(comments()),
    ),
};
fn comments() {
    let data = Comments {
        title: "Comments",
        message: "Comments are fine anywhere",
    };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
            &"<!DOCTYPE html>\n<title>Comments</title>\n<p>Comments are fine anywhere</p>test\n",
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
#[oxiplate_inline(
    r#"{% extends "extends-wrapper.html.oxip" %}
{% block content -%}
    <p>{{ message }}</p>
    {%- parent %}
{%- endblock %}
"#
)]
struct Prefix {
    title: &'static str,
    message: &'static str,
}
impl ::core::fmt::Display for Prefix {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str("<!DOCTYPE html>\n<title>")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.title)))?;
        oxiplate_formatter.write_str("</title>\n")?;
        {
            {
                oxiplate_formatter.write_str("<p>")?;
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(self.message)))?;
                oxiplate_formatter.write_str("</p>")?;
            }
            {
                oxiplate_formatter.write_str("test")?;
            }
            {}
            {}
        }
        oxiplate_formatter.write_str("\n")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "external::extends::prefix"]
#[doc(hidden)]
pub static prefix: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("external::extends::prefix"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/external/extends.rs",
        start_line: 88usize,
        start_col: 4usize,
        end_line: 88usize,
        end_col: 10usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(prefix()),
    ),
};
fn prefix() {
    let data = Prefix {
        title: "Prefixed block",
        message: "Hello world!",
    };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
            &"<!DOCTYPE html>\n<title>Prefixed block</title>\n<p>Hello world!</p>test\n",
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
#[oxiplate_inline(
    r#"{% extends "extends-wrapper.html.oxip" %}
{% block content -%}
    <p>{{ message }}</p>
{%- endblock %}
"#
)]
struct Replace {
    title: &'static str,
    message: &'static str,
}
impl ::core::fmt::Display for Replace {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str("<!DOCTYPE html>\n<title>")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.title)))?;
        oxiplate_formatter.write_str("</title>\n")?;
        {
            oxiplate_formatter.write_str("<p>")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.message)))?;
            oxiplate_formatter.write_str("</p>")?;
        }
        oxiplate_formatter.write_str("\n")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "external::extends::replace"]
#[doc(hidden)]
pub static replace: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("external::extends::replace"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/external/extends.rs",
        start_line: 114usize,
        start_col: 4usize,
        end_line: 114usize,
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
    let data = Replace {
        title: "Replaced block",
        message: "Hello world!",
    };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
            &"<!DOCTYPE html>\n<title>Replaced block</title>\n<p>Hello world!</p>\n",
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
#[oxiplate_inline(
    r#"{% extends "extends-wrapper.html.oxip" %}
{% block content -%}
    {% parent -%}
    <p>{{ message }}</p>
{%- endblock %}
"#
)]
struct Suffix {
    title: &'static str,
    message: &'static str,
}
impl ::core::fmt::Display for Suffix {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str("<!DOCTYPE html>\n<title>")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.title)))?;
        oxiplate_formatter.write_str("</title>\n")?;
        {
            {}
            {
                oxiplate_formatter.write_str("test")?;
            }
            {}
            {
                oxiplate_formatter.write_str("<p>")?;
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(self.message)))?;
                oxiplate_formatter.write_str("</p>")?;
            }
        }
        oxiplate_formatter.write_str("\n")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "external::extends::suffix"]
#[doc(hidden)]
pub static suffix: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("external::extends::suffix"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/external/extends.rs",
        start_line: 141usize,
        start_col: 4usize,
        end_line: 141usize,
        end_col: 10usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(suffix()),
    ),
};
fn suffix() {
    let data = Suffix {
        title: "Suffixed block",
        message: "Hello world!",
    };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
            &"<!DOCTYPE html>\n<title>Suffixed block</title>\ntest<p>Hello world!</p>\n",
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
