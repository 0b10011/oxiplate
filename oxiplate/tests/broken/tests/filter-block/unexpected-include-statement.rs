use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- default_escaper_group html -%}
{%- filter raw | lower -%}
    Hello World!
    {%- include "page.html.oxip" -%}
{%- endfilter -%}
"#
)]
struct Data;

fn main() {
    print!("{Data}");
}
