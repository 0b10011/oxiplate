use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "{% thisdoesntexist %}"]
struct Data {}

fn main() {
    print!("{}", Data {});
}
