use oxiplate_derive::Oxiplate;

struct Bar(usize);

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ Vec::<self::<Bar>::Bar>::from([19, 89]).join("") }}"#)]
struct Data;

fn main() {
    assert_eq!(format!("{Data}"), "1989");
}
