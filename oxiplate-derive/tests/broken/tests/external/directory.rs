use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "subdirectory"]
struct Data;

fn main() {
    print!("{}", Data);
}
