use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = r"{% if value %}foo{% endif _%}
"]
struct Data {
    value: bool,
}

#[test]
fn adjusted_whitespace() {
    let template = Data { value: true };

    assert_eq!(format!("{template}"), "foo ");
}
