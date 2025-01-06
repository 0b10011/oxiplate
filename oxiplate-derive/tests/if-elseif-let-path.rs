use oxiplate_derive::Oxiplate;

enum Type {
    Text(&'static str),
}

#[derive(Oxiplate)]
#[oxiplate_inline = r"
{%- if check -%}
bar
{%- elseif let Type::Text(text) = ty -%}
{{ text }}
{%- endif -%}
"]
struct Data {
    check: bool,
    ty: Type,
}

#[test]
fn test() {
    let data = Data {
        check: false,
        ty: Type::Text("foo"),
    };

    assert_eq!(format!("{data}"), "foo");
}
