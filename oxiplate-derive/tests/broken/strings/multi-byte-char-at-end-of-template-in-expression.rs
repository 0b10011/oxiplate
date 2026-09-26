use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% if ❯"#)]
struct Data;

fn main() {
    print!("{}", Data);
}
