use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- if let Some(value) = value -%}
    Found
{%- else -%}
    Missing
{%- case None -%}
    Other
{%- endif -%}
"#
)]
struct Data {
    value: Option<&'static str>,
}

fn main() {
    assert_eq!("Data found.", format!("{}", Data { value: None }));
}
