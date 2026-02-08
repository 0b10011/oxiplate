#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{%- if do_this -%}
    This then {{ action }} :D
{%- endif %}"
)]
struct Data {
    do_this: bool,
    action: &'static str,
}

#[test]
fn test_if() {
    let data = Data {
        do_this: true,
        action: "do something",
    };

    assert_eq!(format!("{data}"), "This then do something :D");
}
