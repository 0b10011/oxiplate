#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use oxiplate_derive::Oxiplate;
enum Name {
    Actual(String),
    Nickname { name: String },
    Missing,
}
#[oxiplate_inline(
    "
{%- if let Ok(name) = &name -%}
    {%- if let Some(cats_count) = cats_count -%}
        {%- if let Name::Actual(name) = name -%}
            Found {{ cats_count }} cats named {{ name }}!
        {%- elseif let Name::Nickname { name } = name -%}
            Found {{ cats_count }} cats nicknamed {{ name }}!
        {%- else -%}
            Found {{ cats_count }} cats!
        {%- endif -%}
    {%- else -%}
        {%- if let Name::Actual(missing_name) = &name -%}
            No cats named {{ missing_name }} found :(
        {%- elseif let Name::Nickname { name: missing_name } = &name -%}
            No cats nicknamed {{ missing_name }} found :(
        {%- else -%}
            No cats found :(
        {%- endif -%}
    {%- endif %}
{%- else -%}
    Name could not be fetched.
{%- endif -%}"
)]
struct Data {
    cats_count: Option<u8>,
    name: Result<Name, ()>,
}
impl ::std::fmt::Display for Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(13usize);
            let f = &mut string;
            if let Ok(name) = &self.name {
                if let Some(cats_count) = self.cats_count {
                    if let Name::Actual(name) = name {
                        f.write_str("Found ")?;
                        f.write_str(&::std::string::ToString::to_string(&(cats_count)))?;
                        f.write_str(" cats named ")?;
                        f.write_str(&::std::string::ToString::to_string(&(name)))?;
                        f.write_str("!")?;
                    } else if let Name::Nickname { name } = name {
                        f.write_str("Found ")?;
                        f.write_str(&::std::string::ToString::to_string(&(cats_count)))?;
                        f.write_str(" cats nicknamed ")?;
                        f.write_str(&::std::string::ToString::to_string(&(name)))?;
                        f.write_str("!")?;
                    } else {
                        f.write_str("Found ")?;
                        f.write_str(&::std::string::ToString::to_string(&(cats_count)))?;
                        f.write_str(" cats!")?;
                    }
                } else {
                    if let Name::Actual(missing_name) = &name {
                        f.write_str("No cats named ")?;
                        f.write_str(
                            &::std::string::ToString::to_string(&(missing_name)),
                        )?;
                        f.write_str(" found :(")?;
                    } else if let Name::Nickname { name: missing_name } = &name {
                        f.write_str("No cats nicknamed ")?;
                        f.write_str(
                            &::std::string::ToString::to_string(&(missing_name)),
                        )?;
                        f.write_str(" found :(")?;
                    } else {
                        f.write_str("No cats found :(")?;
                    }
                }
            } else {
                f.write_str("Name could not be fetched.")?;
            }
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "test_count"]
#[doc(hidden)]
pub const test_count: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_count"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/if-let.rs",
        start_line: 40usize,
        start_col: 4usize,
        end_line: 40usize,
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
        name: Ok(Name::Missing),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
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
#[rustc_test_marker = "test_count_name"]
#[doc(hidden)]
pub const test_count_name: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_count_name"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/if-let.rs",
        start_line: 50usize,
        start_col: 4usize,
        end_line: 50usize,
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
        name: Ok(Name::Actual(String::from("Sam"))),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
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
#[rustc_test_marker = "test_name"]
#[doc(hidden)]
pub const test_name: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_name"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/if-let.rs",
        start_line: 60usize,
        start_col: 4usize,
        end_line: 60usize,
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
        name: Ok(Name::Nickname {
            name: String::from("Sam"),
        }),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"No cats nicknamed Sam found :(",
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
#[rustc_test_marker = "test_none"]
#[doc(hidden)]
pub const test_none: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_none"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/if-let.rs",
        start_line: 72usize,
        start_col: 4usize,
        end_line: 72usize,
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
        name: Ok(Name::Missing),
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
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
