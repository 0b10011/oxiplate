#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

mod core {}

#[derive(Oxiplate)]
#[oxiplate_inline("{{ foo }}")]
struct Data {
    foo: &'static str,
}

#[test]
fn overridden_std() {
    let data = Data {
        foo: "Hello world!",
    };

    assert_eq!("Hello world!", format!("{data}"));
}
