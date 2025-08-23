use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(html html html)]
struct Data;

fn main() {
    print!("{}", Data);
}