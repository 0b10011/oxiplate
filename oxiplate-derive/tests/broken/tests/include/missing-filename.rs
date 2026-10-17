use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% include "" %}"#)]
struct Data;

fn main() {
    print!("{}", Data);
}
