#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::Oxiplate;
#[oxiplate_inline = "
{%- if let Some(count) = cats_count -%}
    {%- if let Some(name) = name -%}
        Found {{ count }} cats named {{ name }}!
    {%- else -%}
        Found {{ count }} cats!
    {%- endif -%}
{%- else -%}
    {%- if let Some(missing_name) = name -%}
        No cats named {{ missing_name }} found :(
    {%- else -%}
        No cats found :(
    {%- endif -%}
{%- endif %}"]
struct Data {
    cats_count: Option<u8>,
    name: Option<String>,
}
impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(count) = &self.cats_count {
            if let Some(name) = &self.name {
                f.write_fmt(format_args!("{0}", "Found"))?;
                f.write_fmt(format_args!("{0}", " "))?;
                f.write_fmt(format_args!("{0}", count))?;
                f.write_fmt(format_args!("{0}", " "))?;
                f.write_fmt(format_args!("{0}", "cats named"))?;
                f.write_fmt(format_args!("{0}", " "))?;
                f.write_fmt(format_args!("{0}", name))?;
                f.write_fmt(format_args!("{0}", "!"))?;
            } else {
                f.write_fmt(format_args!("{0}", "Found"))?;
                f.write_fmt(format_args!("{0}", " "))?;
                f.write_fmt(format_args!("{0}", count))?;
                f.write_fmt(format_args!("{0}", " "))?;
                f.write_fmt(format_args!("{0}", "cats!"))?;
            }
        } else {
            if let Some(missing_name) = &self.name {
                f.write_fmt(format_args!("{0}", "No cats named"))?;
                f.write_fmt(format_args!("{0}", " "))?;
                f.write_fmt(format_args!("{0}", missing_name))?;
                f.write_fmt(format_args!("{0}", " "))?;
                f.write_fmt(format_args!("{0}", "found :("))?;
            } else {
                f.write_fmt(format_args!("{0}", "No cats found :("))?;
            }
        }
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_count"]
#[doc(hidden)]
pub const test_count: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_count"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\if-let.rs",
        start_line: 24usize,
        start_col: 4usize,
        end_line: 24usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_count()),
    ),
};
fn test_count() {
    let data = Data {
        cats_count: Some(5),
        name: None,
    };
    match (
        &::alloc::__export::must_use({
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        }),
        &"Found 5 cats!",
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
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_count_name"]
#[doc(hidden)]
pub const test_count_name: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_count_name"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\if-let.rs",
        start_line: 34usize,
        start_col: 4usize,
        end_line: 34usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_count_name()),
    ),
};
fn test_count_name() {
    let data = Data {
        cats_count: Some(5),
        name: Some(String::from("Sam")),
    };
    match (
        &::alloc::__export::must_use({
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        }),
        &"Found 5 cats named Sam!",
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
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_name"]
#[doc(hidden)]
pub const test_name: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_name"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\if-let.rs",
        start_line: 44usize,
        start_col: 4usize,
        end_line: 44usize,
        end_col: 13usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test_name())),
};
fn test_name() {
    let data = Data {
        cats_count: None,
        name: Some(String::from("Sam")),
    };
    match (
        &::alloc::__export::must_use({
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        }),
        &"No cats named Sam found :(",
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
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_none"]
#[doc(hidden)]
pub const test_none: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_none"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\if-let.rs",
        start_line: 54usize,
        start_col: 4usize,
        end_line: 54usize,
        end_col: 13usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test_none())),
};
fn test_none() {
    let data = Data {
        cats_count: None,
        name: None,
    };
    match (
        &::alloc::__export::must_use({
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        }),
        &"No cats found :(",
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
    test::test_main_static(&[&test_count, &test_count_name, &test_name, &test_none])
}
