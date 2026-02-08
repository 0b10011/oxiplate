#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ name ~ " (" ~ company ~ ")" }}"#)]
struct User {
    name: &'static str,
    company: &'static str,
}

#[test]
fn variable() {
    let data = User {
        name: "Xavier",
        company: "XYZ Company",
    };

    assert_eq!(format!("{data}"), "Xavier (XYZ Company)");
}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ "hello" ~ " " ~ "world" }}"#)]
struct ConcatStrings;

#[test]
fn concat_strings() {
    assert_eq!(format!("{}", ConcatStrings), "hello world");
}
