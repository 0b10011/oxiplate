#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate_derive::Oxiplate;
mod std {}
#[oxiplate_inline = "{{ foo }}"]
struct Data {
    foo: &'static str,
}
impl ::std::fmt::Display for Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_fmt(format_args!("{0}", self.foo))?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "overridden_std"]
#[doc(hidden)]
pub const overridden_std: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("overridden_std"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive\\tests\\overridden-std.rs",
        start_line: 12usize,
        start_col: 4usize,
        end_line: 12usize,
        end_col: 18usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(overridden_std()),
    ),
};
fn overridden_std() {
    let data = Data { foo: "Hello world!" };
    {
        ::std::io::_print(format_args!("{0}", data));
    };
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&overridden_std])
}
