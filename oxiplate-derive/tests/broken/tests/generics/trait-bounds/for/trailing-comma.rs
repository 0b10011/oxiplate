use oxiplate_derive::Oxiplate;

const BAZ: usize = 19;

trait Bar<'a> {}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ Vec::<T: for <'a,> Bar<'a>>::from([19, 89]).join("") }}"#)]
struct Data;

fn main() {
    assert_eq!(format!("{Data}"), "1989");
}
