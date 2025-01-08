#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline = "Hello  \t\n {_} \r\n\t wo{_}r{-}ld \n\t {-} \t\n !"]
struct AdjustedWhitespace {}
impl ::std::fmt::Display for AdjustedWhitespace {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("Hello world!")?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "adjusted_whitespace"]
#[doc(hidden)]
pub const adjusted_whitespace: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("adjusted_whitespace"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive\\tests\\whitespace.rs",
        start_line: 8usize,
        start_col: 4usize,
        end_line: 8usize,
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
            let res = ::alloc::fmt::format(format_args!("{0}", template));
            res
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
#[oxiplate_inline = "Hello  \t\t  \r\n\t {{_ username _}}  \t\t  \r\n\t (  \t\t  \r\n\t {{- name \
                     -}}  \t\t  \r\n\t )!"]
struct WritWhitespaceControl {
    username: &'static str,
    name: &'static str,
}
impl ::std::fmt::Display for WritWhitespaceControl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_fmt(format_args!("Hello {0} ({1})!", self.username, self.name))?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "writ_whitespace_control"]
#[doc(hidden)]
pub const writ_whitespace_control: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("writ_whitespace_control"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive\\tests\\whitespace.rs",
        start_line: 23usize,
        start_col: 4usize,
        end_line: 23usize,
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
            let res = ::alloc::fmt::format(format_args!("{0}", template));
            res
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
#[oxiplate_inline = "Hello @{{_ username _}}!"]
struct WritPreserveSpaceless {
    username: &'static str,
}
impl ::std::fmt::Display for WritPreserveSpaceless {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_fmt(format_args!("Hello @{0}!", self.username))?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "writ_preserve_spaceless"]
#[doc(hidden)]
pub const writ_preserve_spaceless: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("writ_preserve_spaceless"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive\\tests\\whitespace.rs",
        start_line: 39usize,
        start_col: 4usize,
        end_line: 39usize,
        end_col: 27usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(writ_preserve_spaceless()),
    ),
};
fn writ_preserve_spaceless() {
    let template = WritPreserveSpaceless {
        username: "dia",
    };
    match (
        &::alloc::__export::must_use({
            let res = ::alloc::fmt::format(format_args!("{0}", template));
            res
        }),
        &"Hello @dia!",
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
#[oxiplate_inline = "Hello  \t\t  \r\n\t {#_ Some cool comment _#}  \t\t  \r\n\t (  \t\t  \r\n\t \
                     {#- Hey another comment -#}  \t\t  \r\n\t )!"]
struct CommentWhitespaceControl {}
impl ::std::fmt::Display for CommentWhitespaceControl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("Hello  ()!")?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "comment_whitespace_control"]
#[doc(hidden)]
pub const comment_whitespace_control: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("comment_whitespace_control"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive\\tests\\whitespace.rs",
        start_line: 51usize,
        start_col: 4usize,
        end_line: 51usize,
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
            let res = ::alloc::fmt::format(format_args!("{0}", template));
            res
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
#[oxiplate_inline = "Hello @{#_ Comment! _#}!"]
struct CommentPreserveSpaceless {}
impl ::std::fmt::Display for CommentPreserveSpaceless {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("Hello @!")?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "comment_preserve_spaceless"]
#[doc(hidden)]
pub const comment_preserve_spaceless: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("comment_preserve_spaceless"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive\\tests\\whitespace.rs",
        start_line: 63usize,
        start_col: 4usize,
        end_line: 63usize,
        end_col: 30usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(comment_preserve_spaceless()),
    ),
};
fn comment_preserve_spaceless() {
    let template = CommentPreserveSpaceless {};
    match (
        &::alloc::__export::must_use({
            let res = ::alloc::fmt::format(format_args!("{0}", template));
            res
        }),
        &"Hello @!",
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
            &adjusted_whitespace,
            &comment_preserve_spaceless,
            &comment_whitespace_control,
            &writ_preserve_spaceless,
            &writ_whitespace_control,
        ],
    )
}
