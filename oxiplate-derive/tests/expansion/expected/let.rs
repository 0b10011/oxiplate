#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::vec::Vec;
use alloc::{format, vec};
use oxiplate_derive::Oxiplate;
#[oxiplate_inline(
    r#"
{{- value }}
{%- let value = 19 %}
{{ value }}
{%- let value = "89" %}
{{ value }}
"#
)]
struct Set {
    value: &'static str,
}
impl ::core::fmt::Display for Set {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(6usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
            let value = 19;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(value)))?;
            let value = "89";
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(value)))?;
            oxiplate_formatter.write_str("\n")?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "set"]
#[doc(hidden)]
pub const set: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("set"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/let.rs",
        start_line: 25usize,
        start_col: 4usize,
        end_line: 25usize,
        end_col: 7usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(set())),
};
fn set() {
    let data = Set { value: "Hello world!" };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"Hello world!\n19\n89\n",
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
    };
}
#[oxiplate_inline(
    r#"
{{- value }}
{% if value == "Hello world!" -%}
    if
    {{_ value }}
    {%- let value = 19 %}
    {{_ value }}
{%- elseif value == "Goodbye world!" -%}
    elseif
    {{_ value }}
    {%- let value = 89 %}
    {{_ value }}
{%- else -%}
    else
    {{_ value }}
    {%- let value = 42 %}
    {{_ value }}
{%- endif %}
{{ value }}
"#
)]
struct ShadowIf {
    value: &'static str,
}
impl ::core::fmt::Display for ShadowIf {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(11usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
            oxiplate_formatter.write_str("\n")?;
            if self.value == "Hello world!" {
                oxiplate_formatter.write_str("if ")?;
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
                let value = 19;
                oxiplate_formatter.write_str(" ")?;
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(value)))?;
            } else if self.value == "Goodbye world!" {
                oxiplate_formatter.write_str("elseif ")?;
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
                let value = 89;
                oxiplate_formatter.write_str(" ")?;
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(value)))?;
            } else {
                oxiplate_formatter.write_str("else ")?;
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
                let value = 42;
                oxiplate_formatter.write_str(" ")?;
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(value)))?;
            }
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
            oxiplate_formatter.write_str("\n")?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "shadow_if"]
#[doc(hidden)]
pub const shadow_if: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("shadow_if"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/let.rs",
        start_line: 61usize,
        start_col: 4usize,
        end_line: 61usize,
        end_col: 13usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(shadow_if())),
};
fn shadow_if() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", ShadowIf { value: "Hello world!" }))
        }),
        &"Hello world!\nif Hello world! 19\nHello world!\n",
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
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    ShadowIf {
                        value: "Goodbye world!",
                    },
                ),
            )
        }),
        &"Goodbye world!\nelseif Goodbye world! 89\nGoodbye world!\n",
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
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", ShadowIf { value: "foobar" }))
        }),
        &"foobar\nelse foobar 42\nfoobar\n",
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
    };
}
#[oxiplate_inline(
    r#"
{{- value }}
{% for number in &numbers %}
    {{- value }}
    {{_ number }}
    {%- let value = 19 %}
    {%- let number = 89 %}
    {{_ value }}
    {{_ number }}
{% endfor %}
{{- value }}
"#
)]
struct ShadowFor {
    value: &'static str,
    numbers: Vec<usize>,
}
impl ::core::fmt::Display for ShadowFor {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(20usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
            oxiplate_formatter.write_str("\n")?;
            for number in &self.numbers {
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
                oxiplate_formatter.write_str(" ")?;
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(number)))?;
                let value = 19;
                let number = 89;
                oxiplate_formatter.write_str(" ")?;
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(value)))?;
                oxiplate_formatter.write_str(" ")?;
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(number)))?;
                oxiplate_formatter.write_str("\n")?;
            }
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
            oxiplate_formatter.write_str("\n")?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "shadow_for"]
#[doc(hidden)]
pub const shadow_for: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("shadow_for"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/let.rs",
        start_line: 107usize,
        start_col: 4usize,
        end_line: 107usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(shadow_for()),
    ),
};
fn shadow_for() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    ShadowFor {
                        value: "Hello world!",
                        numbers: <[_]>::into_vec(::alloc::boxed::box_new([1, 2, 3])),
                    },
                ),
            )
        }),
        &"Hello world!\nHello world! 1 19 89\nHello world! 2 19 89\nHello world! 3 19 89\nHello \
         world!\n",
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
    };
}
#[oxiplate_inline(
    r#"
{{- value }}
{% match number %}
{% case 19 %}
    {{- value }}
    {%- let value = 19 %}
    {{_ value }}
{% case value %}
    {{- value }}
    {%- let value = "Goodbye world!" %}
    {{_ value }}
{% endmatch %}
{{- value }}
"#
)]
struct ShadowMatch {
    value: &'static str,
    number: usize,
}
impl ::core::fmt::Display for ShadowMatch {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(8usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
            oxiplate_formatter.write_str("\n")?;
            match self.number {
                19 => {
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
                    let value = 19;
                    oxiplate_formatter.write_str(" ")?;
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(value)))?;
                    oxiplate_formatter.write_str("\n")?;
                }
                value => {
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(value)))?;
                    let value = "Goodbye world!";
                    oxiplate_formatter.write_str(" ")?;
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(value)))?;
                    oxiplate_formatter.write_str("\n")?;
                }
            }
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
            oxiplate_formatter.write_str("\n")?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "shadow_match"]
#[doc(hidden)]
pub const shadow_match: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("shadow_match"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/let.rs",
        start_line: 144usize,
        start_col: 4usize,
        end_line: 144usize,
        end_col: 16usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(shadow_match()),
    ),
};
fn shadow_match() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    ShadowMatch {
                        value: "Hello world!",
                        number: 19,
                    },
                ),
            )
        }),
        &"Hello world!\nHello world! 19\nHello world!\n",
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
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    ShadowMatch {
                        value: "Hello world!",
                        number: 89,
                    },
                ),
            )
        }),
        &"Hello world!\n89 Goodbye world!\nHello world!\n",
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
    };
}
#[oxiplate_inline(
    r#"{% extends "let.html.oxip" %}
{% block content %}
    {{- value }}
    {%- let value = 69 %}
    {{_ value _}}
    |
    {%_ parent _%}
    |
    {{_ value }}
{%- endblock %}
{% block footer %}
    {{- value }}
    {%- let value = 420 %}
    {{_ value _}}
    |
    {%_ parent _%}
    |
    {{_ value }}
{%- endblock %}
"#
)]
struct Extends {
    value: &'static str,
}
impl ::core::fmt::Display for Extends {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(97usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter.write_str("<!DOCTYPE html>\n<header>")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
            oxiplate_formatter.write_str("</header>\n<main>")?;
            {
                {
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
                    let value = 69;
                    oxiplate_formatter.write_str(" ")?;
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(value)))?;
                    oxiplate_formatter.write_str(" | ")?;
                }
                {
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
                    let value = 19;
                    oxiplate_formatter.write_str(" ")?;
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(value)))?;
                }
                {}
                {
                    oxiplate_formatter.write_str(" | ")?;
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
                }
            }
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
            oxiplate_formatter.write_str("</main>\n")?;
            let value = 42;
            oxiplate_formatter.write_str("<footer>")?;
            {
                {
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
                    let value = 420;
                    oxiplate_formatter.write_str(" ")?;
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(value)))?;
                    oxiplate_formatter.write_str(" | ")?;
                }
                {
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(value)))?;
                    let value = 89;
                    oxiplate_formatter.write_str(" ")?;
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(value)))?;
                }
                {}
                {
                    oxiplate_formatter.write_str(" | ")?;
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
                }
            }
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(value)))?;
            oxiplate_formatter.write_str("</footer>\n")?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "extends"]
#[doc(hidden)]
pub const extends: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("extends"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/let.rs",
        start_line: 195usize,
        start_col: 4usize,
        end_line: 195usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(extends())),
};
fn extends() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", Extends { value: "foo" }))
        }),
        &r#"<!DOCTYPE html>
<header>foo</header>
<main>foo 69 | foo 19 | foo foo</main>
<footer>foo 420 | 42 89 | foo 42</footer>
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
    };
}
#[oxiplate_inline(r#"{% extends "let.html.oxip" %}"#)]
struct ExtendsDefault {
    value: &'static str,
}
impl ::core::fmt::Display for ExtendsDefault {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(77usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter.write_str("<!DOCTYPE html>\n<header>")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
            oxiplate_formatter.write_str("</header>\n<main>")?;
            {
                {}
                {
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
                    let value = 19;
                    oxiplate_formatter.write_str(" ")?;
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(value)))?;
                }
                {}
                {}
            }
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.value)))?;
            oxiplate_formatter.write_str("</main>\n")?;
            let value = 42;
            oxiplate_formatter.write_str("<footer>")?;
            {
                {}
                {
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(value)))?;
                    let value = 89;
                    oxiplate_formatter.write_str(" ")?;
                    oxiplate_formatter
                        .write_str(&alloc::string::ToString::to_string(&(value)))?;
                }
                {}
                {}
            }
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(value)))?;
            oxiplate_formatter.write_str("</footer>\n")?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "extends_default"]
#[doc(hidden)]
pub const extends_default: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("extends_default"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/let.rs",
        start_line: 213usize,
        start_col: 4usize,
        end_line: 213usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(extends_default()),
    ),
};
fn extends_default() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", ExtendsDefault { value: "foo" }))
        }),
        &r#"<!DOCTYPE html>
<header>foo</header>
<main>foo 19 foo</main>
<footer>42 89 42</footer>
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
    };
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[&extends, &extends_default, &set, &shadow_for, &shadow_if, &shadow_match],
    )
}
