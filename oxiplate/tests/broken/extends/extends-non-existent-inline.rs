use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline(html: r#"{% extends "extends-non-existent.html.oxip" %}"#)]
struct Data;

fn main() {
    assert_eq!("", Data.render().unwrap());
}
