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
}
#[oxiplate_inline(
    r"
{%- if check -%}
bar
{%- elseif let Type::Text(text) = ty -%}
{{ text }}
{%- endif -%}
"
)]
struct Data {
    check: bool,
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
            if self.check {
                oxiplate_formatter.write_str("bar")?;
            } else if let Type::Text(text) = self.ty {
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(text)))?;
            }
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "test"]
#[doc(hidden)]
pub const test: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/if-elseif-let-path.rs",
        start_line: 29usize,
        start_col: 4usize,
        end_line: 29usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(test())),
};
fn test() {
    let data = Data {
        check: false,
        ty: Type::Text("foo"),
    };
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
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test])
}
