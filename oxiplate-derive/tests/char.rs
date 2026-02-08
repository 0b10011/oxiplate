#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r"{{ 'a' }}")]
struct A;

#[test]
fn a() {
    assert_eq!(format!("{}", A), "a");
}

#[derive(Oxiplate)]
#[oxiplate_inline(r"{{ '\'' }}")]
struct SingleQuote;

#[test]
fn single_quote() {
    assert_eq!(format!("{}", SingleQuote), "'");
}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ '\"' }}"#)]
struct DoubleQuote;

#[test]
fn double_quote() {
    assert_eq!(format!("{}", DoubleQuote), r#"""#);
}

#[derive(Oxiplate)]
#[oxiplate_inline(r"{{ '\\' }}")]
struct Slash;

#[test]
fn slash() {
    assert_eq!(format!("{}", Slash), r"\");
}

#[derive(Oxiplate)]
#[oxiplate_inline("{{ '\n' }} {{ '\\n' }}")]
struct NewLine;

#[test]
fn new_line() {
    assert_eq!(format!("{}", NewLine), "\n \n");
}

#[derive(Oxiplate)]
#[oxiplate_inline("{{ '\r' }} {{ '\\r' }}")]
struct CarriageReturn;

#[test]
fn carriage_return() {
    assert_eq!(format!("{}", CarriageReturn), "\r \r");
}

#[derive(Oxiplate)]
#[oxiplate_inline("{{ '\t' }} {{ '\\t' }}")]
struct Tab;

#[test]
fn tab() {
    assert_eq!(format!("{}", Tab), "\t \t");
}

#[derive(Oxiplate)]
#[oxiplate_inline(r"{{ '\0' }}")]
struct Null;

#[test]
fn null() {
    assert_eq!(format!("{}", Null), "\0");
}
