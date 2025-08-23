use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_extends = "path/that/does/not/exist"]
struct Data;

fn main() {
    print!("{}", Data);
}