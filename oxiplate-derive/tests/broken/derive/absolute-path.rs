use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "/absolute"]
struct Data;

fn main() {
    print!("{}", Data);
}