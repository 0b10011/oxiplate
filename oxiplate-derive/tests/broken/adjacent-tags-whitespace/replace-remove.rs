use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ "a" _}} {{- "b" }}"#)]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}
