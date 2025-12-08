use oxiplate_derive::Oxiplate;

struct Value(usize);

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- if let Value(a) -%}
    {{ a }}
{%- endif -%}
"#
)]
struct ValueWrapper {
    value: Value,
}

fn main() {
    assert_eq!(
        "19",
        format!("{}", ValueWrapper { value: Value(19) })
    );
}
