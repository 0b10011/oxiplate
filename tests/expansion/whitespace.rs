#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::Oxiplate;
#[oxiplate_inline = "Hello  \t\n {_} \r\n\t wo{_}r{-}ld \n\t {-} \t\n !"]
struct AdjustedWhitespace {}
impl std::fmt::Display for AdjustedWhitespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0}", "Hello"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "wo"))?;
        f.write_fmt(format_args!("{0}", "r"))?;
        f.write_fmt(format_args!("{0}", "ld"))?;
        f.write_fmt(format_args!("{0}", "!"))?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "adjusted_whitespace"]
pub const adjusted_whitespace: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("adjusted_whitespace"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\whitespace.rs",
        start_line: 8usize,
        start_col: 4usize,
        end_line: 8usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(adjusted_whitespace())),
};
fn adjusted_whitespace() {
    let template = AdjustedWhitespace {};
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", template));
            res
        },
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
#[oxiplate_inline = "Hello  \t\t  \r\n\t {{_ username _}}  \t\t  \r\n\t (  \t\t  \r\n\t {{- name -}}  \t\t  \r\n\t )!"]
struct WritWhitespaceControl {
    username: &'static str,
    name: &'static str,
}
impl std::fmt::Display for WritWhitespaceControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0}", "Hello"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self
                .username)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "("))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.name));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", ")!"))?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "writ_whitespace_control"]
pub const writ_whitespace_control: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("writ_whitespace_control"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\whitespace.rs",
        start_line: 22usize,
        start_col: 4usize,
        end_line: 22usize,
        end_col: 27usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(writ_whitespace_control())),
};
fn writ_whitespace_control() {
    let template = WritWhitespaceControl {
        username: "dia",
        name: "Diana",
    };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", template));
            res
        },
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
impl std::fmt::Display for WritPreserveSpaceless {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0}", "Hello @"))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self
                .username)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "!"))?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "writ_preserve_spaceless"]
pub const writ_preserve_spaceless: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("writ_preserve_spaceless"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\whitespace.rs",
        start_line: 38usize,
        start_col: 4usize,
        end_line: 38usize,
        end_col: 27usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(writ_preserve_spaceless())),
};
fn writ_preserve_spaceless() {
    let template = WritPreserveSpaceless {
        username: "dia",
    };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", template));
            res
        },
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
#[oxiplate_inline = "Hello  \t\t  \r\n\t {#_ Some cool comment _#}  \t\t  \r\n\t (  \t\t  \r\n\t {#- Hey another comment -#}  \t\t  \r\n\t )!"]
struct CommentWhitespaceControl {}
impl std::fmt::Display for CommentWhitespaceControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0}", "Hello"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "("))?;
        f.write_fmt(format_args!("{0}", ")!"))?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "comment_whitespace_control"]
pub const comment_whitespace_control: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("comment_whitespace_control"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\whitespace.rs",
        start_line: 49usize,
        start_col: 4usize,
        end_line: 49usize,
        end_col: 30usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(comment_whitespace_control())),
};
fn comment_whitespace_control() {
    let template = CommentWhitespaceControl {};
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", template));
            res
        },
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
impl std::fmt::Display for CommentPreserveSpaceless {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0}", "Hello @"))?;
        f.write_fmt(format_args!("{0}", "!"))?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "comment_preserve_spaceless"]
pub const comment_preserve_spaceless: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("comment_preserve_spaceless"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\whitespace.rs",
        start_line: 61usize,
        start_col: 4usize,
        end_line: 61usize,
        end_col: 30usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(comment_preserve_spaceless())),
};
fn comment_preserve_spaceless() {
    let template = CommentPreserveSpaceless {};
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", template));
            res
        },
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
#[no_coverage]
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
