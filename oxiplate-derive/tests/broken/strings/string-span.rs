use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ "foo".repeat("bar") }}"#)]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}
