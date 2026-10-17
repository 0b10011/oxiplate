use alloc::format;

use oxiplate_derive::Oxiplate;
#[oxiplate = "./extends-deep-skipped-block.html.oxip"]
struct AbsoluteData {
    title: &'static str,
    message: &'static str,
}
impl ::core::fmt::Display for AbsoluteData {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str("<!DOCTYPE html>\n<title>")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.title)))?;
        oxiplate_formatter.write_str("</title>\n")?;
        {
            oxiplate_formatter.write_str("<h2>")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.title)))?;
            oxiplate_formatter.write_str("</h2>\n  <div>")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.message)))?;
            oxiplate_formatter.write_str("</div>")?;
        }
        {}
        oxiplate_formatter.write_str("\n")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "external::extends_nested_skipped_block::absolute"]
#[doc(hidden)]
pub static absolute: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("external::extends_nested_skipped_block::absolute"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/external/extends_nested_skipped_block.rs",
        start_line: 13usize,
        start_col: 4usize,
        end_line: 13usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(absolute()),
    ),
};
fn absolute() {
    let data = AbsoluteData {
        title: "Oxiplate Example",
        message: "Hello world!",
    };
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", data)) }),
            &"<!DOCTYPE html>\n<title>Oxiplate Example</title>\n<h2>Oxiplate Example</h2>\n  \
              <div>Hello world!</div>\n",
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
    };
}
