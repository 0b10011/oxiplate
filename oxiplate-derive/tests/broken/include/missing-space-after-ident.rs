use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% include"include.html.oxip" %}"#)]
struct Data;

fn main() {
    print!("{}", Data);
}
