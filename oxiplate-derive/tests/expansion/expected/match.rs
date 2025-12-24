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
{%- match (&name, cats_count) -%}
    {%- case ( Ok ( Name::Actual ( name ) ) , Some ( cats_count ) ) -%}
        {# Extra whitespace intentionally inserted for coverage purposes -#}
        Found {{ cats_count }} cats named {{ name }}!
    {%- case (Ok(Name::Actual(missing_name)), None) -%}
        No cats named {{ missing_name }} found :(
    {%- case (Ok(Name::Nickname { name }), Some(cats_count)) -%}
        {# Extra whitespace intentionally skipped for coverage purposes -#}
        Found {{ cats_count }} cats nicknamed {{ name }}!
    {%- case (Ok(Name::Nickname { name: missing_name }), None) -%}
        No cats nicknamed {{ missing_name }} found :(
    {%- case (Ok(Name::Missing), Some(cats_count)) -%}
        Found {{ cats_count }} cats!
    {%- case (Ok(Name::Missing), None) -%}
        No cats found :(
    {%- case (Err(_), _) -%}
        Name could not be fetched.
{%- endmatch -%}"
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
            match (&self.name, self.cats_count) {
                (Ok(Name::Actual(name)), Some(cats_count)) => {
                    f.write_str("Found ")?;
                    f.write_str(&::std::string::ToString::to_string(&(cats_count)))?;
                    f.write_str(" cats named ")?;
                    f.write_str(&::std::string::ToString::to_string(&(name)))?;
                    f.write_str("!")?;
                }
                (Ok(Name::Actual(missing_name)), None) => {
                    f.write_str("No cats named ")?;
                    f.write_str(&::std::string::ToString::to_string(&(missing_name)))?;
                    f.write_str(" found :(")?;
                }
                (Ok(Name::Nickname { name }), Some(cats_count)) => {
                    f.write_str("Found ")?;
                    f.write_str(&::std::string::ToString::to_string(&(cats_count)))?;
                    f.write_str(" cats nicknamed ")?;
                    f.write_str(&::std::string::ToString::to_string(&(name)))?;
                    f.write_str("!")?;
                }
                (Ok(Name::Nickname { name: missing_name }), None) => {
                    f.write_str("No cats nicknamed ")?;
                    f.write_str(&::std::string::ToString::to_string(&(missing_name)))?;
                    f.write_str(" found :(")?;
                }
                (Ok(Name::Missing), Some(cats_count)) => {
                    f.write_str("Found ")?;
                    f.write_str(&::std::string::ToString::to_string(&(cats_count)))?;
                    f.write_str(" cats!")?;
                }
                (Ok(Name::Missing), None) => {
                    f.write_str("No cats found :(")?;
                }
                (Err(_), _) => {
                    f.write_str("Name could not be fetched.")?;
                }
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
        source_file: "oxiplate-derive/tests/match.rs",
        start_line: 37usize,
        start_col: 4usize,
        end_line: 37usize,
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
        source_file: "oxiplate-derive/tests/match.rs",
        start_line: 47usize,
        start_col: 4usize,
        end_line: 47usize,
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
        source_file: "oxiplate-derive/tests/match.rs",
        start_line: 57usize,
        start_col: 4usize,
        end_line: 57usize,
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
        source_file: "oxiplate-derive/tests/match.rs",
        start_line: 69usize,
        start_col: 4usize,
        end_line: 69usize,
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
impl ::std::fmt::Display for MultipleWrapper {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(2usize);
            let f = &mut string;
            if let Multiple { a: 10, b: 'b', c: "19", d: false } = self.multiple {
                f.write_str("bad")?;
            } else if let Multiple { a: 10, b: 'b', c: "19", d: true } = self.multiple {
                f.write_str("yes")?;
            } else {
                f.write_str("no")?;
            }
            string
        };
        f.write_str(&string)
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
        source_file: "oxiplate-derive/tests/match.rs",
        start_line: 102usize,
        start_col: 4usize,
        end_line: 102usize,
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
impl ::std::fmt::Display for Outer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(6usize);
            let f = &mut string;
            if let MiddleA { a: InnerA { value: 42 }, b: InnerB(b) } = self.a {
                f.write_str("a.b: ")?;
                f.write_str(&::std::string::ToString::to_string(&(b)))?;
            } else if let MiddleB(InnerA { value: a }, InnerB(42)) = self.b {
                f.write_str("b.a: ")?;
                f.write_str(&::std::string::ToString::to_string(&(a)))?;
            }
            string
        };
        f.write_str(&string)
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
        source_file: "oxiplate-derive/tests/match.rs",
        start_line: 149usize,
        start_col: 4usize,
        end_line: 149usize,
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
