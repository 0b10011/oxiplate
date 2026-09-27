use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"unreachable: mocking parsing of `r#`"#)]
struct Data;

fn main() {
    assert_eq!(format!("{}", Data), "Hello world!");
}
