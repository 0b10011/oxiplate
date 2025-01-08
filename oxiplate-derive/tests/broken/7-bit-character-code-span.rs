use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "hello\x41{{"]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}
