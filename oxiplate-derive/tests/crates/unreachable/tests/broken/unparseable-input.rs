use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("hello world")]
struct UnreachableUnparseableInput;

fn main() {
    assert_eq!(format!("{}", UnreachableUnparseableInput), "Hello world!");
}
