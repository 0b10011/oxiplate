#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use std::fmt::Display;

use oxiplate_derive::Oxiplate;

enum Name {
    Actual(String),
    Nickname {
        name: String,
    },
    Missing,
}

#[oxiplate_inline("
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
{%- endmatch -%}")]
struct Data {
    cats_count: Option<u8>,
    name: Result<Name, ()>,
}
impl ::std::fmt::Display for Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string =
            {
                use ::std::fmt::Write;
                let mut string = String::with_capacity(13usize);
                let f = &mut string;
                match (&self.name, self.cats_count) {
                    (Ok(Name::Actual(name)), Some(cats_count)) => {
                        f.write_str("Found ")?;
                        f.write_str(&::std::string::ToString::to_string(&(cats_count)))?;
                        ;
                        f.write_str(" cats named ")?;
                        f.write_str(&::std::string::ToString::to_string(&(name)))?;
                        ;
                        f.write_str("!")?;
                    }
                    (Ok(Name::Actual(missing_name)), None) => {
                        f.write_str("No cats named ")?;
                        f.write_str(&::std::string::ToString::to_string(&(missing_name)))?;
                        ;
                        f.write_str(" found :(")?;
                    }
                    (Ok(Name::Nickname { name }), Some(cats_count)) => {
                        f.write_str("Found ")?;
                        f.write_str(&::std::string::ToString::to_string(&(cats_count)))?;
                        ;
                        f.write_str(" cats nicknamed ")?;
                        f.write_str(&::std::string::ToString::to_string(&(name)))?;
                        ;
                        f.write_str("!")?;
                    }
                    (Ok(Name::Nickname { name: missing_name }), None) => {
                        f.write_str("No cats nicknamed ")?;
                        f.write_str(&::std::string::ToString::to_string(&(missing_name)))?;
                        ;
                        f.write_str(" found :(")?;
                    }
                    (Ok(Name::Missing), Some(cats_count)) => {
                        f.write_str("Found ")?;
                        f.write_str(&::std::string::ToString::to_string(&(cats_count)))?;
                        ;
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
pub const test_count: test::TestDescAndFn =
    test::TestDescAndFn {

        desc: test::TestDesc {
            name: test::StaticTestName("test_count"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "oxiplate-derive/tests/match.rs",
            start_line: 39usize,
            start_col: 4usize,
            end_line: 39usize,
            end_col: 14usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] ||
                test::assert_test_result(test_count())),
    };
fn test_count() {
    let data = Data { cats_count: Some(5), name: Ok(Name::Missing) };






















    match (&::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}", data))
                    }), &"Found 5 cats!") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_count_name"]
#[doc(hidden)]
pub const test_count_name: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_count_name"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "oxiplate-derive/tests/match.rs",
            start_line: 49usize,
            start_col: 4usize,
            end_line: 49usize,
            end_col: 19usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] ||
                test::assert_test_result(test_count_name())),
    };
fn test_count_name() {
    let data =
        Data {
            cats_count: Some(5),
            name: Ok(Name::Actual(String::from("Sam"))),
        };
    match (&::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}", data))
                    }), &"Found 5 cats named Sam!") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_name"]
#[doc(hidden)]
pub const test_name: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_name"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "oxiplate-derive/tests/match.rs",
            start_line: 59usize,
            start_col: 4usize,
            end_line: 59usize,
            end_col: 13usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] ||
                test::assert_test_result(test_name())),
    };
fn test_name() {
    let data =
        Data {
            cats_count: None,
            name: Ok(Name::Nickname { name: String::from("Sam") }),
        };
    match (&::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}", data))
                    }), &"No cats nicknamed Sam found :(") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
}
extern crate test;
#[rustc_test_marker = "test_none"]
#[doc(hidden)]
pub const test_none: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_none"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "oxiplate-derive/tests/match.rs",
            start_line: 71usize,
            start_col: 4usize,
            end_line: 71usize,
            end_col: 13usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] ||
                test::assert_test_result(test_none())),
    };
fn test_none() {
    let data = Data { cats_count: None, name: Ok(Name::Missing) };
    match (&::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}", data))
                    }), &"No cats found :(") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
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
#[oxiplate_inline(r#"
{%- if let Multiple { a: 10,b:'b' , c: "19", d: false } = multiple -%}
    bad
{%- elseif let Multiple { a: 10,b:'b' , c: "19", d: true } = multiple -%}
    yes
{%- else -%}
    no
{%- endif -%}
"#)]
struct MultipleWrapper {
    multiple: Multiple,
}
impl ::std::fmt::Display for MultipleWrapper {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string =
            {
                use ::std::fmt::Write;
                let mut string = String::with_capacity(2usize);
                let f = &mut string;
                if let Multiple { a: 10, b: 'b', c: "19", d: false } =
                        self.multiple {
                    f.write_str("bad")?;
                } else if let Multiple { a: 10, b: 'b', c: "19", d: true } =
                        self.multiple {
                    f.write_str("yes")?;
                } else { f.write_str("no")?; }
                string
            };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "test_multiple"]
#[doc(hidden)]
pub const test_multiple: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_multiple"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "oxiplate-derive/tests/match.rs",
            start_line: 104usize,
            start_col: 4usize,
            end_line: 104usize,
            end_col: 17usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] ||
                test::assert_test_result(test_multiple())),
    };
fn test_multiple() {
    match (&"yes",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                MultipleWrapper {
                                    multiple: Multiple { a: 10, b: 'b', c: "19", d: true },
                                }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    }
}
struct InnerA<T: Display> {
    value: T,
}
struct InnerB<T: Display>(T);
struct MiddleA<A: Display, B: Display> {
    a: InnerA<A>,
    b: InnerB<B>,
}
struct MiddleB<A: Display, B: Display>(InnerA<A>, InnerB<B>);
#[oxiplate_inline(r#"
{%- if let MiddleA { a: InnerA { value: 42 } , b: InnerB(b) } = a -%}
    {# Extra whitespace before comma intentional for coverage -#}
    a.b: {{ b }}
{%- elseif let MiddleB(InnerA { value: a } , InnerB(42.19)) = b -%}
    {# Extra whitespace before comma intentional for coverage -#}
    b.a: {{ a }}
{%- endif -%}
"#)]
struct Outer {
    a: MiddleA<usize, f64>,
    b: MiddleB<usize, f64>,
}
impl ::std::fmt::Display for Outer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string =
            {
                use ::std::fmt::Write;
                let mut string = String::with_capacity(6usize);
                let f = &mut string;
                if let MiddleA { a: InnerA { value: 42 }, b: InnerB(b) } =
                        self.a {
                    f.write_str("a.b: ")?;
                    f.write_str(&::std::string::ToString::to_string(&(b)))?;
                    ;
                } else if let MiddleB(InnerA { value: a }, InnerB(42.19)) =
                        self.b {
                    f.write_str("b.a: ")?;
                    f.write_str(&::std::string::ToString::to_string(&(a)))?;
                    ;
                }
                string
            };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "nested"]
#[doc(hidden)]
pub const nested: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("nested"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "oxiplate-derive/tests/match.rs",
            start_line: 151usize,
            start_col: 4usize,
            end_line: 151usize,
            end_col: 10usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] ||
                test::assert_test_result(nested())),
    };
fn nested() {
    macro_rules! a {
        ($a:literal, $b:literal) =>
        { MiddleA { a: InnerA { value: $a }, b: InnerB($b), } };
    }
    macro_rules! b {
        ($a:literal, $b:literal) =>
        { MiddleB(InnerA { value: $a }, InnerB($b)) };
    }
    macro_rules! outer {
        ($aa:literal, $ab:literal, $ba:literal, $bb:literal) =>
        { Outer { a: a!($aa, $ab), b: b!($ba, $bb), } };
    }
    match (&::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                Outer {
                                    a: MiddleA { a: InnerA { value: 42 }, b: InnerB(19.89) },
                                    b: MiddleB(InnerA { value: 89 }, InnerB(42.19)),
                                }))
                    }), &"a.b: 19.89") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                Outer {
                                    a: MiddleA { a: InnerA { value: 64 }, b: InnerB(19.89) },
                                    b: MiddleB(InnerA { value: 89 }, InnerB(42.19)),
                                }))
                    }), &"b.a: 89") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                Outer {
                                    a: MiddleA { a: InnerA { value: 64 }, b: InnerB(19.89) },
                                    b: MiddleB(InnerA { value: 89 }, InnerB(16.19)),
                                }))
                    }), &"") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
}
#[oxiplate_inline(r#"
{%- match value %}
    {%- case ..1 %}To 1
    {%- case ..=1 %}Through 1
    {%- case 2 %}2
    {%- case 3..4 %}3 to 4
    {%- case 3..=4 %}3 through 4
    {%- case 3.. %}3 and up
{%- endmatch -%}
"#)]
struct RangeInteger {
    value: isize,
}
impl ::std::fmt::Display for RangeInteger {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string =
            {
                use ::std::fmt::Write;
                let mut string = String::with_capacity(1usize);
                let f = &mut string;
                match self.value {
                    ..1 => { f.write_str("To 1")?; }
                    ..=1 => { f.write_str("Through 1")?; }
                    2 => { f.write_str("2")?; }
                    3..4 => { f.write_str("3 to 4")?; }
                    3..=4 => { f.write_str("3 through 4")?; }
                    3.. => { f.write_str("3 and up")?; }
                }
                string
            };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "range_integer"]
#[doc(hidden)]
pub const range_integer: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("range_integer"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "oxiplate-derive/tests/match.rs",
            start_line: 196usize,
            start_col: 4usize,
            end_line: 196usize,
            end_col: 17usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] ||
                test::assert_test_result(range_integer())),
    };
fn range_integer() {
    match (&"To 1",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeInteger { value: 0 }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"Through 1",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeInteger { value: 1 }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"2",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeInteger { value: 2 }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"3 to 4",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeInteger { value: 3 }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"3 through 4",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeInteger { value: 4 }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"3 and up",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeInteger { value: 5 }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
}
#[oxiplate_inline(r#"
{%- match value %}
    {%- case ..1. %}To 1
    {%- case ..=1. %}Through 1
    {%- case 2.0 %}2
    {%- case 3. ..4. %}3 to 4
    {%- case 3. ..=4. %}3 through 4
    {%- case 3. .. %}3 and up
    {%- case _ %}Something else
{%- endmatch -%}
"#)]
struct RangeFloat {
    value: f64,
}
impl ::std::fmt::Display for RangeFloat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string =
            {
                use ::std::fmt::Write;
                let mut string = String::with_capacity(1usize);
                let f = &mut string;
                match self.value {
                    ..1. => { f.write_str("To 1")?; }
                    ..=1. => { f.write_str("Through 1")?; }
                    2.0 => { f.write_str("2")?; }
                    3...4. => { f.write_str("3 to 4")?; }
                    3...=4. => { f.write_str("3 through 4")?; }
                    3... => { f.write_str("3 and up")?; }
                    _ => { f.write_str("Something else")?; }
                }
                string
            };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "range_float"]
#[doc(hidden)]
pub const range_float: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("range_float"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "oxiplate-derive/tests/match.rs",
            start_line: 224usize,
            start_col: 4usize,
            end_line: 224usize,
            end_col: 15usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] ||
                test::assert_test_result(range_float())),
    };
fn range_float() {
    match (&"To 1",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeFloat { value: 0. }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"Through 1",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeFloat { value: 1. }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"2",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeFloat { value: 2. }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"Something else",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeFloat { value: 2.19 }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"3 to 4",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeFloat { value: 3. }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"3 through 4",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeFloat { value: 4. }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"3 and up",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeFloat { value: 5. }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
}
#[oxiplate_inline(r#"
{%- match value %}
    {%- case ..'b' %}To b
    {%- case ..='b' %}Through b
    {%- case 'c' %}c
    {%- case 'd'..'e' %}d to e
    {%- case 'd'..='e' %}d through e
    {%- case 'd'.. %}d and up
{%- endmatch -%}"#)]
struct RangeChar {
    value: char,
}
impl ::std::fmt::Display for RangeChar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string =
            {
                use ::std::fmt::Write;
                let mut string = String::with_capacity(1usize);
                let f = &mut string;
                match self.value {
                    ..'b' => { f.write_str("To b")?; }
                    ..='b' => { f.write_str("Through b")?; }
                    'c' => { f.write_str("c")?; }
                    'd'..'e' => { f.write_str("d to e")?; }
                    'd'..='e' => { f.write_str("d through e")?; }
                    'd'.. => { f.write_str("d and up")?; }
                }
                string
            };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "range_char"]
#[doc(hidden)]
pub const range_char: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("range_char"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "oxiplate-derive/tests/match.rs",
            start_line: 251usize,
            start_col: 4usize,
            end_line: 251usize,
            end_col: 14usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] ||
                test::assert_test_result(range_char())),
    };
fn range_char() {
    match (&"To b",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeChar { value: 'a' }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"Through b",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeChar { value: 'b' }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"c",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeChar { value: 'c' }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"d to e",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeChar { value: 'd' }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"d through e",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeChar { value: 'e' }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"d and up",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                RangeChar { value: 'f' }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
}
#[oxiplate_inline(r#"
{%- match value %}
    {%- case 19 %}The best number
    {%- case 42 %}The answer
    {%- case 69 | 420 %}Internet number
    {%- case _ %}Boring number
{%- endmatch -%}
"#)]
struct MultipleCases {
    value: usize,
}
impl ::std::fmt::Display for MultipleCases {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string =
            {
                use ::std::fmt::Write;
                let mut string = String::with_capacity(10usize);
                let f = &mut string;
                match self.value {
                    19 => { f.write_str("The best number")?; }
                    42 => { f.write_str("The answer")?; }
                    69 | 420 => { f.write_str("Internet number")?; }
                    _ => { f.write_str("Boring number")?; }
                }
                string
            };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "multiple_cases"]
#[doc(hidden)]
pub const multiple_cases: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("multiple_cases"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "oxiplate-derive/tests/match.rs",
            start_line: 276usize,
            start_col: 4usize,
            end_line: 276usize,
            end_col: 18usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] ||
                test::assert_test_result(multiple_cases())),
    };
fn multiple_cases() {
    match (&"The best number",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                MultipleCases { value: 19 }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"The answer",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                MultipleCases { value: 42 }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"Internet number",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                MultipleCases { value: 69 }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"Internet number",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                MultipleCases { value: 420 }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
    match (&"Boring number",
            &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}",
                                MultipleCases { value: 794 }))
                    })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&multiple_cases, &nested, &range_char,
                    &range_float, &range_integer, &test_count, &test_count_name,
                    &test_multiple, &test_name, &test_none])
}
