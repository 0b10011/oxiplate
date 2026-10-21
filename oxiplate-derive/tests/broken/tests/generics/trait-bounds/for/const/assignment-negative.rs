use oxiplate_derive::Oxiplate;

trait Bar {}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ Vec::<T: for <const FOO: usize = -19> Bar>::from([19, 89]).join("") }}"#)]
struct Data;

fn main() {
    assert_eq!(format!("{Data}"), "1989");
}
