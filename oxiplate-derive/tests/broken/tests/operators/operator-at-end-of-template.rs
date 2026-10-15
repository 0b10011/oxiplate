use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% if 19 +"#)]
struct Data;

fn main() {
    print!("{}", Data);
}
