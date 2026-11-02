use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"{% extends "extends-wrapper.html.oxip" %}
{% block content -%}
    <p>{{ message }}</p>
{%- endblock aside %}
"#
)]
struct Data {
    title: &'static str,
    message: &'static str,
}

fn main() {
    print!(
        "{}",
        Data {
            title: "Title",
            message: "Message"
        }
    );
}
