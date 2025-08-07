use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    r"{% if value %}foo{% endif _%}
"
)]
struct Data {
    value: bool,
}

#[test]
fn adjusted_whitespace() {
    let data = Data { value: true };

    assert_eq!(format!("{data}"), "foo ");
}
