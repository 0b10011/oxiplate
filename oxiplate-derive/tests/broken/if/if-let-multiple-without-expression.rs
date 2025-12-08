use oxiplate_derive::Oxiplate;

struct Couple(usize, usize);

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- if let Couple(a, b) -%}
    {{ a }} + {{ b }} = {{ a + b }}
{%- endif -%}
"#
)]
struct CoupleWrapper {
    couple: Couple,
}

fn main() {
    assert_eq!(
        "10 + 9 = 19",
        format!("{}", CoupleWrapper { couple: Couple(10, 9) })
    );
}
