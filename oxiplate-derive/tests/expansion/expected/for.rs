#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline("
{%- for value in &values -%}
    {{ value }}<br>
{%- endfor %}")]
struct Data {
    values: Vec<&'static str>,
}
impl ::std::fmt::Display for Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        for value in &self.values {
            f.write_fmt(format_args!("{0}<br>", value))?;
        }
        Ok(())
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
        start_line: 15usize,
        start_col: 4usize,
        end_line: 15usize,
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
        values: <[_]>::into_vec(::alloc::boxed::box_new(["foo", "bar", "baz"])),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"foo<br>bar<br>baz<br>",
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
        for person in &self.people {
            f.write_fmt(format_args!("{0}<br>", person.get_name()))?;
        }
        Ok(())
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
        start_line: 44usize,
        start_col: 4usize,
        end_line: 44usize,
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
        f.write_fmt(format_args!("{0}!\n", self.value))?;
        for value in &self.values {
            f.write_fmt(format_args!("{0}\n", value))?;
        }
        f.write_fmt(format_args!("{0} again :D", self.value))?;
        Ok(())
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
        start_line: 67usize,
        start_col: 4usize,
        end_line: 67usize,
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
        for function in &self.functions {
            f.write_fmt(format_args!("{0}\n", function()))?;
        }
        Ok(())
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
        start_line: 95usize,
        start_col: 4usize,
        end_line: 95usize,
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
        {
            let mut loop_ran = false;
            for value in &self.values {
                loop_ran = true;
                f.write_fmt(format_args!("{0}<br>", value))?;
            }
            if !loop_ran {
                f.write_str("No values :(")?;
            }
        }
        Ok(())
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
        start_line: 117usize,
        start_col: 4usize,
        end_line: 117usize,
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
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &test_for,
            &test_for_else,
            &test_function_variables,
            &test_method_calls,
            &test_shadow_variable,
        ],
    )
}
