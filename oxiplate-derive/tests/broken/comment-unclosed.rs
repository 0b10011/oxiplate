use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{#"#)]
struct Data;

fn main() {
    assert_eq!("foo", format!("{}", Data {}));
}
