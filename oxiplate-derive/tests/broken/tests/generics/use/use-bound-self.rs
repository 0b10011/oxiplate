use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ Into::<str: use <Self>>::into(value) }}"#)]
struct Data<'a> {
    value: &'a str,
}

fn main() {
    assert_eq!(format!("{}", Data { value: "19" }), "19");
}
