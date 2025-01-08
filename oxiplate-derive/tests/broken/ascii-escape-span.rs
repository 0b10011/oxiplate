use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "hello\n{{"]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}
