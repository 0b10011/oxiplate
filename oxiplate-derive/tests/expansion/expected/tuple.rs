#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline("
{%- if let (a,) = (a,) -%}
    {{ a }}
{%- endif -%}
")]
struct Single {
    a: usize,
}
impl ::std::fmt::Display for Single {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(1usize);
            let oxiplate_formatter = &mut string;
            if let (a,) = (self.a,) {
                oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(a)))?;
            }
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "single"]
#[doc(hidden)]
pub const single: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("single"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/tuple.rs",
        start_line: 16usize,
        start_col: 4usize,
        end_line: 16usize,
        end_col: 10usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(single())),
};
fn single() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", Single { a: 9 }))
        }),
        &"9",
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
{%- if let (a, b) = (b, a) -%}
    {{ a }} + {{ b }} = {{ a + b -}}
{% endif -%}
"
)]
struct Double {
    a: usize,
    b: usize,
}
impl ::std::fmt::Display for Double {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(9usize);
            let oxiplate_formatter = &mut string;
            if let (a, b) = (self.b, self.a) {
                oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(a)))?;
                oxiplate_formatter.write_str(" + ")?;
                oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(b)))?;
                oxiplate_formatter.write_str(" = ")?;
                oxiplate_formatter
                    .write_str(&::std::string::ToString::to_string(&(a + b)))?;
            }
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "double"]
#[doc(hidden)]
pub const double: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("double"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/tuple.rs",
        start_line: 34usize,
        start_col: 4usize,
        end_line: 34usize,
        end_col: 10usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(double())),
};
fn double() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", Double { a: 10, b: 9 }))
        }),
        &"9 + 10 = 19",
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
{%- if let (a, b, c, d, e) = (e, d, c, b, a) -%}
    {{ a _}}
    {{ b _}}
    {{ c _}}
    {{ d _}}
    {{ e -}}
{% endif -%}
"
)]
struct Several {
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
}
impl ::std::fmt::Display for Several {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(9usize);
            let oxiplate_formatter = &mut string;
            if let (a, b, c, d, e) = (self.e, self.d, self.c, self.b, self.a) {
                oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(a)))?;
                oxiplate_formatter.write_str(" ")?;
                oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(b)))?;
                oxiplate_formatter.write_str(" ")?;
                oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(c)))?;
                oxiplate_formatter.write_str(" ")?;
                oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(d)))?;
                oxiplate_formatter.write_str(" ")?;
                oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(e)))?;
            }
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "several"]
#[doc(hidden)]
pub const several: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("several"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/tuple.rs",
        start_line: 59usize,
        start_col: 4usize,
        end_line: 59usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(several())),
};
fn several() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "{0}",
                    Several {
                        a: 5,
                        b: 4,
                        c: 3,
                        d: 2,
                        e: 1,
                    },
                ),
            )
        }),
        &"1 2 3 4 5",
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
    test::test_main_static(&[&double, &several, &single])
}
