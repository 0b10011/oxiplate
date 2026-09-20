#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use oxiplate_derive::Oxiplate;
mod std {}
#[oxiplate_inline("{{ foo }}")]
struct Data {
    foo: &'static str,
}
impl ::core::fmt::Display for Data {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.foo)))?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "overridden_std"]
#[doc(hidden)]
pub static overridden_std: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("overridden_std"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/overridden-std.rs",
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
extern crate test;
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> test::ExitCode {
    test::test_main_env_args(&[&overridden_std])
}
