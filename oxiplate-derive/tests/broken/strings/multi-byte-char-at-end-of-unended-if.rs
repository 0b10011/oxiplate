use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% if true %}❯"#)]
struct Data;

fn main() {
    print!("{}", Data);
}
