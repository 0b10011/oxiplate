#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{%- if do_this -%}
    This then {{ action }} :D
{%- elseif do_that -%}
    That then {{ action }} :D
{%- endif %}"
)]
struct Data {
    do_this: bool,
    do_that: bool,
    action: &'static str,
}

#[test]
fn test_if() {
    let data = Data {
        do_this: true,
        do_that: true,
        action: "do something",
    };

    assert_eq!(format!("{data}"), "This then do something :D");
}

#[test]
fn test_else_if() {
    let data = Data {
        do_this: false,
        do_that: true,
        action: "do something",
    };

    assert_eq!(format!("{data}"), "That then do something :D");
}
