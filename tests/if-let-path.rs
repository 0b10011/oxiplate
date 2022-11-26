use oxiplate::Oxiplate;

enum Type {
    Text(&'static str),
}

#[derive(Oxiplate)]
#[oxiplate_inline = r#"
{%- if let Type::Text(text) = ty -%}
{{ text }}
{%- endif -%}
"#]
struct Data {
    ty: Type,
}

#[test]
fn test() {
    let data = Data {
        ty: Type::Text("foo"),
    };

    assert_eq!(format!("{}", data), "foo");
}
