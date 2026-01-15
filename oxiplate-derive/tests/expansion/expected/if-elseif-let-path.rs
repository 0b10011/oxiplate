#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
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
impl ::std::fmt::Display for Data {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(1usize);
            let oxiplate_formatter = &mut string;
            if self.check {
                oxiplate_formatter.write_str("bar")?;
            } else if let Type::Text(text) = self.ty {
                oxiplate_formatter
                    .write_str(&::std::string::ToString::to_string(&(text)))?;
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
        start_line: 23usize,
        start_col: 4usize,
        end_line: 23usize,
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
