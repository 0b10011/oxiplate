use alloc::format;

use oxiplate_derive::Oxiplate;
#[oxiplate_inline("Hello  \t\n {_} \r\n\t wor{-}ld \n\t {-} \t\n !")]
struct AdjustedWhitespace {}
impl ::core::fmt::Display for AdjustedWhitespace {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str("Hello world!")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "whitespace::whitespace::adjusted_whitespace"]
#[doc(hidden)]
pub static adjusted_whitespace: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("whitespace::whitespace::adjusted_whitespace"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/whitespace/whitespace.rs",
        start_line: 10usize,
        start_col: 4usize,
        end_line: 10usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(adjusted_whitespace()),
    ),
};
fn adjusted_whitespace() {
    let template = AdjustedWhitespace {};
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", template)) }),
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
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str("Hello ")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.username)))?;
        oxiplate_formatter.write_str(" (")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&(self.name)))?;
        oxiplate_formatter.write_str(")!")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "whitespace::whitespace::writ_whitespace_control"]
#[doc(hidden)]
pub static writ_whitespace_control: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("whitespace::whitespace::writ_whitespace_control"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/whitespace/whitespace.rs",
        start_line: 27usize,
        start_col: 4usize,
        end_line: 27usize,
        end_col: 27usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
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
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", template)) }),
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
        }
    };
}
#[oxiplate_inline(
    "Hello  \t\t  \r\n\t {#_ Some cool comment _#}  \t\t  \r\n\t (  \t\t  \r\n\t {#- Hey another \
     comment -#}  \t\t  \r\n\t )!"
)]
struct CommentWhitespaceControl {}
impl ::core::fmt::Display for CommentWhitespaceControl {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str("Hello  ()!")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "whitespace::whitespace::comment_whitespace_control"]
#[doc(hidden)]
pub static comment_whitespace_control: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("whitespace::whitespace::comment_whitespace_control"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/whitespace/whitespace.rs",
        start_line: 44usize,
        start_col: 4usize,
        end_line: 44usize,
        end_col: 30usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(comment_whitespace_control()),
    ),
};
fn comment_whitespace_control() {
    let template = CommentWhitespaceControl {};
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", template)) }),
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
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str("\n")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("leave")))?;
        oxiplate_formatter.write_str("  ")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("leave")))?;
        oxiplate_formatter.write_str("\n")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("leave")))?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("remove")))?;
        oxiplate_formatter.write_str("\n")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("leave")))?;
        oxiplate_formatter.write_str(" ")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("replace")))?;
        oxiplate_formatter.write_str("\n")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("removetag")))?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("leave")))?;
        oxiplate_formatter.write_str("\n")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("leave")))?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("removetag")))?;
        oxiplate_formatter.write_str("\n")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("replacetag")))?;
        oxiplate_formatter.write_str(" ")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("leave")))?;
        oxiplate_formatter.write_str("\n")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("leave")))?;
        oxiplate_formatter.write_str(" ")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("replacetag")))?;
        oxiplate_formatter.write_str("\n\n")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("remove")))?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("leave")))?;
        oxiplate_formatter.write_str("\n")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("remove")))?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("remove")))?;
        oxiplate_formatter.write_str("\n")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("removetag")))?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("remove")))?;
        oxiplate_formatter.write_str("\n")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("remove")))?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("removetag")))?;
        oxiplate_formatter.write_str("\n\n")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("replace")))?;
        oxiplate_formatter.write_str(" ")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("leave")))?;
        oxiplate_formatter.write_str("\n")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("replace")))?;
        oxiplate_formatter.write_str(" ")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("replace")))?;
        oxiplate_formatter.write_str("\n")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("replacetag")))?;
        oxiplate_formatter.write_str(" ")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("replace")))?;
        oxiplate_formatter.write_str("\n")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("replace")))?;
        oxiplate_formatter.write_str(" ")?;
        oxiplate_formatter.write_str(&alloc::string::ToString::to_string(&("replacetag")))?;
        oxiplate_formatter.write_str("\n")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "whitespace::whitespace::adjacent_tags"]
#[doc(hidden)]
pub static adjacent_tags: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("whitespace::whitespace::adjacent_tags"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/whitespace/whitespace.rs",
        start_line: 76usize,
        start_col: 4usize,
        end_line: 76usize,
        end_col: 17usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(adjacent_tags()),
    ),
};
fn adjacent_tags() {
    let template = AdjacentTags {};
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", template)) }),
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
        }
    };
}
#[oxiplate_inline(" \t\r\n{")]
struct WhitespaceOnly;
impl ::core::fmt::Display for WhitespaceOnly {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        oxiplate_formatter.write_str(" \t\r\n{")?;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "whitespace::whitespace::whitespace_only"]
#[doc(hidden)]
pub static whitespace_only: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("whitespace::whitespace::whitespace_only"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/whitespace/whitespace.rs",
        start_line: 109usize,
        start_col: 4usize,
        end_line: 109usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(whitespace_only()),
    ),
};
fn whitespace_only() {
    {
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
        }
    };
}
