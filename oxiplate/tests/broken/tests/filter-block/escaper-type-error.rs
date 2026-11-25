use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- filter text | lower -%}
    Hello World!
{%- endfilter -%}
"#
)]
struct Data;

fn main() {
    print!("{Data}");
}
