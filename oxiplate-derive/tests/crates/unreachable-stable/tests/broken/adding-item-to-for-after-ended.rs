use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% for _ in 0..2 %}{% endfor %}{{ "b" }}"#)]
struct Data;

fn main() {
    print!("{}", Data);
}
