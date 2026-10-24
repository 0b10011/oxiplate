use oxiplate_derive::Oxiplate;

struct Foo;
struct Bar;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ Vec::(Foo, Bar,)::from([19, 89]).join("") }}"#)]
struct Data;

fn main() {
    assert_eq!(format!("{Data}"), "1989");
}
