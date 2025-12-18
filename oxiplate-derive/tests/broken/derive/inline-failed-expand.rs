use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(include_str!("/path/that/does/not/exist"))]
struct Data;

fn main() {
    print!("{}", Data);
}
