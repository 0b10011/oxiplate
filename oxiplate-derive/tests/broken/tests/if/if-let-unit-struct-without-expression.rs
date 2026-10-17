use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- if let None -%}
    No value provided.
{%- endif -%}
"#
)]
struct Data {
    value: Option<&'static str>,
}

fn main() {
    assert_eq!("10 + 9 = 19", format!("{}", Data { value: None }));
}
