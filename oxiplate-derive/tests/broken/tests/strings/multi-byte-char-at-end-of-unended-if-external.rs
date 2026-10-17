use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "multi-byte-char-at-end-of-unended-if.oxip"]
struct Data;

fn main() {
    print!("{}", Data);
}
