use oxiplate_derive::Oxiplate;

const FOO: usize = 19;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ Vec::<FOO>::from([19, 89]).join("") }}"#)]
struct Data;

fn main() {
    assert_eq!(format!("{Data}"), "1989");
}
