use oxiplate_derive::Oxiplate;

struct Value {
    value: &'static str,
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- if let std::option:: -%}
    No value provided.
{%- endif -%}
"#
)]
struct Data {
    value: Option<&'static str>,
}

fn main() {
    assert_eq!(
        "No value provided.",
        format!("{}", Data { value: None })
    );
}
