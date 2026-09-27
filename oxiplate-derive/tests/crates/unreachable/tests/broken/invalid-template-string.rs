use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("unreachable string never starts")]
struct Data;

fn main() {
    assert_eq!(format!("{}", Data), "Hello world!");
}
