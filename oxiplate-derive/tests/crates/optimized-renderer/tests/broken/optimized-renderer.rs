use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("hello world")]
struct Data;

fn main() {
    assert_eq!(Data.render().unwrap(), "Hello world!");
}
