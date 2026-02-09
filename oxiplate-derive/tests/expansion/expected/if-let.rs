#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use alloc::string::String;
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
        {%- if let Name::Actual ( name ) = name -%}
            {# Extra whitespace intentionally inserted for coverage purposes -#}
            Found {{ cats_count }} cats named {{ name }}!
        {%- elseif let Name::Nickname{name}=name -%}
            {# Extra whitespace intentionally skipped for coverage purposes -#}
            Found {{ cats_count }} cats nicknamed {{ name }}!
        {%- else -%}
            Found {{ cats_count }} cats!
        {%- endif -%}
    {%- elseif let core::option::Option::None = cats_count -%}
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
impl ::core::fmt::Display for Data {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(13usize);
            let oxiplate_formatter = &mut string;
            if let Ok(name) = &self.name {
                if let Some(cats_count) = self.cats_count {
                    if let Name::Actual(name) = name {
                        oxiplate_formatter.write_str("Found ")?;
                        oxiplate_formatter
                            .write_str(
                                &alloc::string::ToString::to_string(&(cats_count)),
                            )?;
                        oxiplate_formatter.write_str(" cats named ")?;
                        oxiplate_formatter
                            .write_str(&alloc::string::ToString::to_string(&(name)))?;
                        oxiplate_formatter.write_str("!")?;
                    } else if let Name::Nickname { name } = name {
                        oxiplate_formatter.write_str("Found ")?;
                        oxiplate_formatter
                            .write_str(
                                &alloc::string::ToString::to_string(&(cats_count)),
                            )?;
                        oxiplate_formatter.write_str(" cats nicknamed ")?;
                        oxiplate_formatter
                            .write_str(&alloc::string::ToString::to_string(&(name)))?;
                        oxiplate_formatter.write_str("!")?;
                    } else {
                        oxiplate_formatter.write_str("Found ")?;
                        oxiplate_formatter
                            .write_str(
                                &alloc::string::ToString::to_string(&(cats_count)),
                            )?;
                        oxiplate_formatter.write_str(" cats!")?;
                    }
                } else if let core::option::Option::None = self.cats_count {
                    if let Name::Actual(missing_name) = &name {
                        oxiplate_formatter.write_str("No cats named ")?;
                        oxiplate_formatter
                            .write_str(
                                &alloc::string::ToString::to_string(&(missing_name)),
                            )?;
                        oxiplate_formatter.write_str(" found :(")?;
                    } else if let Name::Nickname { name: missing_name } = &name {
                        oxiplate_formatter.write_str("No cats nicknamed ")?;
                        oxiplate_formatter
                            .write_str(
                                &alloc::string::ToString::to_string(&(missing_name)),
                            )?;
                        oxiplate_formatter.write_str(" found :(")?;
                    } else {
                        oxiplate_formatter.write_str("No cats found :(")?;
                    }
                }
            } else {
                oxiplate_formatter.write_str("Name could not be fetched.")?;
            }
            string
        };
        oxiplate_formatter.write_str(&string)
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
        start_line: 49usize,
        start_col: 4usize,
        end_line: 49usize,
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
        start_line: 59usize,
        start_col: 4usize,
        end_line: 59usize,
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
        start_line: 69usize,
        start_col: 4usize,
        end_line: 69usize,
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
        start_line: 81usize,
        start_col: 4usize,
        end_line: 81usize,
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
struct Multiple {
    a: usize,
    b: char,
    c: &'static str,
    d: bool,
}
#[oxiplate_inline(
    r#"
{%- if let Multiple { a: 10,b:'b' , c: "19", d: false } = multiple -%}
    bad
{%- elseif let Multiple { a: 10,b:'b' , c: "19", d: true } = multiple -%}
    yes
{%- else -%}
    no
{%- endif -%}
"#
)]
struct MultipleWrapper {
    multiple: Multiple,
}
impl ::core::fmt::Display for MultipleWrapper {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(2usize);
            let oxiplate_formatter = &mut string;
            if let Multiple { a: 10, b: 'b', c: "19", d: false } = self.multiple {
                oxiplate_formatter.write_str("bad")?;
            } else if let Multiple { a: 10, b: 'b', c: "19", d: true } = self.multiple {
                oxiplate_formatter.write_str("yes")?;
            } else {
                oxiplate_formatter.write_str("no")?;
            }
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "test_multiple"]
#[doc(hidden)]
pub const test_multiple: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_multiple"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/if-let.rs",
        start_line: 114usize,
        start_col: 4usize,
        end_line: 114usize,
        end_col: 17usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_multiple()),
    ),
};
fn test_multiple() {
    match (
        &"yes",
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    MultipleWrapper {
                        multiple: Multiple {
                            a: 10,
                            b: 'b',
                            c: "19",
                            d: true,
                        },
                    },
                ),
            )
        }),
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
}
struct InnerA {
    value: usize,
}
struct InnerB(usize);
struct MiddleA {
    a: InnerA,
    b: InnerB,
}
struct MiddleB(InnerA, InnerB);
#[oxiplate_inline(
    r#"
{%- if let MiddleA { a: InnerA { value: 42 } , b: InnerB(b) } = a -%}
    {# Extra whitespace before comma intentional for coverage -#}
    a.b: {{ b }}
{%- elseif let MiddleB(InnerA { value: a } , InnerB(42)) = b -%}
    {# Extra whitespace before comma intentional for coverage -#}
    b.a: {{ a }}
{%- endif -%}
"#
)]
struct Outer {
    a: MiddleA,
    b: MiddleB,
}
impl ::core::fmt::Display for Outer {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(6usize);
            let oxiplate_formatter = &mut string;
            if let MiddleA { a: InnerA { value: 42 }, b: InnerB(b) } = self.a {
                oxiplate_formatter.write_str("a.b: ")?;
                oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(b)))?;
            } else if let MiddleB(InnerA { value: a }, InnerB(42)) = self.b {
                oxiplate_formatter.write_str("b.a: ")?;
                oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(a)))?;
            }
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "nested"]
#[doc(hidden)]
pub const nested: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("nested"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/if-let.rs",
        start_line: 161usize,
        start_col: 4usize,
        end_line: 161usize,
        end_col: 10usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(nested())),
};
fn nested() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    Outer {
                        a: MiddleA {
                            a: InnerA { value: 42 },
                            b: InnerB(19),
                        },
                        b: MiddleB(InnerA { value: 89 }, InnerB(42)),
                    },
                ),
            )
        }),
        &"a.b: 19",
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
                    Outer {
                        a: MiddleA {
                            a: InnerA { value: 64 },
                            b: InnerB(19),
                        },
                        b: MiddleB(InnerA { value: 89 }, InnerB(42)),
                    },
                ),
            )
        }),
        &"b.a: 89",
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
                    Outer {
                        a: MiddleA {
                            a: InnerA { value: 64 },
                            b: InnerB(19),
                        },
                        b: MiddleB(InnerA { value: 89 }, InnerB(16)),
                    },
                ),
            )
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
        &[&nested, &test_count, &test_count_name, &test_multiple, &test_name, &test_none],
    )
}
