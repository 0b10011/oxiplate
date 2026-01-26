#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use oxiplate_derive::Oxiplate;
#[oxiplate_inline(
    "
dec: {{ 0 }} {{ 000 }} {{ 19 }} {{ 10 + 9 }} {{ 1_234_567_890 }}
float: {{ 19. }} {{ 19.0 }} {{ 1.9e10 }} {{ 190.0e-1 }} {{ 1_234_567.8e-9 }} {{ 1_234_567_890e0 }}
bin: {{ 0b0 }} {{ 0b0_0000 }} {{ 0b1_0011 }} {{ 0b_1010 + 9 }} {{ 0b01 }}
hex: {{ 0x0 }} {{ 0x0_00 }} {{ 0x13 }} {{ 0x_a + 0x9 }} {{ 0x_23_45_67_89 }} {{ 0x_01_ab_cd_ef }}
oct: {{ 0o0 }} {{ 0o0_00 }} {{ 0o23 }} {{ 0o12 + 0o11 }} {{ 0o01_234_567 }}

dec: {{ -0 }} {{ -000 }} {{ -19 }} {{ -10 + -9 }} {{ -1_234_567_890 }}
float: {{ -19. }} {{ -19.0 }} {{ -1.9e1 }} {{ -190.0e-1 }} {{ -1_234_567.8e-9 }} {{ \
     -1_234_567_890e0 }}
bin: {{ -0b0 }} {{ -0b0_0000 }} {{ -0b1_0011 }} {{ -0b_1010 + -9 }} {{ -0b01 }}
hex: {{ -0x0 }} {{ -0x0_00 }} {{ -0x13 }} {{ -0x_a + -0x9 }} {{ -0x_23_45_67_89 }} {{ \
     -0x_01_ab_cd_ef }}
oct: {{ -0o0 }} {{ -0o0_00 }} {{ -0o23 }} {{ -0o12 + -0o11 }} {{ -0o01_234_567 }}"
)]
struct Data;
impl ::std::fmt::Display for Data {
    fn fmt(
        &self,
        oxiplate_formatter: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::fmt::Result {
        let string = {
            use ::std::fmt::Write;
            let mut string = String::with_capacity(442usize);
            let oxiplate_formatter = &mut string;
            oxiplate_formatter.write_str("\ndec: ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(0)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(000)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(19)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(10 + 9)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(1_234_567_890)))?;
            oxiplate_formatter.write_str("\nfloat: ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(19.)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(19.0)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(1.9e10)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(190.0e-1)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(1_234_567.8e-9)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(1_234_567_890e0)))?;
            oxiplate_formatter.write_str("\nbin: ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(0b0)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(0b0_0000)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(0b1_0011)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(0b_1010 + 9)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(0b01)))?;
            oxiplate_formatter.write_str("\nhex: ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(0x0)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(0x0_00)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(0x13)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(0x_a + 0x9)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(0x_23_45_67_89)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(0x_01_ab_cd_ef)))?;
            oxiplate_formatter.write_str("\noct: ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(0o0)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(0o0_00)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(0o23)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(0o12 + 0o11)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(0o01_234_567)))?;
            oxiplate_formatter.write_str("\n\ndec: ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(-0)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(-000)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(-19)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(-10 + -9)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(-1_234_567_890)))?;
            oxiplate_formatter.write_str("\nfloat: ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(-19.)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(-19.0)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(-1.9e1)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(-190.0e-1)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(-1_234_567.8e-9)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(-1_234_567_890e0)))?;
            oxiplate_formatter.write_str("\nbin: ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(-0b0)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(-0b0_0000)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(-0b1_0011)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(-0b_1010 + -9)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(-0b01)))?;
            oxiplate_formatter.write_str("\nhex: ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(-0x0)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(-0x0_00)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(-0x13)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(-0x_a + -0x9)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(-0x_23_45_67_89)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(-0x_01_ab_cd_ef)))?;
            oxiplate_formatter.write_str("\noct: ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(-0o0)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(-0o0_00)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter.write_str(&::std::string::ToString::to_string(&(-0o23)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(-0o12 + -0o11)))?;
            oxiplate_formatter.write_str(" ")?;
            oxiplate_formatter
                .write_str(&::std::string::ToString::to_string(&(-0o01_234_567)))?;
            string
        };
        oxiplate_formatter.write_str(&string)
    }
}
extern crate test;
#[rustc_test_marker = "field"]
#[doc(hidden)]
pub const field: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("field"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "oxiplate-derive/tests/number.rs",
        start_line: 23usize,
        start_col: 4usize,
        end_line: 23usize,
        end_col: 9usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(field())),
};
fn field() {
    let data = Data;
    match (
        &::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}", data))
        }),
        &"
dec: 0 0 19 19 1234567890
float: 19 19 19000000000 19 0.0012345678 1234567890
bin: 0 0 19 19 1
hex: 0 0 19 19 591751049 28036591
oct: 0 0 19 19 342391

dec: 0 0 -19 -19 -1234567890
float: -19 -19 -19 -19 -0.0012345678 -1234567890
bin: 0 0 -19 -19 -1
hex: 0 0 -19 -19 -591751049 -28036591
oct: 0 0 -19 -19 -342391",
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
    test::test_main_static(&[&field])
}
