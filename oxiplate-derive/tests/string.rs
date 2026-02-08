#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r###"{{ ##"jane #"the deer"# doe"## }}"###)]
struct RawString {}

#[test]
fn raw_string() {
    let template = RawString {};

    assert_eq!(format!("{template}"), r###"jane #"the deer"# doe"###);
}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ "" }}"#)]
struct EmptyString {}

#[test]
fn empty_string() {
    let template = EmptyString {};

    assert_eq!(format!("{template}"), "");
}

#[derive(Oxiplate)]
#[oxiplate_inline("\x00 \x0F \x0f \x7F")]
struct SevenBitEscapes;

#[test]
fn seven_bit_escapes() {
    assert_eq!("\0 \u{f} \u{f} \u{7f}", format!("{}", SevenBitEscapes));
}
