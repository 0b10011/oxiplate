use alloc::format;

use oxiplate_derive::Oxiplate;
extern crate test;
#[rustc_test_marker = "expressions::array::test_assignment"]
#[doc(hidden)]
pub static test_assignment: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::array::test_assignment"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/array.rs",
        start_line: 6usize,
        start_col: 4usize,
        end_line: 6usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_assignment()),
    ),
};
fn test_assignment() {
    #[oxiplate_inline(r#"{%- let array = [ "a" ;  3 ] -%}{{ array.join("") }}"#)]
    struct Array;
    impl ::core::fmt::Display for Array {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            let array = ["a"; 3];
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(array.join(""))))?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Array)) }),
            &"aaa",
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
#[rustc_test_marker = "expressions::array::test_array_of_string_slices_in_writ"]
#[doc(hidden)]
pub static test_array_of_string_slices_in_writ: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::array::test_array_of_string_slices_in_writ"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/array.rs",
        start_line: 15usize,
        start_col: 4usize,
        end_line: 15usize,
        end_col: 39usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_array_of_string_slices_in_writ()),
    ),
};
fn test_array_of_string_slices_in_writ() {
    #[oxiplate_inline(r#"[{{ ["a", "b", "c"].join("") }}]"#)]
    struct Array;
    impl ::core::fmt::Display for Array {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("[")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(["a", "b", "c"].join("")),
            ))?;
            oxiplate_formatter.write_str("]")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Array)) }),
            &"[abc]",
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
#[rustc_test_marker = "expressions::array::test_array_of_string_slices_in_writ_trailing_comma"]
#[doc(hidden)]
pub static test_array_of_string_slices_in_writ_trailing_comma: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "expressions::array::test_array_of_string_slices_in_writ_trailing_comma",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "oxiplate-derive/tests/integration/expressions/array.rs",
            start_line: 24usize,
            start_col: 4usize,
            end_line: 24usize,
            end_col: 54usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_array_of_string_slices_in_writ_trailing_comma()),
        ),
    };
fn test_array_of_string_slices_in_writ_trailing_comma() {
    #[oxiplate_inline(r#"[{{ ["a", "b", "c",].join("") }}]"#)]
    struct Array;
    impl ::core::fmt::Display for Array {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("[")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(["a", "b", "c"].join("")),
            ))?;
            oxiplate_formatter.write_str("]")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Array)) }),
            &"[abc]",
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
#[rustc_test_marker = "expressions::array::test_array_of_numbers_in_for_loop"]
#[doc(hidden)]
pub static test_array_of_numbers_in_for_loop: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::array::test_array_of_numbers_in_for_loop"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/array.rs",
        start_line: 33usize,
        start_col: 4usize,
        end_line: 33usize,
        end_col: 37usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_array_of_numbers_in_for_loop()),
    ),
};
fn test_array_of_numbers_in_for_loop() {
    #[oxiplate_inline(
        r#"[
            {%- for elem in [1, 2, 3] -%}
                {{ elem }}
            {%- endfor -%}
        ]"#
    )]
    struct Array;
    impl ::core::fmt::Display for Array {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("[")?;
            for elem in [1, 2, 3] {
                oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(elem)))?;
            }
            oxiplate_formatter.write_str("]")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Array)) }),
            &"[123]",
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
#[rustc_test_marker = "expressions::array::test_array_of_expressions"]
#[doc(hidden)]
pub static test_array_of_expressions: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::array::test_array_of_expressions"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/array.rs",
        start_line: 48usize,
        start_col: 4usize,
        end_line: 48usize,
        end_col: 29usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_array_of_expressions()),
    ),
};
fn test_array_of_expressions() {
    #[oxiplate_inline(r#"[{{ ["a", "b", &"c".to_uppercase()].join("") }}]"#)]
    struct Array;
    impl ::core::fmt::Display for Array {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("[")?;
            oxiplate_formatter.write_str(&alloc::string::ToString::to_string(
                &(["a", "b", &"c".to_uppercase()].join("")),
            ))?;
            oxiplate_formatter.write_str("]")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Array)) }),
            &"[abC]",
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
#[rustc_test_marker = "expressions::array::test_repeat"]
#[doc(hidden)]
pub static test_repeat: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::array::test_repeat"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/array.rs",
        start_line: 57usize,
        start_col: 4usize,
        end_line: 57usize,
        end_col: 15usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_repeat()),
    ),
};
fn test_repeat() {
    #[oxiplate_inline(r#"[{{ ["a"; 3].join("") }}]"#)]
    struct Array;
    impl ::core::fmt::Display for Array {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("[")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(["a"; 3].join(""))))?;
            oxiplate_formatter.write_str("]")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Array)) }),
            &"[aaa]",
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
#[rustc_test_marker = "expressions::array::test_repeat_empty"]
#[doc(hidden)]
pub static test_repeat_empty: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("expressions::array::test_repeat_empty"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/expressions/array.rs",
        start_line: 66usize,
        start_col: 4usize,
        end_line: 66usize,
        end_col: 21usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_repeat_empty()),
    ),
};
fn test_repeat_empty() {
    #[oxiplate_inline(r#"[{{ ["a"; 0].join("") }}]"#)]
    struct Array;
    impl ::core::fmt::Display for Array {
        fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            extern crate alloc;
            use ::core::fmt::Write as _;
            oxiplate_formatter.write_str("[")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(["a"; 0].join(""))))?;
            oxiplate_formatter.write_str("]")?;
            ::core::result::Result::Ok(())
        }
    }
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Array)) }),
            &"[]",
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
