use oxiplate_derive::Oxiplate;

const BAZ: usize = 19;

trait Bar {}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ Vec::<T: ? Bar>::from([19, 89]).join("") }}"#)]
struct Data;

fn main() {
    assert_eq!(format!("{Data}"), "1989");
}
