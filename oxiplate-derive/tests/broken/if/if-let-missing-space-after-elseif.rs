use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- if value > 0 -%}
    Greater than 0.
{%- elseifvalue < 0 -%}
    Less than 0.
{%- endif -%}
"#
)]
struct Data {
    value: usize,
}

fn main() {
    assert_eq!(
        "Greater than 0.",
        format!("{}", Data { value: 19 })
    );
}
