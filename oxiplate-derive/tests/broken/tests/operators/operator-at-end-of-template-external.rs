use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "operator-at-end-of-template.oxip"]
struct Data;

fn main() {
    print!("{}", Data);
}
