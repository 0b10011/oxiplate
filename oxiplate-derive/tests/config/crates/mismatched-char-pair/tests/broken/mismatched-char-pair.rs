use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("hello world!")]
struct Data;

fn main() {
    assert_eq!(format!("{}", Data), "hello world!");
}
