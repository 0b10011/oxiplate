#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::Oxiplate;
#[oxiplate_inline = "
{%- for value in &values -%}
    {{ value }}<br>
{%- endfor %}"]
struct Data {
    values: Vec<&'static str>,
}
impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for value in &self.values {
            f.write_fmt(
                format_args!(
                    "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", value));
                    res } .chars().map(| character | match character { '&' => { let res =
                    ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                    ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                    ::alloc::fmt::format(format_args!("{0}", character)); res } })
                    .collect::< String > ()
                ),
            )?;
            f.write_fmt(format_args!("{0}", "<br>"))?;
        }
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_for"]
pub const test_for: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_for"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\for.rs",
        start_line: 13usize,
        start_col: 4usize,
        end_line: 13usize,
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
        values: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new(["foo", "bar", "baz"]),
        ),
    };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        },
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
#[oxiplate_inline = "
{%- for person in &people -%}
    {{ person.get_name() }}<br>
{%- endfor %}"]
struct Accounts {
    people: Vec<Person>,
}
impl std::fmt::Display for Accounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for person in &self.people {
            f.write_fmt(
                format_args!(
                    "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", person
                    .get_name())); res } .chars().map(| character | match character { '&'
                    => { let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<'
                    => { let res = ::alloc::fmt::format(format_args!("&lt;")); res } _ =>
                    { let res = ::alloc::fmt::format(format_args!("{0}", character)); res
                    } }).collect::< String > ()
                ),
            )?;
            f.write_fmt(format_args!("{0}", "<br>"))?;
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
#[cfg(test)]
#[rustc_test_marker = "test_method_calls"]
pub const test_method_calls: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_method_calls"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\for.rs",
        start_line: 40usize,
        start_col: 4usize,
        end_line: 40usize,
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
            #[rustc_box]
            ::alloc::boxed::Box::new([Person { name: "Zoe" }, Person { name: "Alice" }]),
        ),
    };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        },
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
#[oxiplate_inline = "
{{- value }}!
{% for value in &values -%}
    {{ value }}
{% endfor -%}
{{ value }} again :D"]
struct ShadowVariable {
    values: Vec<&'static str>,
    value: &'static str,
}
impl std::fmt::Display for ShadowVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.value));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "!"))?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        for value in &self.values {
            f.write_fmt(
                format_args!(
                    "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", value));
                    res } .chars().map(| character | match character { '&' => { let res =
                    ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                    ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                    ::alloc::fmt::format(format_args!("{0}", character)); res } })
                    .collect::< String > ()
                ),
            )?;
            f.write_fmt(format_args!("{0}", "\n"))?;
        }
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.value));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "again :D"))?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_shadow_variable"]
pub const test_shadow_variable: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_shadow_variable"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\for.rs",
        start_line: 61usize,
        start_col: 4usize,
        end_line: 61usize,
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
        values: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new(["foo", "bar", "baz"]),
        ),
        value: "hello world",
    };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        },
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
#[oxiplate_inline = "
{%- for function in &functions -%}
    {{ function() }}
{% endfor %}"]
struct Functions {
    functions: Vec<fn() -> i32>,
}
impl std::fmt::Display for Functions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for function in &self.functions {
            f.write_fmt(
                format_args!(
                    "{0}", { let res = ::alloc::fmt::format(format_args!("{0}",
                    function())); res } .chars().map(| character | match character { '&'
                    => { let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<'
                    => { let res = ::alloc::fmt::format(format_args!("&lt;")); res } _ =>
                    { let res = ::alloc::fmt::format(format_args!("{0}", character)); res
                    } }).collect::< String > ()
                ),
            )?;
            f.write_fmt(format_args!("{0}", "\n"))?;
        }
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_function_variables"]
pub const test_function_variables: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_function_variables"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\for.rs",
        start_line: 87usize,
        start_col: 4usize,
        end_line: 87usize,
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
        functions: <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([|| 19, || 89])),
    };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        },
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
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[&test_for, &test_function_variables, &test_method_calls, &test_shadow_variable],
    )
}
