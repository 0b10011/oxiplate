use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- if let Data { value: :( } = data -%}
    Data found.
{%- endif -%}
"#
)]
struct Data {
    value: Option<&'static str>,
}

fn main() {
    assert_eq!("Data found.", format!("{}", Data { value: None }));
}
