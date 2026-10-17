use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ >"foo" }}"#)]
struct Data;

fn main() {
    assert_eq!("foo", format!("{}", Data {}));
}
