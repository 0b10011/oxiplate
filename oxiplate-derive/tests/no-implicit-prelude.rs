#![no_implicit_prelude]
#![no_std]

extern crate alloc;

use alloc::format;

use ::core::assert_eq;
use ::oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "extends.html.oxip"]
struct Data {
    title: &'static str,
    message: &'static str,
}

#[test]
fn no_implicit_prelude() {
    assert_eq!(
        format!(
            "{}",
            Data {
                title: "Oxiplate Example",
                message: "Hello world!",
            }
        ),
        "<!DOCTYPE html>\n<title>Oxiplate Example</title>\n<h1>Oxiplate Example</h1>\n  <p>Hello \
         world!</p>\n"
    );
}
