use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ Vec::<T: for <const FOO: usize = 19>>::from([19, 89]).join("") }}"#)]
struct Data;

fn main() {
    assert_eq!(format!("{Data}"), "1989");
}
