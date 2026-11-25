use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- default_escaper_group html -%}
{%- filter raw | lower -%}
    Hello World!
    {%- block foo %}{% endblock %}
{%- endfilter -%}
"#
)]
struct Data;

fn main() {
    print!("{Data}");
}
