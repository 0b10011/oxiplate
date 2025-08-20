use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"hello{% extends "multiple-blocks.html.oxip" %}"#)]
struct Static;

fn main() {
    print!("{}", Static);
}
