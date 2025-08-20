use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% if true %}{% endif %}{% extends "multiple-blocks.html.oxip" %}"#)]
struct Static;

fn main() {
    print!("{}", Static);
}
