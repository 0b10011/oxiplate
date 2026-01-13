use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% if true %}{% endif %}{{ "b" }}"#)]
struct Data;

fn main() {
    print!("{}", Data);
}
