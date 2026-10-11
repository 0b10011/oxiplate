#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ ["foo", "bar"][1] ~ "..." }}"#)]
struct Data;

#[test]
fn index() {
    assert_eq!(format!("{Data}"), "bar...");
}
