#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate_derive::Oxiplate;
#[oxiplate = "./multiple-blocks-inner.html.oxip"]
struct Data;
impl ::std::fmt::Display for Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(81usize);
            let f = &mut string;
            f.write_str("<!DOCTYPE html>\n<header>")?;
            f.write_str("header")?;
            f.write_str("</header>\n<main>")?;
            f.write_str("main")?;
            f.write_str("</main>\n<footer>")?;
            f.write_str("footer")?;
            f.write_str("</footer>")?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "multiple_blocks"]
#[doc(hidden)]
pub const multiple_blocks: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("multiple_blocks"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/multiple-blocks.rs",
        start_line: 8usize,
        start_col: 4usize,
        end_line: 8usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(multiple_blocks()),
    ),
};
fn multiple_blocks() {
    let data = Data;
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"<!DOCTYPE html>\n<header>header</header>\n<main>main</main>\n<footer>footer</footer>",
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
    test::test_main_static(&[&multiple_blocks])
}
