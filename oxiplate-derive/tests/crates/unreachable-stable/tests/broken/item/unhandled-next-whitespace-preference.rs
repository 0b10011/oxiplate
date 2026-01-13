use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ "" -}} {{$ "" }}"#)]
struct Data;

fn main() {
    print!("{}", Data);
}
