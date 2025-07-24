use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    r"
{%- if check -%}
bar
{%- elseif let Some(text) = ty -%}
{{ text }}
{%- endif -%}
"
)]
struct Data {
    check: bool,
    ty: Option<&'static str>,
}

#[test]
fn test() {
    let data = Data {
        check: false,
        ty: Some("foo"),
    };

    assert_eq!(format!("{data}"), "foo");
}
