use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ Into::<str: use <'a, 'b, 'c>>::into(value) }}"#)]
struct Data<'a, 'b, 'c> {
    value: &'a &'b &'c str,
}

fn main() {
    assert_eq!(format!("{}", Data { value: &&"19" }), "19");
}
