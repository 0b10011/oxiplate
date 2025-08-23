#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline(
    "
{% if (..3).contains(&-1) %}3 contains -1{% endif %}
{% if (..b).contains(&-1) %}b contains -1{% endif %}
{% if (..3).contains(&3) %}3 contains 3{% endif %}
{% if (..b).contains(&3) %}b contains 3{% endif %}
{% if (..3).contains(&4) %}3 contains 4{% endif %}
{% if (..b).contains(&4) %}b contains 4{% endif %}
"
)]
struct RangeToExclusive {
    b: isize,
}
impl ::std::fmt::Display for RangeToExclusive {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(81usize);
            let f = &mut string;
            f.write_str("\n")?;
            if (..3).contains(&-1) {
                f.write_str("3 contains -1")?;
            }
            f.write_str("\n")?;
            if (..self.b).contains(&-1) {
                f.write_str("b contains -1")?;
            }
            f.write_str("\n")?;
            if (..3).contains(&3) {
                f.write_str("3 contains 3")?;
            }
            f.write_str("\n")?;
            if (..self.b).contains(&3) {
                f.write_str("b contains 3")?;
            }
            f.write_str("\n")?;
            if (..3).contains(&4) {
                f.write_str("3 contains 4")?;
            }
            f.write_str("\n")?;
            if (..self.b).contains(&4) {
                f.write_str("b contains 4")?;
            }
            f.write_str("\n")?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "range_to_exclusive"]
#[doc(hidden)]
pub const range_to_exclusive: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("range_to_exclusive"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/range.rs",
        start_line: 19usize,
        start_col: 4usize,
        end_line: 19usize,
        end_col: 22usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(range_to_exclusive()),
    ),
};
fn range_to_exclusive() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", RangeToExclusive { b: 3 }))
        }),
        &"
3 contains -1
b contains -1




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
#[oxiplate_inline(
    "
{% if (..=3).contains(&-1) %}3 contains -1{% endif %}
{% if (..=b).contains(&-1) %}b contains -1{% endif %}
{% if (..=3).contains(&3) %}3 contains 3{% endif %}
{% if (..=b).contains(&3) %}b contains 3{% endif %}
{% if (..=3).contains(&4) %}3 contains 4{% endif %}
{% if (..=b).contains(&4) %}b contains 4{% endif %}
"
)]
struct RangeToInclusive {
    b: isize,
}
impl ::std::fmt::Display for RangeToInclusive {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(81usize);
            let f = &mut string;
            f.write_str("\n")?;
            if (..=3).contains(&-1) {
                f.write_str("3 contains -1")?;
            }
            f.write_str("\n")?;
            if (..=self.b).contains(&-1) {
                f.write_str("b contains -1")?;
            }
            f.write_str("\n")?;
            if (..=3).contains(&3) {
                f.write_str("3 contains 3")?;
            }
            f.write_str("\n")?;
            if (..=self.b).contains(&3) {
                f.write_str("b contains 3")?;
            }
            f.write_str("\n")?;
            if (..=3).contains(&4) {
                f.write_str("3 contains 4")?;
            }
            f.write_str("\n")?;
            if (..=self.b).contains(&4) {
                f.write_str("b contains 4")?;
            }
            f.write_str("\n")?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "range_to_inclusive"]
#[doc(hidden)]
pub const range_to_inclusive: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("range_to_inclusive"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/range.rs",
        start_line: 49usize,
        start_col: 4usize,
        end_line: 49usize,
        end_col: 22usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(range_to_inclusive()),
    ),
};
fn range_to_inclusive() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", RangeToInclusive { b: 3 }))
        }),
        &"
3 contains -1
b contains -1
3 contains 3
b contains 3


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
#[oxiplate_inline(
    "
{% if (4..).contains(&3) %}4 contains 3{% endif %}
{% if (a..).contains(&3) %}a contains 3{% endif %}
{% if (4..).contains(&4) %}4 contains 4{% endif %}
{% if (a..).contains(&4) %}a contains 4{% endif %}
{% if (4..).contains(&127) %}4 contains 127{% endif %}
{% if (a..).contains(&127) %}a contains 127{% endif %}
"
)]
struct RangeFrom {
    a: i8,
}
impl ::std::fmt::Display for RangeFrom {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(83usize);
            let f = &mut string;
            f.write_str("\n")?;
            if (4..).contains(&3) {
                f.write_str("4 contains 3")?;
            }
            f.write_str("\n")?;
            if (self.a..).contains(&3) {
                f.write_str("a contains 3")?;
            }
            f.write_str("\n")?;
            if (4..).contains(&4) {
                f.write_str("4 contains 4")?;
            }
            f.write_str("\n")?;
            if (self.a..).contains(&4) {
                f.write_str("a contains 4")?;
            }
            f.write_str("\n")?;
            if (4..).contains(&127) {
                f.write_str("4 contains 127")?;
            }
            f.write_str("\n")?;
            if (self.a..).contains(&127) {
                f.write_str("a contains 127")?;
            }
            f.write_str("\n")?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "range_from"]
#[doc(hidden)]
pub const range_from: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("range_from"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/range.rs",
        start_line: 79usize,
        start_col: 4usize,
        end_line: 79usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(range_from()),
    ),
};
fn range_from() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", RangeFrom { a: 4 }))
        }),
        &"


4 contains 4
a contains 4
4 contains 127
a contains 127
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
#[oxiplate_inline(
    "
{% if (3..19).contains(&2) %}3 contains 2{% endif %}
{% if (a..b).contains(&2) %}ab contains 2{% endif %}
{% if (3..19).contains(&4) %}3 contains 3{% endif %}
{% if (a..b).contains(&4) %}ab contains 3{% endif %}
{% if (3..19).contains(&18) %}3 contains 18{% endif %}
{% if (a..b).contains(&18) %}ab contains 18{% endif %}
{% if (3..19).contains(&19) %}3 contains 19{% endif %}
{% if (a..b).contains(&19) %}ab contains 19{% endif %}
{% if (3..19).contains(&20) %}3 contains 20{% endif %}
{% if (a..b).contains(&20) %}ab contains 20{% endif %}
"
)]
struct RangeExclusive {
    a: i8,
    b: i8,
}
impl ::std::fmt::Display for RangeExclusive {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(142usize);
            let f = &mut string;
            f.write_str("\n")?;
            if (3..19).contains(&2) {
                f.write_str("3 contains 2")?;
            }
            f.write_str("\n")?;
            if (self.a..self.b).contains(&2) {
                f.write_str("ab contains 2")?;
            }
            f.write_str("\n")?;
            if (3..19).contains(&4) {
                f.write_str("3 contains 3")?;
            }
            f.write_str("\n")?;
            if (self.a..self.b).contains(&4) {
                f.write_str("ab contains 3")?;
            }
            f.write_str("\n")?;
            if (3..19).contains(&18) {
                f.write_str("3 contains 18")?;
            }
            f.write_str("\n")?;
            if (self.a..self.b).contains(&18) {
                f.write_str("ab contains 18")?;
            }
            f.write_str("\n")?;
            if (3..19).contains(&19) {
                f.write_str("3 contains 19")?;
            }
            f.write_str("\n")?;
            if (self.a..self.b).contains(&19) {
                f.write_str("ab contains 19")?;
            }
            f.write_str("\n")?;
            if (3..19).contains(&20) {
                f.write_str("3 contains 20")?;
            }
            f.write_str("\n")?;
            if (self.a..self.b).contains(&20) {
                f.write_str("ab contains 20")?;
            }
            f.write_str("\n")?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "range_exclusive"]
#[doc(hidden)]
pub const range_exclusive: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("range_exclusive"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/range.rs",
        start_line: 114usize,
        start_col: 4usize,
        end_line: 114usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(range_exclusive()),
    ),
};
fn range_exclusive() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", RangeExclusive { a: 3, b: 19 }))
        }),
        &"


3 contains 3
ab contains 3
3 contains 18
ab contains 18




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
#[oxiplate_inline(
    "
{% if (3..=19).contains(&2) %}3 contains 2{% endif %}
{% if (a..=b).contains(&2) %}ab contains 2{% endif %}
{% if (3..=19).contains(&4) %}3 contains 3{% endif %}
{% if (a..=b).contains(&4) %}ab contains 3{% endif %}
{% if (3..=19).contains(&18) %}3 contains 18{% endif %}
{% if (a..=b).contains(&18) %}ab contains 18{% endif %}
{% if (3..=19).contains(&19) %}3 contains 19{% endif %}
{% if (a..=b).contains(&19) %}ab contains 19{% endif %}
{% if (3..=19).contains(&20) %}3 contains 20{% endif %}
{% if (a..=b).contains(&20) %}ab contains 20{% endif %}
"
)]
struct RangeInclusive {
    a: i8,
    b: i8,
}
impl ::std::fmt::Display for RangeInclusive {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(142usize);
            let f = &mut string;
            f.write_str("\n")?;
            if (3..=19).contains(&2) {
                f.write_str("3 contains 2")?;
            }
            f.write_str("\n")?;
            if (self.a..=self.b).contains(&2) {
                f.write_str("ab contains 2")?;
            }
            f.write_str("\n")?;
            if (3..=19).contains(&4) {
                f.write_str("3 contains 3")?;
            }
            f.write_str("\n")?;
            if (self.a..=self.b).contains(&4) {
                f.write_str("ab contains 3")?;
            }
            f.write_str("\n")?;
            if (3..=19).contains(&18) {
                f.write_str("3 contains 18")?;
            }
            f.write_str("\n")?;
            if (self.a..=self.b).contains(&18) {
                f.write_str("ab contains 18")?;
            }
            f.write_str("\n")?;
            if (3..=19).contains(&19) {
                f.write_str("3 contains 19")?;
            }
            f.write_str("\n")?;
            if (self.a..=self.b).contains(&19) {
                f.write_str("ab contains 19")?;
            }
            f.write_str("\n")?;
            if (3..=19).contains(&20) {
                f.write_str("3 contains 20")?;
            }
            f.write_str("\n")?;
            if (self.a..=self.b).contains(&20) {
                f.write_str("ab contains 20")?;
            }
            f.write_str("\n")?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "range_inclusive"]
#[doc(hidden)]
pub const range_inclusive: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("range_inclusive"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/range.rs",
        start_line: 153usize,
        start_col: 4usize,
        end_line: 153usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(range_inclusive()),
    ),
};
fn range_inclusive() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", RangeInclusive { a: 3, b: 19 }))
        }),
        &"


3 contains 3
ab contains 3
3 contains 18
ab contains 18
3 contains 19
ab contains 19


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
#[oxiplate_inline(
    r#"{-}
{{ a[..] _}}
{{ a[2..] _}}
{{ a[..2] _}}
{{ a[..=2] _}}
{{ a[2..4] _}}
{{ a[2..=4] -}}
"#
)]
struct RangeFull {
    a: &'static str,
}
impl ::std::fmt::Display for RangeFull {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(11usize);
            let f = &mut string;
            f.write_str(&::std::string::ToString::to_string(&(self.a[..])))?;
            f.write_str(" ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.a[2..])))?;
            f.write_str(" ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.a[..2])))?;
            f.write_str(" ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.a[..=2])))?;
            f.write_str(" ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.a[2..4])))?;
            f.write_str(" ")?;
            f.write_str(&::std::string::ToString::to_string(&(self.a[2..=4])))?;
            string
        };
        f.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "range_full"]
#[doc(hidden)]
pub const range_full: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("range_full"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/range.rs",
        start_line: 187usize,
        start_col: 4usize,
        end_line: 187usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(range_full()),
    ),
};
fn range_full() {
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", RangeFull { a: "abcde" }))
        }),
        &"abcde cde ab abc cd cde",
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
            &range_exclusive,
            &range_from,
            &range_full,
            &range_inclusive,
            &range_to_exclusive,
            &range_to_inclusive,
        ],
    )
}
