use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- default_escaper_group html -%}
{%- filter raww | lower -%}
    Hello World!
{%- endfilter -%}
"#
)]
struct Data;

fn main() {
    print!("{Data}");
}
