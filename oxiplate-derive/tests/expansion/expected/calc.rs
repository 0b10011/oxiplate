#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline(
    "{-}
1 + 2 = {{ 1 + 2 }}
{{ max }} + {{ min }} = {{ max + min }}
{{ max }} - {{ min }} = {{ max - min }}
{{ max }} * {{ min }} = {{ max * min }}
{{ max }} / {{ min }} = {{ max / min }}
{{ max }} % {{ min }} = {{ max % min }}
{{ min }} + {{ min }} * {{ max }} = {{ min + min * max }}
{{ max }} + {{ max }} / {{ min }} = {{ max + max / min }}
{{ max }} - {{ min }} % {{ min }} = {{ max - min % min }}
{{ a }} - {{ b }} * {{ c }} = {{ a - b * c }}
{{ a }} / {{ b }} + {{ c }} = {{ a / b + c }}"
)]
struct Math {
    min: i16,
    max: i16,
    a: usize,
    b: usize,
    c: usize,
}
impl ::std::fmt::Display for Math {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(129usize);
            let f = &mut string;
            f.write_str("1 + 2 = ")?;
            f.write_str(&::std::string::ToString::to_string(&(1 + 2)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max)))?;
            f.write_str(" + ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.min)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max + self.min)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max)))?;
            f.write_str(" - ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.min)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max - self.min)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max)))?;
            f.write_str(" * ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.min)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max * self.min)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max)))?;
            f.write_str(" / ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.min)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max / self.min)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max)))?;
            f.write_str(" % ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.min)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max % self.min)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.min)))?;
            f.write_str(" + ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.min)))?;
            f.write_str(" * ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max)))?;
            f.write_str(" = ")?;
            f.write_str(
                &::std::string::ToString::to_string(&(self.min + self.min * self.max)),
            )?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max)))?;
            f.write_str(" + ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max)))?;
            f.write_str(" / ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.min)))?;
            f.write_str(" = ")?;
            f.write_str(
                &::std::string::ToString::to_string(&(self.max + self.max / self.min)),
            )?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max)))?;
            f.write_str(" - ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.min)))?;
            f.write_str(" % ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.min)))?;
            f.write_str(" = ")?;
            f.write_str(
                &::std::string::ToString::to_string(&(self.max - self.min % self.min)),
            )?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.a)))?;
            f.write_str(" - ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.b)))?;
            f.write_str(" * ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.c)))?;
            f.write_str(" = ")?;
            f.write_str(
                &::std::string::ToString::to_string(&(self.a - self.b * self.c)),
            )?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.a)))?;
            f.write_str(" / ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.b)))?;
            f.write_str(" + ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.c)))?;
            f.write_str(" = ")?;
            f.write_str(
                &::std::string::ToString::to_string(&(self.a / self.b + self.c)),
            )?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "test_math"]
#[doc(hidden)]
pub const test_math: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_math"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/calc.rs",
        start_line: 27usize,
        start_col: 4usize,
        end_line: 27usize,
        end_col: 13usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test_math())),
};
fn test_math() {
    let data = Math {
        min: 19,
        max: 89,
        a: 16,
        b: 8,
        c: 2,
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"1 + 2 = 3
89 + 19 = 108
89 - 19 = 70
89 * 19 = 1691
89 / 19 = 4
89 % 19 = 13
19 + 19 * 89 = 1710
89 + 89 / 19 = 93
89 - 19 % 19 = 89
16 - 8 * 2 = 0
16 / 8 + 2 = 4",
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
    "{-}
{{ max }} == {{ min }} = {{ max == min }}
{{ max }} != {{ min }} = {{ max != min }}
{{ max }} > {{ min }} = {{ max > min }}
{{ max }} < {{ min }} = {{ max < min }}
{{ max }} >= {{ min }} = {{ max >= min }}
{{ max }} <= {{ min }} = {{ max <= min }}"
)]
struct Comparisons {
    min: i16,
    max: i16,
}
impl ::std::fmt::Display for Comparisons {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(63usize);
            let f = &mut string;
            f.write_str(&::std::string::ToString::to_string(&(self.max)))?;
            f.write_str(" == ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.min)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max == self.min)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max)))?;
            f.write_str(" != ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.min)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max != self.min)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max)))?;
            f.write_str(" > ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.min)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max > self.min)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max)))?;
            f.write_str(" < ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.min)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max < self.min)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max)))?;
            f.write_str(" >= ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.min)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max >= self.min)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max)))?;
            f.write_str(" <= ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.min)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.max <= self.min)))?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "test_comparisons"]
#[doc(hidden)]
pub const test_comparisons: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_comparisons"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/calc.rs",
        start_line: 68usize,
        start_col: 4usize,
        end_line: 68usize,
        end_col: 20usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_comparisons()),
    ),
};
fn test_comparisons() {
    let data = Comparisons { min: 19, max: 89 };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"89 == 19 = false
89 != 19 = true
89 > 19 = true
89 < 19 = false
89 >= 19 = true
89 <= 19 = false",
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
    "{-}
{{ yes }} || {{ yes }} = {{ yes || yes2 }}
{{ yes }} || {{ no }} = {{ yes || no }}
{{ no }} || {{ yes }} = {{ no || yes }}
{{ no }} || {{ no }} = {{ no || no2 }}
{{ yes }} && {{ yes }} = {{ yes && yes2 }}
{{ yes }} && {{ no }} = {{ yes && no }}
{{ no }} && {{ yes }} = {{ no && yes }}
{{ no }} && {{ no }} = {{ no && no2 }}
{{ yes }} || {{ no }} && {{ no }} = {{ yes || no && no2 }}
{{ no }} || {{ yes }} && {{ no }} = {{ no || yes && no2 }}
{{ no }} || {{ yes }} && {{ yes }} = {{ no || yes && yes2 }}"
)]
#[allow(clippy::struct_excessive_bools)]
struct OrAnd {
    yes: bool,
    yes2: bool,
    no: bool,
    no2: bool,
}
impl ::std::fmt::Display for OrAnd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(135usize);
            let f = &mut string;
            f.write_str(&::std::string::ToString::to_string(&(self.yes)))?;
            f.write_str(" || ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.yes)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.yes || self.yes2)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.yes)))?;
            f.write_str(" || ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.no)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.yes || self.no)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.no)))?;
            f.write_str(" || ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.yes)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.no || self.yes)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.no)))?;
            f.write_str(" || ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.no)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.no || self.no2)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.yes)))?;
            f.write_str(" && ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.yes)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.yes && self.yes2)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.yes)))?;
            f.write_str(" && ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.no)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.yes && self.no)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.no)))?;
            f.write_str(" && ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.yes)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.no && self.yes)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.no)))?;
            f.write_str(" && ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.no)))?;
            f.write_str(" = ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.no && self.no2)))?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.yes)))?;
            f.write_str(" || ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.no)))?;
            f.write_str(" && ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.no)))?;
            f.write_str(" = ")?;
            f.write_str(
                &::std::string::ToString::to_string(&(self.yes || self.no && self.no2)),
            )?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.no)))?;
            f.write_str(" || ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.yes)))?;
            f.write_str(" && ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.no)))?;
            f.write_str(" = ")?;
            f.write_str(
                &::std::string::ToString::to_string(&(self.no || self.yes && self.no2)),
            )?;
            f.write_str("\n")?;
            f.write_str(&::std::string::ToString::to_string(&(self.no)))?;
            f.write_str(" || ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.yes)))?;
            f.write_str(" && ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.yes)))?;
            f.write_str(" = ")?;
            f.write_str(
                &::std::string::ToString::to_string(&(self.no || self.yes && self.yes2)),
            )?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "test_or_and"]
#[doc(hidden)]
pub const test_or_and: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_or_and"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/calc.rs",
        start_line: 106usize,
        start_col: 4usize,
        end_line: 106usize,
        end_col: 15usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_or_and()),
    ),
};
fn test_or_and() {
    let data = OrAnd {
        yes: true,
        yes2: true,
        no: false,
        no2: false,
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"true || true = true
true || false = true
false || true = true
false || false = false
true && true = true
true && false = false
false && true = false
false && false = false
true || false && false = true
false || true && false = false
false || true && true = true",
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
{%- if a - b < c + b -%}
    {{ a - b }} < {{ c + b }}
{%- else -%}
    {{ a - b }} > {{ c + b }}
{%- endif -%}
"
)]
#[allow(clippy::struct_excessive_bools)]
struct OrderOfOperations {
    a: usize,
    b: usize,
    c: usize,
}
impl ::std::fmt::Display for OrderOfOperations {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(5usize);
            let f = &mut string;
            if self.a - self.b < self.c + self.b {
                f.write_str(&::std::string::ToString::to_string(&(self.a - self.b)))?;
                f.write_str(" < ")?;
                f.write_str(&::std::string::ToString::to_string(&(self.c + self.b)))?;
            } else {
                f.write_str(&::std::string::ToString::to_string(&(self.a - self.b)))?;
                f.write_str(" > ")?;
                f.write_str(&::std::string::ToString::to_string(&(self.c + self.b)))?;
            }
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "test_order_of_operations"]
#[doc(hidden)]
pub const test_order_of_operations: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_order_of_operations"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/calc.rs",
        start_line: 148usize,
        start_col: 4usize,
        end_line: 148usize,
        end_col: 28usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_order_of_operations()),
    ),
};
fn test_order_of_operations() {
    let data = OrderOfOperations {
        a: 16,
        b: 8,
        c: 2,
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"8 < 10",
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
        &[&test_comparisons, &test_math, &test_or_and, &test_order_of_operations],
    )
}
