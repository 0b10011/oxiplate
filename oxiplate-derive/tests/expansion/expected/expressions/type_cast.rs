use alloc::format;
use alloc::string::String;
use core::fmt::Display;

use oxiplate_derive::Oxiplate;
extern crate test;
#[rustc_test_marker = "expressions::type_cast::as_u64"]
#[doc(hidden)]
pub static as_u64: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::type_cast::as_u64"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/type_cast.rs",
        start_line: 8usize,
        start_col: 4usize,
        end_line: 8usize,
        end_col: 10usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(as_u64()),
    ),
};
fn as_u64() {
    #[oxiplate_inline("{{ unsigned_16 as u64 + unsigned_64 }}")]
    struct Data {
        unsigned_16: u16,
        unsigned_64: u64,
    }
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(self.unsigned_16 as u64 + self.unsigned_64),
            ))?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!(
                    "{0}",
                    Data {
                        unsigned_16: 10_u16,
                        unsigned_64: 9_u64
                    }
                ))
            }),
            &"19",
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
#[rustc_test_marker = "expressions::type_cast::as_trait"]
#[doc(hidden)]
pub static as_trait: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::type_cast::as_trait"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/type_cast.rs",
        start_line: 29usize,
        start_col: 4usize,
        end_line: 29usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(as_trait()),
    ),
};
fn as_trait() {
    struct Greeting {
        message: &'static str,
    }
    impl Display for Greeting {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str(self.message)
        }
    }
    trait Yell {
        fn yell(&self) -> String;
    }
    impl Yell for Greeting {
        fn yell(&self) -> String {
            self.message.to_uppercase()
        }
    }
    impl Display for dyn Yell {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str(&self.yell())
        }
    }
    #[oxiplate_inline(r#"{{ "Greeting: " ~ &greeting }}"#)]
    struct AsGreeting {
        greeting: Greeting,
    }
    impl ::core::fmt::Display for AsGreeting {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("Greeting: {0}", &self.greeting))
                })),
            ))?;
            ::core::result::Result::Ok(())
        }
    }
    #[oxiplate_inline(r#"{{ "Greeting: " ~ &greeting as &dyn Yell }}"#)]
    struct AsYell {
        greeting: Greeting,
    }
    impl ::core::fmt::Display for AsYell {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("Greeting: {0}", &self.greeting as &dyn Yell))
                })),
            ))?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!(
                    "{0}",
                    AsGreeting {
                        greeting: Greeting {
                            message: "Hello world!"
                        },
                    }
                ))
            }),
            &"Greeting: Hello world!",
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
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!(
                    "{0}",
                    AsYell {
                        greeting: Greeting {
                            message: "Hello world!"
                        }
                    }
                ))
            }),
            &"Greeting: HELLO WORLD!",
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
