use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "{% if %}"]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}
