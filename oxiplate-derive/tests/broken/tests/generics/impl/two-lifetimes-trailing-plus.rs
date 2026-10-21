use core::fmt::Display;

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ Vec::<impl 'a + 'b +>::from([19, 89]).join("") }}"#)]
struct Data<'a, 'b>;

fn main() {
    assert_eq!(format!("{Data}"), "1989");
}
