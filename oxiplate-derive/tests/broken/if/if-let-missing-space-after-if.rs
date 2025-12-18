use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- ifvalue == true -%}
    Data found.
{%- endif -%}
"#
)]
struct Data {
    value: bool,
}

fn main() {
    assert_eq!("Data found.", format!("{}", Data { value: true }));
}
