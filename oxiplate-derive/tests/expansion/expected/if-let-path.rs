#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use oxiplate_derive::Oxiplate;
enum Type {
    Text(&'static str),
    Numbers(u8, u8),
}
#[oxiplate_inline(
    r"
{%- if let Type::Text(text) = ty -%}
{{ text }}
{%- elseif let Type::Numbers(left, right) = ty -%}
{{ left }} + {{ right }} = {{ left + right }}
{%- endif -%}
"
)]
struct Data {
    ty: Type,
}
impl ::core::fmt::Display for Data {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(1usize);
            let oxiplate_formatter = &mut string;
            if let Type::Text(text) = self.ty {
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(text)))?;
            } else if let Type::Numbers(left, right) = self.ty {
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(left)))?;
                oxiplate_formatter.write_str(" + ")?;
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(right)))?;
                oxiplate_formatter.write_str(" = ")?;
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(left + right)))?;
            }
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "text"]
#[doc(hidden)]
pub const text: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("text"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/if-let-path.rs",
        start_line: 29usize,
        start_col: 4usize,
        end_line: 29usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(text())),
};
fn text() {
    let data = Data { ty: Type::Text("foo") };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"foo",
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
#[rustc_test_marker = "numbers"]
#[doc(hidden)]
pub const numbers: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("numbers"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/if-let-path.rs",
        start_line: 38usize,
        start_col: 4usize,
        end_line: 38usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(numbers())),
};
fn numbers() {
    let data = Data { ty: Type::Numbers(10, 9) };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"10 + 9 = 19",
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
    test::test_main_static(&[&numbers, &text])
}
