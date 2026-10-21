use oxiplate_derive::Oxiplate;

trait Bar {}

struct Foo;

struct Baz;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ Vec::<T: for <Foo = Baz> Bar>::from([19, 89]).join("") }}"#)]
struct Data;

fn main() {
    assert_eq!(format!("{Data}"), "1989");
}
