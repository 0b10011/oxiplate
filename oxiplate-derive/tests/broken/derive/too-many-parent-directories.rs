use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "../../../../../../../../../../../../../../../../../../README.md"]
struct Data;

fn main() {
    print!("{}", Data);
}