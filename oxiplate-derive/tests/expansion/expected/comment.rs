use alloc::format;

use oxiplate_derive::Oxiplate;
#[oxiplate_inline("{# Hashes (#) are allowed anywhere in a comment #}")]
struct Hashes;
impl ::core::fmt::Display for Hashes {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "comment::hashes"]
#[doc(hidden)]
pub static hashes: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("comment::hashes"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/comment.rs",
        start_line: 10usize,
        start_col: 4usize,
        end_line: 10usize,
        end_col: 10usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(hashes()),
    ),
};
fn hashes() {
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", Hashes)) }),
            &"",
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
    "{# Other close tokens (`%}`, `-%}`, `_%}`, `}}`, `-}}`, and `_}}`) should not affect parsing \
     #}"
)]
struct TagEnds;
impl ::core::fmt::Display for TagEnds {
    fn fmt(&self, oxiplate_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        extern crate alloc;
        use ::core::fmt::Write as _;
        ::core::result::Result::Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "comment::tag_ends"]
#[doc(hidden)]
pub static tag_ends: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("comment::tag_ends"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/integration/comment.rs",
        start_line: 22usize,
        start_col: 4usize,
        end_line: 22usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(tag_ends()),
    ),
};
fn tag_ends() {
    {
        match (
            &::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0}", TagEnds)) }),
            &"",
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
