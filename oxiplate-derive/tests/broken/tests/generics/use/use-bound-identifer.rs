use oxiplate_derive::Oxiplate;

const FOO: usize = 19;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ Into::<str: use <FOO>>::into(value) }}"#)]
struct Data<'a> {
    value: &'a str,
}

fn main() {
    assert_eq!(format!("{}", Data { value: "19" }), "19");
}
