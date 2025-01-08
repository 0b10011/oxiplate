use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "hello\u{7FFF}{{"]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}
