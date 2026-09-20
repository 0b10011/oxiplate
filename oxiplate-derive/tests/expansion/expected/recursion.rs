#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use oxiplate_derive::Oxiplate;
extern crate test;
#[rustc_test_marker = "recursion"]
#[doc(hidden)]
pub static recursion: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("recursion"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/recursion.rs",
        start_line: 10usize,
        start_col: 4usize,
        end_line: 10usize,
        end_col: 13usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(recursion())),
};
fn recursion() {
    #[oxiplate_inline(
        r#"{{name}} ({% for child in children %}{{ child }}, {% endfor %})"#
    )]
    struct Person<'a> {
        name: &'a str,
        children: &'a [Person<'a>],
    }
    impl<'a> ::core::fmt::Display for Person<'a> {
        fn fmt(
            &self,
            oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.name)))?;
            oxiplate_formatter.write_str(" (")?;
            for child in self.children {
                oxiplate_formatter
                    .write_str(&alloc::string::ToString::to_string(&(child)))?;
                oxiplate_formatter.write_str(", ")?;
            }
            oxiplate_formatter.write_str(")")?;
            ::core::result::Result::Ok(())
        }
    }
    let jane = Person {
        name: "Jane",
        children: &[],
    };
    let john = Person {
        name: "John",
        children: &[jane],
    };
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!(
                        "{0}",
                        Person {
                            name: "Jen",
                            children: &[john],
                        },
                    ),
                )
            }),
            &"Jen (John (Jane (), ), )",
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
extern crate test;
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> test::ExitCode {
    test::test_main_env_args(&[&recursion])
}
