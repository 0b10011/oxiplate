use alloc::format;
use alloc::string::String;
use core::fmt::Debug;

use oxiplate_derive::Oxiplate;
fn format<T: Debug>(value: T) -> String {
    ::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0:?}", value)) })
}
extern crate test;
#[rustc_test_marker = "generics::r#type::string"]
#[doc(hidden)]
pub static string: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("generics::r#type::string"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/generics/type.rs",
        start_line: 13usize,
        start_col: 4usize,
        end_line: 13usize,
        end_col: 10usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(string()),
    ),
};
#[allow(unused_parens)]
fn string() {
    #[oxiplate_inline(
        r#"
{{ format(Into::<String>::into(value)) }}
{{ format(Into::<&str>::into(value)) }}
{{ format(Into::<&'a str>::into(value)) }}
{{ format(Into::<(String)>::into(value)) }}
{{ format(Into::<alloc::string::String>::into(value)) }}
{{ format(Into::<::alloc::string::String>::into(value)) }}
"#
    )]
    struct Data<'a> {
        value: &'a str,
    }
    impl<'a> ::core::fmt::Display for Data<'a> {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(format(Into::<String>::into(self.value))),
            ))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(format(Into::<&str>::into(self.value))),
            ))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(format(Into::<&'a str>::into(self.value))),
            ))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(format(Into::<(String)>::into(self.value))),
            ))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(format(Into::<alloc::string::String>::into(self.value))),
            ))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(format(Into::<::alloc::string::String>::into(self.value))),
            ))?;
            oxiplate_formatter.write_str("\n")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", Data { value: "hello" }))
            }),
            &r#"
"hello"
"hello"
"hello"
"hello"
"hello"
"hello"
"#,
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
#[rustc_test_marker = "generics::r#type::cow"]
#[doc(hidden)]
pub static cow: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("generics::r#type::cow"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/generics/type.rs",
        start_line: 43usize,
        start_col: 4usize,
        end_line: 43usize,
        end_col: 7usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(cow()),
    ),
};
fn cow() {
    use alloc::borrow::Cow;
    #[oxiplate_inline(r#"{{ format(Into::<Cow<str>>::into(value)) }}"#)]
    struct Data {
        value: &'static str,
    }
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(format(Into::<Cow<str>>::into(self.value))),
            ))?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", Data { value: "hello" }))
            }),
            &r#""hello""#,
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
#[rustc_test_marker = "generics::r#type::integer"]
#[doc(hidden)]
pub static integer: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("generics::r#type::integer"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/generics/type.rs",
        start_line: 56usize,
        start_col: 4usize,
        end_line: 56usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(integer()),
    ),
};
fn integer() {
    use alloc::string::ToString;
    #[oxiplate_inline(
        r#"
{{ Into::<u64>::into(unsigned_32) + unsigned_64 }}
{{ unsigned_32.to_string().parse::<u64>().unwrap() + unsigned_64 }}
"#
    )]
    struct Data {
        unsigned_32: u32,
        unsigned_64: u64,
    }
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(Into::<u64>::into(self.unsigned_32) + self.unsigned_64),
            ))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(self.unsigned_32.to_string().parse::<u64>().unwrap() + self.unsigned_64),
            ))?;
            oxiplate_formatter.write_str("\n")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!(
                    "{0}",
                    Data {
                        unsigned_32: 10_u32,
                        unsigned_64: 9_u64
                    }
                ))
            }),
            &"
19
19
",
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
#[rustc_test_marker = "generics::r#type::inferred"]
#[doc(hidden)]
pub static inferred: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("generics::r#type::inferred"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/generics/type.rs",
        start_line: 87usize,
        start_col: 4usize,
        end_line: 87usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(inferred()),
    ),
};
fn inferred() {
    use alloc::vec::Vec;
    #[oxiplate_inline(r#"{% for i in Vec::<_>::from([19, 89]) %}{{ i }}{% endfor %}"#)]
    struct Data;
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            for i in Vec::<_>::from([19, 89]) {
                oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(i)))?;
            }
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Data)) }),
            &"1989",
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
#[rustc_test_marker = "generics::r#type::qualified_path"]
#[doc(hidden)]
pub static qualified_path: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("generics::r#type::qualified_path"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/generics/type.rs",
        start_line: 98usize,
        start_col: 4usize,
        end_line: 98usize,
        end_col: 18usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(qualified_path()),
    ),
};
fn qualified_path() {
    struct Foo;
    trait Bar {
        type Baz;
    }
    impl Bar for Foo {
        type Baz = u64;
    }
    #[oxiplate_inline(r#"{{ Into::<<Foo as Bar>::Baz>::into(unsigned_16) + unsigned_64 }}"#)]
    struct Data {
        unsigned_16: u16,
        unsigned_64: u64,
    }
    impl ::core::fmt::Display for Data {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(Into::<<Foo as Bar>::Baz>::into(self.unsigned_16) + self.unsigned_64),
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
