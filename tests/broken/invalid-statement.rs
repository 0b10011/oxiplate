use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "{% thisdoesntexist %}"]
struct Data {}

fn main() {
    print!("{}", Data {});
}
