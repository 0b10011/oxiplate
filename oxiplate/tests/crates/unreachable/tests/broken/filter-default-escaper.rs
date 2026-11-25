use oxiplate::prelude::*;

fn main() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(
        r#"{-}
{% default_escaper_group html -%}
{% filter default | lower -%}
    <hi>
{%- endfilter -%}
"#
    )]
    struct Data;

    assert_eq!(Data.render().unwrap(), "&lt;hi>");
}
