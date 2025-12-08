use oxiplate_derive::Oxiplate;

struct Value {
    value: &'static str,
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- if let Value { % } -%}
    No value provided.
{%- endif -%}
"#
)]
struct Data {
    value: Value,
}

fn main() {
    assert_eq!(
        "10 + 9 = 19",
        format!("{}", Data { value: Value { value: "10 + 9 = 19" } })
    );
}
