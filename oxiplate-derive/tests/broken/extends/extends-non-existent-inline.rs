use oxiplate_derive::{Oxiplate};

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% extends "extends-non-existent.html.oxip" %}"#)]
struct Data;

fn main() {
    assert_eq!("", format!("{}", Data));
}
