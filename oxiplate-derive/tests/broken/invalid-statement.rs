use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "{% thisdoesntexist %}"]
struct Data {}

fn main() {
    print!("{}", Data {});
}
