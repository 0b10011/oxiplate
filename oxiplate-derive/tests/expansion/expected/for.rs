#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline(
    r#"
{%- for a in &values -%}
    {%- for b in &values -%}
        {{ a ~ " - " ~ b }}<br>
    {%- endfor %}
{%- endfor %}"#
)]
struct Data {
    values: Vec<&'static str>,
}
impl ::std::fmt::Display for Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(36usize);
            let f = &mut string;
            for a in &self.values {
                for b in &self.values {
                    f.write_str(
                        &::std::string::ToString::to_string(
                            &(::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!("{0} - {1}", a, b))
                            })),
                        ),
                    )?;
                    f.write_str("<br>")?;
                }
            }
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "test_for"]
#[doc(hidden)]
pub const test_for: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_for"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/for.rs",
        start_line: 17usize,
        start_col: 4usize,
        end_line: 17usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test_for())),
};
fn test_for() {
    let data = Data {
        values: <[_]>::into_vec(::alloc::boxed::box_new(["foo", "bar"])),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"foo - foo<br>foo - bar<br>bar - foo<br>bar - bar<br>",
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
    "
{%- for person in &people -%}
    {{ person.get_name() }}<br>
{%- endfor %}"
)]
struct Accounts {
    people: Vec<Person>,
}
impl ::std::fmt::Display for Accounts {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(10usize);
            let f = &mut string;
            for person in &self.people {
                f.write_str(&::std::string::ToString::to_string(&(person.get_name())))?;
                f.write_str("<br>")?;
            }
            string
        };
        f.write_str(&string)
    }
}
struct Person {
    name: &'static str,
}
impl Person {
    pub fn get_name(&self) -> &'static str {
        self.name
    }
}
extern crate test;
#[rustc_test_marker = "test_method_calls"]
#[doc(hidden)]
pub const test_method_calls: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_method_calls"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/for.rs",
        start_line: 49usize,
        start_col: 4usize,
        end_line: 49usize,
        end_col: 21usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_method_calls()),
    ),
};
fn test_method_calls() {
    let data = Accounts {
        people: <[_]>::into_vec(
            ::alloc::boxed::box_new([Person { name: "Zoe" }, Person { name: "Alice" }]),
        ),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"Zoe<br>Alice<br>",
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
    "
{{- value }}!
{% for value in &values -%}
    {{ value }}
{% endfor -%}
{{ value }} again :D"
)]
struct ShadowVariable {
    values: Vec<&'static str>,
    value: &'static str,
}
impl ::std::fmt::Display for ShadowVariable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(17usize);
            let f = &mut string;
            f.write_str(&::std::string::ToString::to_string(&(self.value)))?;
            f.write_str("!\n")?;
            for value in &self.values {
                f.write_str(&::std::string::ToString::to_string(&(value)))?;
                f.write_str("\n")?;
            }
            f.write_str(&::std::string::ToString::to_string(&(self.value)))?;
            f.write_str(" again :D")?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "test_shadow_variable"]
#[doc(hidden)]
pub const test_shadow_variable: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_shadow_variable"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/for.rs",
        start_line: 72usize,
        start_col: 4usize,
        end_line: 72usize,
        end_col: 24usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_shadow_variable()),
    ),
};
fn test_shadow_variable() {
    let data = ShadowVariable {
        values: <[_]>::into_vec(::alloc::boxed::box_new(["foo", "bar", "baz"])),
        value: "hello world",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"hello world!
foo
bar
baz
hello world again :D",
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
    "
{%- for function in &functions -%}
    {{ function() }}
{% endfor %}"
)]
struct Functions {
    functions: Vec<fn() -> i32>,
}
impl ::std::fmt::Display for Functions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(4usize);
            let f = &mut string;
            for function in &self.functions {
                f.write_str(&::std::string::ToString::to_string(&(function())))?;
                f.write_str("\n")?;
            }
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "test_function_variables"]
#[doc(hidden)]
pub const test_function_variables: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_function_variables"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/for.rs",
        start_line: 100usize,
        start_col: 4usize,
        end_line: 100usize,
        end_col: 27usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_function_variables()),
    ),
};
fn test_function_variables() {
    let data = Functions {
        functions: <[_]>::into_vec(::alloc::boxed::box_new([|| 19, || 89])),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"19\n89\n",
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
    "
{%- for value in &values -%}
    {{ value }}<br>
{%- else -%}
    No values :(
{%- endfor %}"
)]
struct ForElse {
    values: Vec<&'static str>,
}
impl ::std::fmt::Display for ForElse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(10usize);
            let f = &mut string;
            {
                let mut loop_ran = false;
                for value in &self.values {
                    loop_ran = true;
                    f.write_str(&::std::string::ToString::to_string(&(value)))?;
                    f.write_str("<br>")?;
                }
                if !loop_ran {
                    f.write_str("No values :(")?;
                }
            }
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "test_for_else"]
#[doc(hidden)]
pub const test_for_else: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_for_else"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/for.rs",
        start_line: 122usize,
        start_col: 4usize,
        end_line: 122usize,
        end_col: 17usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_for_else()),
    ),
};
fn test_for_else() {
    let data = ForElse {
        values: ::alloc::vec::Vec::new(),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"No values :(",
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
    "
{%- for value in &values -%}
    {% if *value == 23 -%}
        {% continue -%}
    {% endif -%}

    {{ value _}}
{% endfor %}"
)]
struct Continue {
    values: Vec<usize>,
}
impl ::std::fmt::Display for Continue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(4usize);
            let f = &mut string;
            for value in &self.values {
                if *value == 23 {
                    continue;
                }
                f.write_str(&::std::string::ToString::to_string(&(value)))?;
                f.write_str(" ")?;
            }
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "test_continue"]
#[doc(hidden)]
pub const test_continue: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_continue"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/for.rs",
        start_line: 144usize,
        start_col: 4usize,
        end_line: 144usize,
        end_col: 17usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_continue()),
    ),
};
fn test_continue() {
    let data = Continue {
        values: <[_]>::into_vec(::alloc::boxed::box_new([19, 23, 89])),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"19 89 ",
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
    "
{%- for value in &values -%}
    {% if *value > 42 -%}
        {% break -%}
    {% endif -%}

    {{ value _}}
{% endfor %}"
)]
struct Break {
    values: Vec<usize>,
}
impl ::std::fmt::Display for Break {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(4usize);
            let f = &mut string;
            for value in &self.values {
                if *value > 42 {
                    break;
                }
                f.write_str(&::std::string::ToString::to_string(&(value)))?;
                f.write_str(" ")?;
            }
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "test_break"]
#[doc(hidden)]
pub const test_break: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_break"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/for.rs",
        start_line: 168usize,
        start_col: 4usize,
        end_line: 168usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_break()),
    ),
};
fn test_break() {
    let data = Break {
        values: <[_]>::into_vec(::alloc::boxed::box_new([19, 23, 89])),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"19 23 ",
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
    "
{%- for _value in &values -%}
    {% break %}
{%- else -%}
    No values :(
{%- endfor %}"
)]
struct BreakElse {
    values: Vec<usize>,
}
impl ::std::fmt::Display for BreakElse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(0usize);
            let f = &mut string;
            {
                let mut loop_ran = false;
                for _value in &self.values {
                    loop_ran = true;
                    break;
                }
                if !loop_ran {
                    f.write_str("No values :(")?;
                }
            }
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "test_break_else"]
#[doc(hidden)]
pub const test_break_else: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_break_else"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/for.rs",
        start_line: 190usize,
        start_col: 4usize,
        end_line: 190usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_break_else()),
    ),
};
fn test_break_else() {
    let data = BreakElse {
        values: <[_]>::into_vec(::alloc::boxed::box_new([19, 89])),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
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
    };
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &test_break,
            &test_break_else,
            &test_continue,
            &test_for,
            &test_for_else,
            &test_function_variables,
            &test_method_calls,
            &test_shadow_variable,
        ],
    )
}
