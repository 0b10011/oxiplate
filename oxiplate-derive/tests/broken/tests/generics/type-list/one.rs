use oxiplate_derive::Oxiplate;

struct Foo;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ Vec::(Foo)::from([19, 89]).join("") }}"#)]
struct Data;

fn main() {
    assert_eq!(format!("{Data}"), "1989");
}
