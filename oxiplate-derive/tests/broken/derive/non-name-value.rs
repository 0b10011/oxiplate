use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate("foo")]
struct Data;

fn main() {
    print!("{}", Data);
}