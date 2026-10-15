use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "hello"]
struct Data;

fn main() {
    print!("{}", Data);
}
