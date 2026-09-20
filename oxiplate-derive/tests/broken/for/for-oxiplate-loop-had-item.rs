use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- for oxiplate_loop_had_item in numbers -%}
    Each: {{ oxiplate_loop_had_item -}}
{% else -%}
    Else: {{ oxiplate_loop_had_item -}}
{% endfor -%}
"#
)]
struct Data {
    numbers: Vec<i32>,
}

fn main() {
    print!(
        "{}",
        Data {
            numbers: vec![19, 89],
        }
    );
}
