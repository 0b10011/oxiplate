use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{%- if let (a) = (a,) -%}
    {{ a }}
{%- endif -%}
"
)]
struct Single {
    a: usize,
}

fn main() {
    print!("{}", Single { a: 19 })
}
