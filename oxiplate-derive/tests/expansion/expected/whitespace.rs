#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2024::*;
extern crate alloc;
use alloc::format;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline("Hello  \t\n {_} \r\n\t wor{-}ld \n\t {-} \t\n !")]
struct AdjustedWhitespace {}
impl ::core::fmt::Display for AdjustedWhitespace {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(12usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter.write_str("Hello world!")?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "adjusted_whitespace"]
#[doc(hidden)]
pub const adjusted_whitespace: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("adjusted_whitespace"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/whitespace.rs",
        start_line: 14usize,
        start_col: 4usize,
        end_line: 14usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(adjusted_whitespace()),
    ),
};
fn adjusted_whitespace() {
    let template = AdjustedWhitespace {};
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", template))
        }),
        &"Hello world!",
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
#[oxiplate_inline(
    "Hello  \t\t  \r\n\t {{_ username _}}  \t\t  \r\n\t (  \t\t  \r\n\t {{- name -}}  \t\t  \
     \r\n\t )!"
)]
struct WritWhitespaceControl {
    username: &'static str,
    name: &'static str,
}
impl ::core::fmt::Display for WritWhitespaceControl {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(12usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter.write_str("Hello ")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.username)))?;
            oxiplate_formatter.write_str(" (")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&(self.name)))?;
            oxiplate_formatter.write_str(")!")?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "writ_whitespace_control"]
#[doc(hidden)]
pub const writ_whitespace_control: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("writ_whitespace_control"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/whitespace.rs",
        start_line: 31usize,
        start_col: 4usize,
        end_line: 31usize,
        end_col: 27usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(writ_whitespace_control()),
    ),
};
fn writ_whitespace_control() {
    let template = WritWhitespaceControl {
        username: "dia",
        name: "Diana",
    };
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", template))
        }),
        &"Hello dia (Diana)!",
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
#[oxiplate_inline(
    "Hello  \t\t  \r\n\t {#_ Some cool comment _#}  \t\t  \r\n\t (  \t\t  \r\n\t {#- Hey another \
     comment -#}  \t\t  \r\n\t )!"
)]
struct CommentWhitespaceControl {}
impl ::core::fmt::Display for CommentWhitespaceControl {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(10usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter.write_str("Hello  ()!")?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "comment_whitespace_control"]
#[doc(hidden)]
pub const comment_whitespace_control: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("comment_whitespace_control"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/whitespace.rs",
        start_line: 48usize,
        start_col: 4usize,
        end_line: 48usize,
        end_col: 30usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(comment_whitespace_control()),
    ),
};
fn comment_whitespace_control() {
    let template = CommentWhitespaceControl {};
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", template))
        }),
        &"Hello  ()!",
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
#[oxiplate_inline(
    r#"
{{ "leave" }}  {{ "leave" }}
{{ "leave" }}  {{- "remove" }}
{{ "leave" }}  {{_ "replace" }}
{{ "removetag" }}{-}  {{ "leave" }}
{{ "leave" }}  {-}{{ "removetag" }}
{{ "replacetag" }}{_}  {{ "leave" }}
{{ "leave" }}  {_}{{ "replacetag" }}

{{ "remove" -}}  {{ "leave" }}
{{ "remove" -}}  {{- "remove" }}
{{ "removetag" }}{-}  {{- "remove" }}
{{ "remove" -}}  {-}{{ "removetag" }}

{{ "replace" _}}  {{ "leave" }}
{{ "replace" _}}  {{_ "replace" }}
{{ "replacetag" }}{_}  {{_ "replace" }}
{{ "replace" _}}  {_}{{ "replacetag" }}
"#
)]
struct AdjacentTags {}
impl ::core::fmt::Display for AdjacentTags {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(231usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("leave")))?;
            oxiplate_formatter.write_str("  ")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("leave")))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("leave")))?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("remove")))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("leave")))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("replace")))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("removetag")))?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("leave")))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("leave")))?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("removetag")))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("replacetag")))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("leave")))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("leave")))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("replacetag")))?;
            oxiplate_formatter.write_str("\n\n")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("remove")))?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("leave")))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("remove")))?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("remove")))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("removetag")))?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("remove")))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("remove")))?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("removetag")))?;
            oxiplate_formatter.write_str("\n\n")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("replace")))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("leave")))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("replace")))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("replace")))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("replacetag")))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("replace")))?;
            oxiplate_formatter.write_str("\n")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("replace")))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&alloc::string::ToString::to_string(&("replacetag")))?;
            oxiplate_formatter.write_str("\n")?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "adjacent_tags"]
#[doc(hidden)]
pub const adjacent_tags: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("adjacent_tags"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/whitespace.rs",
        start_line: 80usize,
        start_col: 4usize,
        end_line: 80usize,
        end_col: 17usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(adjacent_tags()),
    ),
};
fn adjacent_tags() {
    let template = AdjacentTags {};
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", template))
        }),
        &"
leave  leave
leaveremove
leave replace
removetagleave
leaveremovetag
replacetag leave
leave replacetag

removeleave
removeremove
removetagremove
removeremovetag

replace leave
replace replace
replacetag replace
replace replacetag
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
    };
}
#[oxiplate_inline(" \t\r\n{")]
struct WhitespaceOnly;
impl ::core::fmt::Display for WhitespaceOnly {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        let string = {
            extern crate alloc;
            use ::core::fmt::Write;
            let mut string = alloc::string::String::with_capacity(5usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter.write_str(" \t\r\n{")?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "whitespace_only"]
#[doc(hidden)]
pub const whitespace_only: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("whitespace_only"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/whitespace.rs",
        start_line: 113usize,
        start_col: 4usize,
        end_line: 113usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(whitespace_only()),
    ),
};
fn whitespace_only() {
    match (
        &" \t\r\n{",
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", WhitespaceOnly))
        }),
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
    test::test_main_static(
        &[
            &adjacent_tags,
            &adjusted_whitespace,
            &comment_whitespace_control,
            &whitespace_only,
            &writ_whitespace_control,
        ],
    )
}
