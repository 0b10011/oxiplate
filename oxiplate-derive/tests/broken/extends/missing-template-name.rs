use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% extends %}"#)]
struct Data;

fn main() {
    print!("{}", Data);
}
