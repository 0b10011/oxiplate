#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use oxiplate::Oxiplate;
#[oxiplate_inline = "{-}
{{ max }} + {{ min }} = {{ max + min }}
{{ max }} - {{ min }} = {{ max - min }}
{{ max }} * {{ min }} = {{ max * min }}
{{ max }} / {{ min }} = {{ max / min }}
{{ max }} % {{ min }} = {{ max % min }}"]
struct Math {
    min: i16,
    max: i16,
}
impl std::fmt::Display for Math {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "+"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.min));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max +
                self.min)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "-"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.min));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max -
                self.min)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "*"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.min));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max *
                self.min)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "/"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.min));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max /
                self.min)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "%"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.min));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max %
                self.min)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_math"]
pub const test_math: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_math"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\calc.rs",
        start_line: 16usize,
        start_col: 4usize,
        end_line: 16usize,
        end_col: 13usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(test_math())),
};
fn test_math() {
    let data = Math { min: 19, max: 89 };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        },
        &"89 + 19 = 108
89 - 19 = 70
89 * 19 = 1691
89 / 19 = 4
89 % 19 = 13",
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
#[oxiplate_inline = "{-}
{{ max }} == {{ min }} = {{ max == min }}
{{ max }} != {{ min }} = {{ max != min }}
{{ max }} > {{ min }} = {{ max > min }}
{{ max }} < {{ min }} = {{ max < min }}
{{ max }} >= {{ min }} = {{ max >= min }}
{{ max }} <= {{ min }} = {{ max <= min }}"]
struct Comparisons {
    min: i16,
    max: i16,
}
impl std::fmt::Display for Comparisons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "=="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.min));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max ==
                self.min)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "!="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.min));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max !=
                self.min)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", ">"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.min));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max >
                self.min)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "<"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.min));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max <
                self.min)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", ">="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.min));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max >=
                self.min)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "<="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.min));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.max <=
                self.min)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_comparisons"]
pub const test_comparisons: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_comparisons"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\calc.rs",
        start_line: 43usize,
        start_col: 4usize,
        end_line: 43usize,
        end_col: 20usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(test_comparisons())),
};
fn test_comparisons() {
    let data = Comparisons { min: 19, max: 89 };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        },
        &"89 == 19 = false
89 != 19 = true
89 > 19 = true
89 < 19 = false
89 >= 19 = true
89 <= 19 = false",
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
#[oxiplate_inline = "{-}
{{ yes }} || {{ yes }} = {{ yes || yes2 }}
{{ yes }} || {{ no }} = {{ yes || no }}
{{ no }} || {{ yes }} = {{ no || yes }}
{{ no }} || {{ no }} = {{ no || no2 }}
{{ yes }} && {{ yes }} = {{ yes && yes2 }}
{{ yes }} && {{ no }} = {{ yes && no }}
{{ no }} && {{ yes }} = {{ no && yes }}
{{ no }} && {{ no }} = {{ no && no2 }}"]
struct OrAnd {
    yes: bool,
    yes2: bool,
    no: bool,
    no2: bool,
}
impl std::fmt::Display for OrAnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.yes));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "||"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.yes));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.yes ||
                self.yes2)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.yes));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "||"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.no));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.yes ||
                self.no)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.no));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "||"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.yes));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.no ||
                self.yes)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.no));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "||"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.no));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.no ||
                self.no2)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.yes));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "&&"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.yes));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.yes &&
                self.yes2)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.yes));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "&&"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.no));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.yes &&
                self.no)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.no));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "&&"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.yes));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.no &&
                self.yes)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", "\n"))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.no));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "&&"))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.no));
                res } .chars().map(| character | match character { '&' => { let res =
                ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let res =
                ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(format_args!("{0}", "="))?;
        f.write_fmt(format_args!("{0}", " "))?;
        f.write_fmt(
            format_args!(
                "{0}", { let res = ::alloc::fmt::format(format_args!("{0}", self.no &&
                self.no2)); res } .chars().map(| character | match character { '&' => {
                let res = ::alloc::fmt::format(format_args!("&amp;")); res } '<' => { let
                res = ::alloc::fmt::format(format_args!("&lt;")); res } _ => { let res =
                ::alloc::fmt::format(format_args!("{0}", character)); res } }).collect::<
                String > ()
            ),
        )?;
        Ok(())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_or_and"]
pub const test_or_and: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_or_and"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests\\calc.rs",
        start_line: 75usize,
        start_col: 4usize,
        end_line: 75usize,
        end_col: 15usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(test_or_and())),
};
fn test_or_and() {
    let data = OrAnd {
        yes: true,
        yes2: true,
        no: false,
        no2: false,
    };
    match (
        &{
            let res = ::alloc::fmt::format(format_args!("{0}", data));
            res
        },
        &"true || true = true
true || false = true
false || true = true
false || false = false
true && true = true
true && false = false
false && true = false
false && false = false",
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
    test::test_main_static(&[&test_comparisons, &test_math, &test_or_and])
}
