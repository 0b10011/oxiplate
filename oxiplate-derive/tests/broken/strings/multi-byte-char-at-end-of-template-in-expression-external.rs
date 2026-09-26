use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "multi-byte-char-at-end-of-template-in-expression.oxip"]
struct Data;

fn main() {
    print!("{}", Data);
}
