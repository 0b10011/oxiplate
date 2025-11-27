use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% extends "multiple-blocks.html.oxip" %}{{ "hello" }}"#)]
struct Static;

fn main() {
    print!("{}", Static);
}
